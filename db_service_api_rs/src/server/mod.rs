use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     RiotApiResponse,
     ServerChallengerGetResponse,
     ServerGrandmasterGetResponse,
     ServerMatchListGetResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/riot/$",
            r"^/(?P<Server>[^/?#]*)/challenger$",
            r"^/(?P<Server>[^/?#]*)/grandmaster$",
            r"^/(?P<Server>[^/?#]*)/matchList$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_RIOT_: usize = 0;
    pub(crate) static ID_SERVER_CHALLENGER: usize = 1;
    lazy_static! {
        pub static ref REGEX_SERVER_CHALLENGER: regex::Regex =
            regex::Regex::new(r"^/(?P<Server>[^/?#]*)/challenger$")
                .expect("Unable to create regex for SERVER_CHALLENGER");
    }
    pub(crate) static ID_SERVER_GRANDMASTER: usize = 2;
    lazy_static! {
        pub static ref REGEX_SERVER_GRANDMASTER: regex::Regex =
            regex::Regex::new(r"^/(?P<Server>[^/?#]*)/grandmaster$")
                .expect("Unable to create regex for SERVER_GRANDMASTER");
    }
    pub(crate) static ID_SERVER_MATCHLIST: usize = 3;
    lazy_static! {
        pub static ref REGEX_SERVER_MATCHLIST: regex::Regex =
            regex::Regex::new(r"^/(?P<Server>[^/?#]*)/matchList$")
                .expect("Unable to create regex for SERVER_MATCHLIST");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match &method {

            // RiotApi - GET /riot/
            &hyper::Method::GET if path.matched(paths::ID_RIOT_) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_riot_api_url = query_params.iter().filter(|e| e.0 == "riot_api_url").map(|e| e.1.to_owned())
                    .nth(0);
                let param_riot_api_url = match param_riot_api_url {
                    Some(param_riot_api_url) => {
                        let param_riot_api_url =
                            <String as std::str::FromStr>::from_str
                                (&param_riot_api_url);
                        match param_riot_api_url {
                            Ok(param_riot_api_url) => Some(param_riot_api_url),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter riot_api_url - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter riot_api_url")),
                        }
                    },
                    None => None,
                };
                let param_riot_api_url = match param_riot_api_url {
                    Some(param_riot_api_url) => param_riot_api_url,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter riot_api_url"))
                        .expect("Unable to create Bad Request response for missing query parameter riot_api_url")),
                };

                                let result = api_impl.riot_api(
                                            param_riot_api_url,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RiotApiResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RIOT_API_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                RiotApiResponse::Status400
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                RiotApiResponse::Status500
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ServerChallengerGet - GET /{Server}/challenger
            &hyper::Method::GET if path.matched(paths::ID_SERVER_CHALLENGER) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVER_CHALLENGER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVER_CHALLENGER in set but failed match against \"{}\"", path, paths::REGEX_SERVER_CHALLENGER.as_str())
                    );

                let param_server = match percent_encoding::percent_decode(path_params["Server"].as_bytes()).decode_utf8() {
                    Ok(param_server) => match param_server.parse::<String>() {
                        Ok(param_server) => param_server,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Server: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Server"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.server_challenger_get(
                                            param_server,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ServerChallengerGetResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERVER_CHALLENGER_GET_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ServerChallengerGetResponse::Status400
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                ServerChallengerGetResponse::Status500
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ServerGrandmasterGet - GET /{Server}/grandmaster
            &hyper::Method::GET if path.matched(paths::ID_SERVER_GRANDMASTER) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVER_GRANDMASTER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVER_GRANDMASTER in set but failed match against \"{}\"", path, paths::REGEX_SERVER_GRANDMASTER.as_str())
                    );

                let param_server = match percent_encoding::percent_decode(path_params["Server"].as_bytes()).decode_utf8() {
                    Ok(param_server) => match param_server.parse::<String>() {
                        Ok(param_server) => param_server,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Server: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Server"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.server_grandmaster_get(
                                            param_server,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ServerGrandmasterGetResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERVER_GRANDMASTER_GET_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ServerGrandmasterGetResponse::Status400
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                ServerGrandmasterGetResponse::Status500
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ServerMatchListGet - GET /{Server}/matchList
            &hyper::Method::GET if path.matched(paths::ID_SERVER_MATCHLIST) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_SERVER_MATCHLIST
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE SERVER_MATCHLIST in set but failed match against \"{}\"", path, paths::REGEX_SERVER_MATCHLIST.as_str())
                    );

                let param_server = match percent_encoding::percent_decode(path_params["Server"].as_bytes()).decode_utf8() {
                    Ok(param_server) => match param_server.parse::<String>() {
                        Ok(param_server) => param_server,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter Server: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["Server"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_player = query_params.iter().filter(|e| e.0 == "Player").map(|e| e.1.to_owned())
                    .nth(0);
                let param_player = match param_player {
                    Some(param_player) => {
                        let param_player =
                            <String as std::str::FromStr>::from_str
                                (&param_player);
                        match param_player {
                            Ok(param_player) => Some(param_player),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter Player - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter Player")),
                        }
                    },
                    None => None,
                };
                let param_player = match param_player {
                    Some(param_player) => param_player,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter Player"))
                        .expect("Unable to create Bad Request response for missing query parameter Player")),
                };

                                let result = api_impl.server_match_list_get(
                                            param_server,
                                            param_player,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ServerMatchListGetResponse::Status200
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for SERVER_MATCH_LIST_GET_STATUS200"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ServerMatchListGetResponse::Status400
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                ServerMatchListGetResponse::Status500
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_RIOT_) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVER_CHALLENGER) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVER_GRANDMASTER) => method_not_allowed(),
            _ if path.matched(paths::ID_SERVER_MATCHLIST) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // RiotApi - GET /riot/
            &hyper::Method::GET if path.matched(paths::ID_RIOT_) => Ok("RiotApi"),
            // ServerChallengerGet - GET /{Server}/challenger
            &hyper::Method::GET if path.matched(paths::ID_SERVER_CHALLENGER) => Ok("ServerChallengerGet"),
            // ServerGrandmasterGet - GET /{Server}/grandmaster
            &hyper::Method::GET if path.matched(paths::ID_SERVER_GRANDMASTER) => Ok("ServerGrandmasterGet"),
            // ServerMatchListGet - GET /{Server}/matchList
            &hyper::Method::GET if path.matched(paths::ID_SERVER_MATCHLIST) => Ok("ServerMatchListGet"),
            _ => Err(()),
        }
    }
}
