#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "1.0.0";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum MatchHistoryResponse {
    /// 200 OK
    Status200
    (serde_json::Value)
    ,
    /// 400 Bad Request
    Status400
    ,
    /// 500 Internal Server Error
    Status500
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum RiotApiResponse {
    /// 200 OK
    Status200
    (serde_json::Value)
    ,
    /// 400 Bad Request
    Status400
    ,
    /// 500 Internal Server Error
    Status500
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Get match history for a single summoner
    async fn match_history(
        &self,
        puuid: Option<String>,
        name: Option<String>,
        context: &C) -> Result<MatchHistoryResponse, ApiError>;

    /// Make riot api request or use cached result
    async fn riot_api(
        &self,
        url: String,
        force: Option<bool>,
        context: &C) -> Result<RiotApiResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Get match history for a single summoner
    async fn match_history(
        &self,
        puuid: Option<String>,
        name: Option<String>,
        ) -> Result<MatchHistoryResponse, ApiError>;

    /// Make riot api request or use cached result
    async fn riot_api(
        &self,
        url: String,
        force: Option<bool>,
        ) -> Result<RiotApiResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Get match history for a single summoner
    async fn match_history(
        &self,
        puuid: Option<String>,
        name: Option<String>,
        ) -> Result<MatchHistoryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().match_history(puuid, name, &context).await
    }

    /// Make riot api request or use cached result
    async fn riot_api(
        &self,
        url: String,
        force: Option<bool>,
        ) -> Result<RiotApiResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().riot_api(url, force, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
