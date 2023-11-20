//! Main library entry point for tmf639_server implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use tmf639_server::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        tmf639_server::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("tmf639/server/cert/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("tmf639/server/cert/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use tmf639_server::{
    Api,
    RegisterListenerResponse,
    UnregisterListenerResponse,
    ListenToResourceAttributeValueChangeEventResponse,
    ListenToResourceCreateEventResponse,
    ListenToResourceDeleteEventResponse,
    ListenToResourceStateChangeEventResponse,
    CreateResourceResponse,
    DeleteResourceResponse,
    ListResourceResponse,
    PatchResourceResponse,
    RetrieveResourceResponse,
};
use tmf639_server::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Register a listener
    async fn register_listener(
        &self,
        data: models::EventSubscriptionInput,
        context: &C) -> Result<RegisterListenerResponse, ApiError>
    {
        info!("register_listener({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Unregister a listener
    async fn unregister_listener(
        &self,
        id: String,
        context: &C) -> Result<UnregisterListenerResponse, ApiError>
    {
        info!("unregister_listener(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceAttributeValueChangeEvent
    async fn listen_to_resource_attribute_value_change_event(
        &self,
        data: models::ResourceAttributeValueChangeEvent,
        context: &C) -> Result<ListenToResourceAttributeValueChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_attribute_value_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCreateEvent
    async fn listen_to_resource_create_event(
        &self,
        data: models::ResourceCreateEvent,
        context: &C) -> Result<ListenToResourceCreateEventResponse, ApiError>
    {
        info!("listen_to_resource_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceDeleteEvent
    async fn listen_to_resource_delete_event(
        &self,
        data: models::ResourceDeleteEvent,
        context: &C) -> Result<ListenToResourceDeleteEventResponse, ApiError>
    {
        info!("listen_to_resource_delete_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceStateChangeEvent
    async fn listen_to_resource_state_change_event(
        &self,
        data: models::ResourceStateChangeEvent,
        context: &C) -> Result<ListenToResourceStateChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_state_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a Resource
    async fn create_resource(
        &self,
        resource: models::ResourceCreate,
        context: &C) -> Result<CreateResourceResponse, ApiError>
    {
        info!("create_resource({:?}) - X-Span-ID: {:?}", resource, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a Resource
    async fn delete_resource(
        &self,
        id: String,
        context: &C) -> Result<DeleteResourceResponse, ApiError>
    {
        info!("delete_resource(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find Resource objects
    async fn list_resource(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListResourceResponse, ApiError>
    {
        info!("list_resource({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Updates partially a Resource
    async fn patch_resource(
        &self,
        id: String,
        resource: models::ResourceUpdate,
        context: &C) -> Result<PatchResourceResponse, ApiError>
    {
        info!("patch_resource(\"{}\", {:?}) - X-Span-ID: {:?}", id, resource, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a Resource by ID
    async fn retrieve_resource(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveResourceResponse, ApiError>
    {
        info!("retrieve_resource(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
