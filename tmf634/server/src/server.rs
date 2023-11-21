//! Main library entry point for tmf634_server implementation.

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

use oda_sdk_tmf634::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        oda_sdk_tmf634::server::context::MakeAddContext::<_, EmptyContext>::new(
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
            ssl.set_private_key_file("tmf634/server/cert/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("tmf634/server/cert/server-chain.pem").expect("Failed to set certificate chain");
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


use oda_sdk_tmf634::{
    Api,
    RegisterListenerResponse,
    UnregisterListenerResponse,
    CreateExportJobResponse,
    DeleteExportJobResponse,
    ListExportJobResponse,
    RetrieveExportJobResponse,
    CreateImportJobResponse,
    DeleteImportJobResponse,
    ListImportJobResponse,
    RetrieveImportJobResponse,
    ListenToExportJobCreateEventResponse,
    ListenToExportJobStateChangeEventResponse,
    ListenToImportJobCreateEventResponse,
    ListenToImportJobStateChangeEventResponse,
    ListenToResourceCandidateChangeEventResponse,
    ListenToResourceCandidateCreateEventResponse,
    ListenToResourceCandidateDeleteEventResponse,
    ListenToResourceCatalogChangeEventResponse,
    ListenToResourceCatalogCreateEventResponse,
    ListenToResourceCatalogDeleteEventResponse,
    ListenToResourceCategoryChangeEventResponse,
    ListenToResourceCategoryCreateEventResponse,
    ListenToResourceCategoryDeleteEventResponse,
    ListenToResourceSpecificationChangeEventResponse,
    ListenToResourceSpecificationCreateEventResponse,
    ListenToResourceSpecificationDeleteEventResponse,
    CreateResourceCandidateResponse,
    DeleteResourceCandidateResponse,
    ListResourceCandidateResponse,
    PatchResourceCandidateResponse,
    RetrieveResourceCandidateResponse,
    CreateResourceCatalogResponse,
    DeleteResourceCatalogResponse,
    ListResourceCatalogResponse,
    PatchResourceCatalogResponse,
    RetrieveResourceCatalogResponse,
    CreateResourceCategoryResponse,
    DeleteResourceCategoryResponse,
    ListResourceCategoryResponse,
    PatchResourceCategoryResponse,
    RetrieveResourceCategoryResponse,
    CreateResourceSpecificationResponse,
    DeleteResourceSpecificationResponse,
    ListResourceSpecificationResponse,
    PatchResourceSpecificationResponse,
    RetrieveResourceSpecificationResponse,
};
use oda_sdk_tmf634::server::MakeService;
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

    /// Creates a ExportJob
    async fn create_export_job(
        &self,
        export_job: models::ExportJobCreate,
        context: &C) -> Result<CreateExportJobResponse, ApiError>
    {
        info!("create_export_job({:?}) - X-Span-ID: {:?}", export_job, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ExportJob
    async fn delete_export_job(
        &self,
        id: String,
        context: &C) -> Result<DeleteExportJobResponse, ApiError>
    {
        info!("delete_export_job(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ExportJob objects
    async fn list_export_job(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListExportJobResponse, ApiError>
    {
        info!("list_export_job({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ExportJob by ID
    async fn retrieve_export_job(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveExportJobResponse, ApiError>
    {
        info!("retrieve_export_job(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a ImportJob
    async fn create_import_job(
        &self,
        import_job: models::ImportJobCreate,
        context: &C) -> Result<CreateImportJobResponse, ApiError>
    {
        info!("create_import_job({:?}) - X-Span-ID: {:?}", import_job, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ImportJob
    async fn delete_import_job(
        &self,
        id: String,
        context: &C) -> Result<DeleteImportJobResponse, ApiError>
    {
        info!("delete_import_job(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ImportJob objects
    async fn list_import_job(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListImportJobResponse, ApiError>
    {
        info!("list_import_job({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ImportJob by ID
    async fn retrieve_import_job(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveImportJobResponse, ApiError>
    {
        info!("retrieve_import_job(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ExportJobCreateEvent
    async fn listen_to_export_job_create_event(
        &self,
        data: models::ExportJobCreateEvent,
        context: &C) -> Result<ListenToExportJobCreateEventResponse, ApiError>
    {
        info!("listen_to_export_job_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ExportJobStateChangeEvent
    async fn listen_to_export_job_state_change_event(
        &self,
        data: models::ExportJobStateChangeEvent,
        context: &C) -> Result<ListenToExportJobStateChangeEventResponse, ApiError>
    {
        info!("listen_to_export_job_state_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ImportJobCreateEvent
    async fn listen_to_import_job_create_event(
        &self,
        data: models::ImportJobCreateEvent,
        context: &C) -> Result<ListenToImportJobCreateEventResponse, ApiError>
    {
        info!("listen_to_import_job_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ImportJobStateChangeEvent
    async fn listen_to_import_job_state_change_event(
        &self,
        data: models::ImportJobStateChangeEvent,
        context: &C) -> Result<ListenToImportJobStateChangeEventResponse, ApiError>
    {
        info!("listen_to_import_job_state_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCandidateChangeEvent
    async fn listen_to_resource_candidate_change_event(
        &self,
        data: models::ResourceCandidateChangeEvent,
        context: &C) -> Result<ListenToResourceCandidateChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_candidate_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCandidateCreateEvent
    async fn listen_to_resource_candidate_create_event(
        &self,
        data: models::ResourceCandidateCreateEvent,
        context: &C) -> Result<ListenToResourceCandidateCreateEventResponse, ApiError>
    {
        info!("listen_to_resource_candidate_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCandidateDeleteEvent
    async fn listen_to_resource_candidate_delete_event(
        &self,
        data: models::ResourceCandidateDeleteEvent,
        context: &C) -> Result<ListenToResourceCandidateDeleteEventResponse, ApiError>
    {
        info!("listen_to_resource_candidate_delete_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCatalogChangeEvent
    async fn listen_to_resource_catalog_change_event(
        &self,
        data: models::ResourceCatalogChangeEvent,
        context: &C) -> Result<ListenToResourceCatalogChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_catalog_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCatalogCreateEvent
    async fn listen_to_resource_catalog_create_event(
        &self,
        data: models::ResourceCatalogCreateEvent,
        context: &C) -> Result<ListenToResourceCatalogCreateEventResponse, ApiError>
    {
        info!("listen_to_resource_catalog_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCatalogDeleteEvent
    async fn listen_to_resource_catalog_delete_event(
        &self,
        data: models::ResourceCatalogDeleteEvent,
        context: &C) -> Result<ListenToResourceCatalogDeleteEventResponse, ApiError>
    {
        info!("listen_to_resource_catalog_delete_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCategoryChangeEvent
    async fn listen_to_resource_category_change_event(
        &self,
        data: models::ResourceCategoryChangeEvent,
        context: &C) -> Result<ListenToResourceCategoryChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_category_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCategoryCreateEvent
    async fn listen_to_resource_category_create_event(
        &self,
        data: models::ResourceCategoryCreateEvent,
        context: &C) -> Result<ListenToResourceCategoryCreateEventResponse, ApiError>
    {
        info!("listen_to_resource_category_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceCategoryDeleteEvent
    async fn listen_to_resource_category_delete_event(
        &self,
        data: models::ResourceCategoryDeleteEvent,
        context: &C) -> Result<ListenToResourceCategoryDeleteEventResponse, ApiError>
    {
        info!("listen_to_resource_category_delete_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceSpecificationChangeEvent
    async fn listen_to_resource_specification_change_event(
        &self,
        data: models::ResourceSpecificationChangeEvent,
        context: &C) -> Result<ListenToResourceSpecificationChangeEventResponse, ApiError>
    {
        info!("listen_to_resource_specification_change_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceSpecificationCreateEvent
    async fn listen_to_resource_specification_create_event(
        &self,
        data: models::ResourceSpecificationCreateEvent,
        context: &C) -> Result<ListenToResourceSpecificationCreateEventResponse, ApiError>
    {
        info!("listen_to_resource_specification_create_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Client listener for entity ResourceSpecificationDeleteEvent
    async fn listen_to_resource_specification_delete_event(
        &self,
        data: models::ResourceSpecificationDeleteEvent,
        context: &C) -> Result<ListenToResourceSpecificationDeleteEventResponse, ApiError>
    {
        info!("listen_to_resource_specification_delete_event({:?}) - X-Span-ID: {:?}", data, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a ResourceCandidate
    async fn create_resource_candidate(
        &self,
        resource_candidate: models::ResourceCandidateCreate,
        context: &C) -> Result<CreateResourceCandidateResponse, ApiError>
    {
        info!("create_resource_candidate({:?}) - X-Span-ID: {:?}", resource_candidate, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ResourceCandidate
    async fn delete_resource_candidate(
        &self,
        id: String,
        context: &C) -> Result<DeleteResourceCandidateResponse, ApiError>
    {
        info!("delete_resource_candidate(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ResourceCandidate objects
    async fn list_resource_candidate(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListResourceCandidateResponse, ApiError>
    {
        info!("list_resource_candidate({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Updates partially a ResourceCandidate
    async fn patch_resource_candidate(
        &self,
        id: String,
        resource_candidate: models::ResourceCandidateUpdate,
        context: &C) -> Result<PatchResourceCandidateResponse, ApiError>
    {
        info!("patch_resource_candidate(\"{}\", {:?}) - X-Span-ID: {:?}", id, resource_candidate, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ResourceCandidate by ID
    async fn retrieve_resource_candidate(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveResourceCandidateResponse, ApiError>
    {
        info!("retrieve_resource_candidate(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a ResourceCatalog
    async fn create_resource_catalog(
        &self,
        resource_catalog: models::ResourceCatalogCreate,
        context: &C) -> Result<CreateResourceCatalogResponse, ApiError>
    {
        info!("create_resource_catalog({:?}) - X-Span-ID: {:?}", resource_catalog, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ResourceCatalog
    async fn delete_resource_catalog(
        &self,
        id: String,
        context: &C) -> Result<DeleteResourceCatalogResponse, ApiError>
    {
        info!("delete_resource_catalog(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ResourceCatalog objects
    async fn list_resource_catalog(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListResourceCatalogResponse, ApiError>
    {
        info!("list_resource_catalog({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Updates partially a ResourceCatalog
    async fn patch_resource_catalog(
        &self,
        id: String,
        resource_catalog: models::ResourceCatalogUpdate,
        context: &C) -> Result<PatchResourceCatalogResponse, ApiError>
    {
        info!("patch_resource_catalog(\"{}\", {:?}) - X-Span-ID: {:?}", id, resource_catalog, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ResourceCatalog by ID
    async fn retrieve_resource_catalog(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveResourceCatalogResponse, ApiError>
    {
        info!("retrieve_resource_catalog(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a ResourceCategory
    async fn create_resource_category(
        &self,
        resource_category: models::ResourceCategoryCreate,
        context: &C) -> Result<CreateResourceCategoryResponse, ApiError>
    {
        info!("create_resource_category({:?}) - X-Span-ID: {:?}", resource_category, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ResourceCategory
    async fn delete_resource_category(
        &self,
        id: String,
        context: &C) -> Result<DeleteResourceCategoryResponse, ApiError>
    {
        info!("delete_resource_category(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ResourceCategory objects
    async fn list_resource_category(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListResourceCategoryResponse, ApiError>
    {
        info!("list_resource_category({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Updates partially a ResourceCategory
    async fn patch_resource_category(
        &self,
        id: String,
        resource_category: models::ResourceCategoryUpdate,
        context: &C) -> Result<PatchResourceCategoryResponse, ApiError>
    {
        info!("patch_resource_category(\"{}\", {:?}) - X-Span-ID: {:?}", id, resource_category, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ResourceCategory by ID
    async fn retrieve_resource_category(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveResourceCategoryResponse, ApiError>
    {
        info!("retrieve_resource_category(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Creates a ResourceSpecification
    async fn create_resource_specification(
        &self,
        resource_specification: models::ResourceSpecificationCreate,
        context: &C) -> Result<CreateResourceSpecificationResponse, ApiError>
    {
        info!("create_resource_specification({:?}) - X-Span-ID: {:?}", resource_specification, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Deletes a ResourceSpecification
    async fn delete_resource_specification(
        &self,
        id: String,
        context: &C) -> Result<DeleteResourceSpecificationResponse, ApiError>
    {
        info!("delete_resource_specification(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List or find ResourceSpecification objects
    async fn list_resource_specification(
        &self,
        fields: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        context: &C) -> Result<ListResourceSpecificationResponse, ApiError>
    {
        info!("list_resource_specification({:?}, {:?}, {:?}) - X-Span-ID: {:?}", fields, offset, limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Updates partially a ResourceSpecification
    async fn patch_resource_specification(
        &self,
        id: String,
        resource_specification: models::ResourceSpecificationUpdate,
        context: &C) -> Result<PatchResourceSpecificationResponse, ApiError>
    {
        info!("patch_resource_specification(\"{}\", {:?}) - X-Span-ID: {:?}", id, resource_specification, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieves a ResourceSpecification by ID
    async fn retrieve_resource_specification(
        &self,
        id: String,
        fields: Option<String>,
        context: &C) -> Result<RetrieveResourceSpecificationResponse, ApiError>
    {
        info!("retrieve_resource_specification(\"{}\", {:?}) - X-Span-ID: {:?}", id, fields, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
