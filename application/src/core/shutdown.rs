use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use futures::FutureExt;
use lazy_static::lazy_static;
use tokio::signal;
use tokio::sync::{mpsc, Mutex, oneshot, RwLock};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::{Instrument, Span};

lazy_static! {
    static ref SHUTDOWN_HOOK: RwLock<Arc<ShutdownHook>> = RwLock::new(Arc::new(ShutdownHook::new()));
}

type Hook = Pin<Box<dyn Future<Output=()> + Send>>;

pub struct ShutdownHook {
    sync_handlers: UnboundedSender<Hook>,
    sync_handlers_cnt: Arc<AtomicUsize>,
    async_handlers: UnboundedSender<Hook>,
    async_handlers_cnt: Arc<AtomicUsize>,
    stop_rx: Arc<Mutex<Option<Receiver<()>>>>,
}

impl ShutdownHook {
    pub fn new() -> Self {
        let (sync_handlers, sync_rx) = mpsc::unbounded_channel::<Hook>();
        let (async_handlers, async_rx) = mpsc::unbounded_channel::<Hook>();
        let sync_handlers_cnt = Arc::new(AtomicUsize::new(0));
        let async_handlers_cnt = Arc::new(AtomicUsize::new(0));
        let (stop_tx, stop_rx) = oneshot::channel();
        Self::add_shutdown_hook(stop_tx, sync_rx, async_rx, sync_handlers_cnt.clone(), async_handlers_cnt.clone());
        Self {
            sync_handlers,
            sync_handlers_cnt,
            async_handlers,
            async_handlers_cnt,
            stop_rx: Arc::new(Some(stop_rx).into()),
        }
    }

    fn add_shutdown_hook(shutdown_tx: oneshot::Sender<()>, mut sync_rx: UnboundedReceiver<Hook>, mut async_rx: UnboundedReceiver<Hook>, sync_handlers_cnt: Arc<AtomicUsize>, async_handlers_cnt: Arc<AtomicUsize>) {
        let span = Span::current().clone();
        tokio::spawn(
            async move {
                #[cfg(unix)]
                let mut sig = signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler");

                #[cfg(windows)]
                let mut sig = signal::windows::ctrl_c().expect("failed to install signal handler");

                tokio::select! {
                    _ = sig.recv() => {},
                }

                let ins = Instant::now();
                tracing::info!("Terminating process due to signal SIGINT");
                let count = sync_handlers_cnt.load(Ordering::SeqCst);
                let mut hooks = Vec::new();
                async_rx.recv_many(&mut hooks, count).await;
                for h in hooks {
                    let _ = tokio::spawn(h).await;
                }

                let count = sync_handlers_cnt.load(Ordering::SeqCst);
                let mut hooks = Vec::new();
                sync_rx.recv_many(&mut hooks, count).await;
                let handles: Vec<_> = hooks.drain(..).map(tokio::spawn).collect();
                for h in handles {
                    let _ = h.await;
                }

                let _ = shutdown_tx.send(());
                tracing::info!("Application shut down completed({:?})", ins.elapsed());
            }
                .instrument(span),
        );
    }
}


/// 等待完成
pub async fn completed() {
    let hook = SHUTDOWN_HOOK.read().await.clone();
    let mut stop = hook.stop_rx.lock().await;
    if let Some(rx) = stop.take() {
        let _ = rx.await;
    }
}

/// 添加处理器
pub async fn push_sync<F>(future: F)
where
    F: Future<Output=()> + Send + 'static,
{
    let hook = SHUTDOWN_HOOK.read().await.clone();
    if hook.sync_handlers.send(future.boxed()).is_ok() {
        hook.sync_handlers_cnt.fetch_add(1, Ordering::SeqCst);
    }
}

/// 添加处理器
pub async fn push<F>(future: F)
where
    F: Future<Output=()> + Send + 'static,
{
    let hook = SHUTDOWN_HOOK.read().await.clone();
    if hook.async_handlers.send(future.boxed()).is_ok() {
        hook.async_handlers_cnt.fetch_add(1, Ordering::SeqCst);
    }
}