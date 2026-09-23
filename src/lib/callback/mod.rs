use std::marker::PhantomData;
use crate::lib::context::{AsyncSafe, Context, StateTrait};

pub struct Middleware<T, C, S>
where C: Callback<T, S> + Send + Sync, S: StateTrait, T: Send + Sync + 'static {
    callback: C,
    data: T,
    _hidden: PhantomData<S>,
}

#[async_trait::async_trait]
pub trait Callback<T, S = ()> : Creatable + Send + Sync where S: StateTrait, T: Send + Sync + 'static {
    type Return : AsyncSafe;
    async fn call(&self, ctx: &Context<S>, input: T) -> Self::Return;
}

pub trait Creatable {
    fn new() -> Self;
}

impl<T, C, S> Middleware<T, C, S> where C: Callback<T, S> + Send + Sync, S: StateTrait, T: Send + Sync + 'static {
    pub async fn call_middleware(self, ctx: &Context<S>) -> C::Return {
        self.callback.call(ctx, self.data).await
    }
}

pub struct NoAction;

impl Creatable for NoAction {
    fn new() -> Self { NoAction }
}

#[async_trait::async_trait]
impl<T, S> Callback<T, S> for NoAction where S: StateTrait, T: Send + Sync + 'static {
    type Return = ();
    async fn call(&self, _: &Context<S>, _: T) -> Self::Return {

    }
}

impl<T, C, S> From<T> for Middleware<T, C, S> where C: Callback<T, S>, S: StateTrait, T: Send + Sync + 'static {
    fn from(value: T) -> Self {
        Self {
            callback: C::new(),
            data: value,
            _hidden: PhantomData,
        }
    }
}