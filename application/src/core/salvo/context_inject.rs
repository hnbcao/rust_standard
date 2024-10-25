use std::any::{Any, TypeId};
use std::sync::Arc;
use salvo::{Depot, handler};
use crate::core::context::Context;
use crate::core::errors::{AppError, AppResult};

pub struct ContextInject<T: Send + Sync + 'static> {
    pub context: Arc<T>,
}

#[handler]
impl<T: Send + Sync + 'static> ContextInject<T> {
    async fn handle(&self, depot: &mut Depot) {
        insert_arc(depot, self.context.clone())
    }
}

/// 从上下文中获取Arc<T>信息
pub fn insert_arc<T: Any + Send + Sync>(depot: &mut Depot, t: Arc<T>) {
    depot.insert(&type_key::<Arc<T>>(), t);
}

/// 从上下文中获取T信息
pub fn obtain_arc<T: Any + Send + Sync>(depot: &Depot) -> AppResult<&Arc<T>> {
    let key = &type_key::<Arc<T>>();
    obtain(depot, key)
}

/// 从上下文中获取T信息
pub fn obtain<'a, T: Any + Send + Sync>(depot: &'a Depot, key: &str) -> AppResult<&'a T> {
    depot.get(key).map_err(|err| {
        if let Some(e) = err {
            AppError::ApiRequestParam(format!("{:?}", e))
        } else {
            AppError::ApiRequestParamStr("Please contact the developer to check `router`.")
        }
    })
}

/// 设置泛型类，注意使用Arc等容器时，可能每次获取都不一样
#[inline]
fn type_key<T: 'static>() -> String {
    format!("{:?}", TypeId::of::<T>())
}

pub fn obtain_context(depot: &Depot) -> AppResult<&Arc<Context>> {
    obtain_arc::<Context>(depot)
}