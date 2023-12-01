#![allow(missing_docs, unused_variables, trivial_casts)]

use clap::{App, Arg};
#[allow(unused_imports)]
use futures::{future, stream, Stream};
#[allow(unused_imports)]
use oda_sdk_tmf634::{
    models, Api, ApiNoContext, Client, ContextWrapperExt, CreateExportJobResponse,
    CreateImportJobResponse, CreateResourceCandidateResponse, CreateResourceCatalogResponse,
    CreateResourceCategoryResponse, CreateResourceSpecificationResponse, DeleteExportJobResponse,
    DeleteImportJobResponse, DeleteResourceCandidateResponse, DeleteResourceCatalogResponse,
    DeleteResourceCategoryResponse, DeleteResourceSpecificationResponse, ListExportJobResponse,
    ListImportJobResponse, ListResourceCandidateResponse, ListResourceCatalogResponse,
    ListResourceCategoryResponse, ListResourceSpecificationResponse,
    ListenToExportJobCreateEventResponse, ListenToExportJobStateChangeEventResponse,
    ListenToImportJobCreateEventResponse, ListenToImportJobStateChangeEventResponse,
    ListenToResourceCandidateChangeEventResponse, ListenToResourceCandidateCreateEventResponse,
    ListenToResourceCandidateDeleteEventResponse, ListenToResourceCatalogChangeEventResponse,
    ListenToResourceCatalogCreateEventResponse, ListenToResourceCatalogDeleteEventResponse,
    ListenToResourceCategoryChangeEventResponse, ListenToResourceCategoryCreateEventResponse,
    ListenToResourceCategoryDeleteEventResponse, ListenToResourceSpecificationChangeEventResponse,
    ListenToResourceSpecificationCreateEventResponse,
    ListenToResourceSpecificationDeleteEventResponse, PatchResourceCandidateResponse,
    PatchResourceCatalogResponse, PatchResourceCategoryResponse,
    PatchResourceSpecificationResponse, RegisterListenerResponse, RetrieveExportJobResponse,
    RetrieveImportJobResponse, RetrieveResourceCandidateResponse, RetrieveResourceCatalogResponse,
    RetrieveResourceCategoryResponse, RetrieveResourceSpecificationResponse,
    UnregisterListenerResponse,
};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(
            Arg::with_name("operation")
                .help("Sets the operation to run")
                .possible_values(&[
                    "UnregisterListener",
                    "DeleteExportJob",
                    "ListExportJob",
                    "RetrieveExportJob",
                    "DeleteImportJob",
                    "ListImportJob",
                    "RetrieveImportJob",
                    "DeleteResourceCandidate",
                    "ListResourceCandidate",
                    "RetrieveResourceCandidate",
                    "DeleteResourceCatalog",
                    "ListResourceCatalog",
                    "RetrieveResourceCatalog",
                    "DeleteResourceCategory",
                    "ListResourceCategory",
                    "RetrieveResourceCategory",
                    "DeleteResourceSpecification",
                    "ListResourceSpecification",
                    "RetrieveResourceSpecification",
                ])
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("https")
                .long("https")
                .help("Whether to use HTTPS or not"),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .takes_value(true)
                .default_value("serverRoot")
                .help("Hostname to contact"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .takes_value(true)
                .default_value("8080")
                .help("Port to contact"),
        )
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!(
        "{}://{}:{}",
        if is_https { "https" } else { "http" },
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap()
    );

    let context: ClientContext = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        None as Option<AuthData>,
        XSpanIdString::default()
    );

    let mut client: Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client =
            Box::new(Client::try_new_https(&base_url).expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client =
            Box::new(Client::try_new_http(&base_url).expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        /* Disabled because there's no example.
        Some("RegisterListener") => {
            let result = rt.block_on(client.register_listener(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("UnregisterListener") => {
            let result = rt.block_on(client.unregister_listener("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("CreateExportJob") => {
            let result = rt.block_on(client.create_export_job(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteExportJob") => {
            let result = rt.block_on(client.delete_export_job("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListExportJob") => {
            let result = rt.block_on(client.list_export_job(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("RetrieveExportJob") => {
            let result =
                rt.block_on(client.retrieve_export_job(
                    "id_example".to_string(),
                    Some("fields_example".to_string()),
                ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("CreateImportJob") => {
            let result = rt.block_on(client.create_import_job(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteImportJob") => {
            let result = rt.block_on(client.delete_import_job("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListImportJob") => {
            let result = rt.block_on(client.list_import_job(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("RetrieveImportJob") => {
            let result =
                rt.block_on(client.retrieve_import_job(
                    "id_example".to_string(),
                    Some("fields_example".to_string()),
                ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("ListenToExportJobCreateEvent") => {
            let result = rt.block_on(client.listen_to_export_job_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToExportJobStateChangeEvent") => {
            let result = rt.block_on(client.listen_to_export_job_state_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToImportJobCreateEvent") => {
            let result = rt.block_on(client.listen_to_import_job_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToImportJobStateChangeEvent") => {
            let result = rt.block_on(client.listen_to_import_job_state_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCandidateChangeEvent") => {
            let result = rt.block_on(client.listen_to_resource_candidate_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCandidateCreateEvent") => {
            let result = rt.block_on(client.listen_to_resource_candidate_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCandidateDeleteEvent") => {
            let result = rt.block_on(client.listen_to_resource_candidate_delete_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCatalogChangeEvent") => {
            let result = rt.block_on(client.listen_to_resource_catalog_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCatalogCreateEvent") => {
            let result = rt.block_on(client.listen_to_resource_catalog_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCatalogDeleteEvent") => {
            let result = rt.block_on(client.listen_to_resource_catalog_delete_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCategoryChangeEvent") => {
            let result = rt.block_on(client.listen_to_resource_category_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCategoryCreateEvent") => {
            let result = rt.block_on(client.listen_to_resource_category_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceCategoryDeleteEvent") => {
            let result = rt.block_on(client.listen_to_resource_category_delete_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceSpecificationChangeEvent") => {
            let result = rt.block_on(client.listen_to_resource_specification_change_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceSpecificationCreateEvent") => {
            let result = rt.block_on(client.listen_to_resource_specification_create_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("ListenToResourceSpecificationDeleteEvent") => {
            let result = rt.block_on(client.listen_to_resource_specification_delete_event(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        /* Disabled because there's no example.
        Some("CreateResourceCandidate") => {
            let result = rt.block_on(client.create_resource_candidate(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteResourceCandidate") => {
            let result = rt.block_on(client.delete_resource_candidate("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListResourceCandidate") => {
            let result = rt.block_on(client.list_resource_candidate(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("PatchResourceCandidate") => {
            let result = rt.block_on(client.patch_resource_candidate(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("RetrieveResourceCandidate") => {
            let result = rt.block_on(client.retrieve_resource_candidate(
                "id_example".to_string(),
                Some("fields_example".to_string()),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("CreateResourceCatalog") => {
            let result = rt.block_on(client.create_resource_catalog(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteResourceCatalog") => {
            let result = rt.block_on(client.delete_resource_catalog("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListResourceCatalog") => {
            let result = rt.block_on(client.list_resource_catalog(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("PatchResourceCatalog") => {
            let result = rt.block_on(client.patch_resource_catalog(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("RetrieveResourceCatalog") => {
            let result = rt.block_on(client.retrieve_resource_catalog(
                "id_example".to_string(),
                Some("fields_example".to_string()),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("CreateResourceCategory") => {
            let result = rt.block_on(client.create_resource_category(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteResourceCategory") => {
            let result = rt.block_on(client.delete_resource_category("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListResourceCategory") => {
            let result = rt.block_on(client.list_resource_category(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("PatchResourceCategory") => {
            let result = rt.block_on(client.patch_resource_category(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("RetrieveResourceCategory") => {
            let result = rt.block_on(client.retrieve_resource_category(
                "id_example".to_string(),
                Some("fields_example".to_string()),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("CreateResourceSpecification") => {
            let result = rt.block_on(client.create_resource_specification(
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("DeleteResourceSpecification") => {
            let result =
                rt.block_on(client.delete_resource_specification("id_example".to_string()));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        Some("ListResourceSpecification") => {
            let result = rt.block_on(client.list_resource_specification(
                Some("fields_example".to_string()),
                Some(56),
                Some(56),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        /* Disabled because there's no example.
        Some("PatchResourceSpecification") => {
            let result = rt.block_on(client.patch_resource_specification(
                  "id_example".to_string(),
                  ???
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        Some("RetrieveResourceSpecification") => {
            let result = rt.block_on(client.retrieve_resource_specification(
                "id_example".to_string(),
                Some("fields_example".to_string()),
            ));
            info!(
                "{:?} (X-Span-ID: {:?})",
                result,
                (client.context() as &dyn Has<XSpanIdString>).get().clone()
            );
        }
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
