use async_trait::async_trait;
use crate::lib::client::Client;
use crate::lib::context::{AsyncSafe, Context, StateTrait};

#[async_trait::async_trait]
pub trait Middleware<T, C, S = ()>
where C: Callback<T, S> + Send + Sync, S: StateTrait, T: AsyncSafe {

    async fn call_middleware(self, client: Context<S>) -> C::Return;
}

#[async_trait::async_trait]
pub trait Callback<T, S = ()> : Creatable where S: StateTrait, T: AsyncSafe {
    type Return : AsyncSafe;
    async fn call(&self, client: Context<S>, input: T) -> Self::Return;
}

pub trait Creatable {
    fn new() -> Self;
}

#[async_trait::async_trait]
impl<T, C, S> Middleware<T, C, S> for T where C: Callback<T, S> + AsyncSafe, S: StateTrait, T: AsyncSafe {
    async fn call_middleware(self, client: Context<S>) -> C::Return {
        let item = <C as Creatable>::new();
        item.call(client, self).await
    }
}

pub struct NoAction;

impl Creatable for NoAction {
    fn new() -> Self { NoAction }
}

#[async_trait::async_trait]
impl<T, S> Callback<T, S> for NoAction where S: StateTrait, T: AsyncSafe {
    type Return = ();
    async fn call(&self, _: Context<S>, _: T) -> Self::Return {

    }
}

#[async_trait::async_trait]
impl<T, S> Middleware<T, NoAction, S> for T where T: AsyncSafe, S: StateTrait {
    async fn call_middleware(self, _: Context<S>) -> () {

    }
}