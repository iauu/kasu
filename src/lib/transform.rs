use async_trait::async_trait;
use crate::lib::context::AsyncSafe;

#[async_trait::async_trait]
pub trait Transform<T>
where T: AsyncSafe, Self: AsyncSafe {

    async fn transform(&self) -> T;
}

#[async_trait::async_trait]
impl<T> Transform<T> for T
where T: AsyncSafe {
    async fn transform(&self) -> T {
        self.clone()
    }
}

#[async_trait::async_trait]
pub trait FromTransform<R> : AsyncSafe {
    async fn from_transform(r: R) -> Self;
}

#[async_trait::async_trait]
impl<T, R> FromTransform<R> for T
where T: AsyncSafe, R: Transform<T> {
    async fn from_transform(r: R) -> Self {
        r.transform().await
    }
}

