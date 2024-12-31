use crate::core::errors::AppResult;
use nu_ansi_term::Style;
use sea_orm::sqlx::types::chrono;
use serde::Deserialize;
use smallvec::SmallVec;
use std::fmt;
use std::fmt::Write;
use std::path::Path;
use std::str::FromStr;
use std::thread::ThreadId;
use tracing::level_filters::LevelFilter;
use tracing::{Event, Level, Metadata, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::{LookupSpan, Scope};
use tracing_subscriber::util::SubscriberInitExt;

/// 日志配置文件
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Logging {
    /// 日志级别
    #[serde(default = "default_log_level")]
    pub level: String,
    /// 多模块日志等级配置，eg:my_crate::module=trace
    #[serde(default)]
    pub level_list: Option<Vec<String>>,
    /// 是否输出到文件
    #[serde(default)]
    pub enable_file: bool,
    /// 日志文件路径
    #[serde(default)]
    pub path: Option<String>,
}

pub fn default_log_level() -> String {
    "info".to_owned()
}

pub fn setup_logging(config: &Logging) -> AppResult<WorkerGuard> {
    let file_non_blocking = config.enable_file.then_some(config.path.as_ref()).flatten().and_then(|path| {
        let file_path = Path::new(path);
        file_path.parent().map(|folder_path| {
            let file_name: &Path = match (file_path.file_name(), file_path.extension()) {
                (Some(name), Some(_)) => name.as_ref(),
                _ => "application".as_ref(),
            };
            let file_appender = tracing_appender::rolling::daily(folder_path, file_name);
            tracing_appender::non_blocking(file_appender)
        })
    });
    let (non_blocking, guard) = file_non_blocking.unwrap_or_else(|| tracing_appender::non_blocking(std::io::stdout()));

    let mut format = TracingFormat::new(LevelFilter::from_str(&config.level.to_lowercase())?);
    if let Some(list) = &config.level_list {
        for x in list {
            let t: Vec<&str> = x.split('=').collect();
            format.add_target(t[0], LevelFilter::from_str(&t[1].to_lowercase())?);
        }
    }

    let fmt = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking)
        .event_format(format);
    tracing_subscriber::registry().with(fmt).init();

    // 这个 guard 的作用是确保 NonBlocking 在程序运行期间保持活动状态
    Ok(guard)
}

pub(crate) struct TracingFormat {
    directives: SmallVec<[StaticDirective; 8]>,
    default_level: LevelFilter,
}

impl TracingFormat {
    pub fn new(default_level: LevelFilter) -> Self {
        Self {
            directives: SmallVec::new(),
            default_level,
        }
    }

    pub fn add_target(&mut self, target: impl Into<String>, level: impl Into<LevelFilter>) {
        self.directives.push(StaticDirective::new(target.into(), level.into()));
    }

    pub fn disable(&self, meta: &Metadata) -> bool {
        let level = meta.level();
        let target = meta.target();
        match self.directives.iter().find(|d| d.target.starts_with(target)) {
            None => self.default_level < *level,
            Some(d) => d.level < *level,
        }
    }
}

impl<S, N> FormatEvent<S, N> for TracingFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(&self, ctx: &FmtContext<'_, S, N>, mut writer: Writer<'_>, event: &Event<'_>) -> fmt::Result {
        let meta = event.metadata();

        if self.disable(meta) {
            return Ok(());
        }

        let t = chrono::Local::now();
        write!(writer, "{}", t.format("%Y-%m-%d %H:%M:%S%.3f"))?;
        writer.write_char(' ')?;

        write!(writer, "{} ", FmtLevel::new(meta.level()))?;

        let current_thread = std::thread::current();
        if let Some(name) = current_thread.name() {
            write!(writer, "{} ", FmtThreadName::new(name, current_thread.id()))?;
        }

        write!(writer, "{}", FmtCtx::new(ctx, event.parent()))?;

        let dimmed = Style::new();
        for span in ctx.event_scope().into_iter().flat_map(Scope::from_root) {
            let exts = span.extensions();
            if let Some(fields) = exts.get::<FormattedFields<N>>() {
                if !fields.is_empty() {
                    write!(writer, "{} ", dimmed.paint(&fields.fields))?;
                }
            }
        }
        ctx.format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

struct FmtThreadName<'a> {
    name: &'a str,
    id: ThreadId,
}

impl<'a> FmtThreadName<'a> {
    pub fn new(name: &'a str, id: ThreadId) -> Self {
        Self { name, id }
    }
}

impl<'a> fmt::Display for FmtThreadName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = if self.name.len() <= 6 {
            self.name
        } else if self.name.starts_with("tokio-runtime") {
            "worker"
        } else if self.name.starts_with("actix-rt") {
            "engine"
        } else {
            &self.name[0..self.name.len().min(6)]
        };
        let id = format!("{:?}", self.id);
        let id_ref = &id[9..id.len() - 1];

        write!(f, "{:>width$}-{:0>2}", name, id_ref, width = 6)
    }
}

struct FmtLevel<'a> {
    level: &'a Level,
}

impl<'a> FmtLevel<'a> {
    pub(crate) fn new(level: &'a Level) -> Self {
        Self { level }
    }
}

impl<'a> fmt::Display for FmtLevel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self.level {
            Level::TRACE => f.pad("TRACE"),
            Level::DEBUG => f.pad("DEBUG"),
            Level::INFO => f.pad(" INFO"),
            Level::WARN => f.pad(" WARN"),
            Level::ERROR => f.pad("ERROR"),
        }
    }
}

struct FmtCtx<'a, S, N> {
    ctx: &'a FmtContext<'a, S, N>,
    span: Option<&'a tracing::span::Id>,
}

impl<'a, S, N: 'a> FmtCtx<'a, S, N>
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    pub(crate) fn new(ctx: &'a FmtContext<'_, S, N>, span: Option<&'a tracing::span::Id>) -> Self {
        Self { ctx, span }
    }

    fn bold(&self) -> Style {
        Style::new()
    }
}

impl<'a, S, N: 'a> fmt::Display for FmtCtx<'a, S, N>
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    N: for<'writer> FormatFields<'writer> + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bold = self.bold();
        let mut seen = false;

        let span = self.span.and_then(|id| self.ctx.span(id)).or_else(|| self.ctx.lookup_current());
        let scope = span.into_iter().flat_map(|span| span.scope().from_root());

        for span in scope {
            seen = true;
            write!(f, "[{}]", bold.paint(span.metadata().name()))?;
        }

        if seen {
            f.write_char(' ')?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct StaticDirective {
    target: String,
    level: LevelFilter,
}

impl StaticDirective {
    pub fn new(target: String, level: LevelFilter) -> Self {
        Self { target, level }
    }
}
