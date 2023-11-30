# ODA Rust API for TMF 634

## Documentation for API Endpoints

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**registerListener**](docs/events_subscription_api.md#registerListener) | **POST** /hub | Register a listener
[**unregisterListener**](docs/events_subscription_api.md#unregisterListener) | **DELETE** /hub/{id} | Unregister a listener
[**createExportJob**](docs/export_job_api.md#createExportJob) | **POST** /exportJob | Creates a ExportJob
[**deleteExportJob**](docs/export_job_api.md#deleteExportJob) | **DELETE** /exportJob/{id} | Deletes a ExportJob
[**listExportJob**](docs/export_job_api.md#listExportJob) | **GET** /exportJob | List or find ExportJob objects
[**retrieveExportJob**](docs/export_job_api.md#retrieveExportJob) | **GET** /exportJob/{id} | Retrieves a ExportJob by ID
[**createImportJob**](docs/import_job_api.md#createImportJob) | **POST** /importJob | Creates a ImportJob
[**deleteImportJob**](docs/import_job_api.md#deleteImportJob) | **DELETE** /importJob/{id} | Deletes a ImportJob
[**listImportJob**](docs/import_job_api.md#listImportJob) | **GET** /importJob | List or find ImportJob objects
[**retrieveImportJob**](docs/import_job_api.md#retrieveImportJob) | **GET** /importJob/{id} | Retrieves a ImportJob by ID
[**listenToExportJobCreateEvent**](docs/notification_listeners_client_side_api.md#listenToExportJobCreateEvent) | **POST** /listener/exportJobCreateEvent | Client listener for entity ExportJobCreateEvent
[**listenToExportJobStateChangeEvent**](docs/notification_listeners_client_side_api.md#listenToExportJobStateChangeEvent) | **POST** /listener/exportJobStateChangeEvent | Client listener for entity ExportJobStateChangeEvent
[**listenToImportJobCreateEvent**](docs/notification_listeners_client_side_api.md#listenToImportJobCreateEvent) | **POST** /listener/importJobCreateEvent | Client listener for entity ImportJobCreateEvent
[**listenToImportJobStateChangeEvent**](docs/notification_listeners_client_side_api.md#listenToImportJobStateChangeEvent) | **POST** /listener/importJobStateChangeEvent | Client listener for entity ImportJobStateChangeEvent
[**listenToResourceCandidateChangeEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCandidateChangeEvent) | **POST** /listener/resourceCandidateChangeEvent | Client listener for entity ResourceCandidateChangeEvent
[**listenToResourceCandidateCreateEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCandidateCreateEvent) | **POST** /listener/resourceCandidateCreateEvent | Client listener for entity ResourceCandidateCreateEvent
[**listenToResourceCandidateDeleteEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCandidateDeleteEvent) | **POST** /listener/resourceCandidateDeleteEvent | Client listener for entity ResourceCandidateDeleteEvent
[**listenToResourceCatalogChangeEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCatalogChangeEvent) | **POST** /listener/resourceCatalogChangeEvent | Client listener for entity ResourceCatalogChangeEvent
[**listenToResourceCatalogCreateEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCatalogCreateEvent) | **POST** /listener/resourceCatalogCreateEvent | Client listener for entity ResourceCatalogCreateEvent
[**listenToResourceCatalogDeleteEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCatalogDeleteEvent) | **POST** /listener/resourceCatalogDeleteEvent | Client listener for entity ResourceCatalogDeleteEvent
[**listenToResourceCategoryChangeEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCategoryChangeEvent) | **POST** /listener/resourceCategoryChangeEvent | Client listener for entity ResourceCategoryChangeEvent
[**listenToResourceCategoryCreateEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCategoryCreateEvent) | **POST** /listener/resourceCategoryCreateEvent | Client listener for entity ResourceCategoryCreateEvent
[**listenToResourceCategoryDeleteEvent**](docs/notification_listeners_client_side_api.md#listenToResourceCategoryDeleteEvent) | **POST** /listener/resourceCategoryDeleteEvent | Client listener for entity ResourceCategoryDeleteEvent
[**listenToResourceSpecificationChangeEvent**](docs/notification_listeners_client_side_api.md#listenToResourceSpecificationChangeEvent) | **POST** /listener/resourceSpecificationChangeEvent | Client listener for entity ResourceSpecificationChangeEvent
[**listenToResourceSpecificationCreateEvent**](docs/notification_listeners_client_side_api.md#listenToResourceSpecificationCreateEvent) | **POST** /listener/resourceSpecificationCreateEvent | Client listener for entity ResourceSpecificationCreateEvent
[**listenToResourceSpecificationDeleteEvent**](docs/notification_listeners_client_side_api.md#listenToResourceSpecificationDeleteEvent) | **POST** /listener/resourceSpecificationDeleteEvent | Client listener for entity ResourceSpecificationDeleteEvent
[**createResourceCandidate**](docs/resource_candidate_api.md#createResourceCandidate) | **POST** /resourceCandidate | Creates a ResourceCandidate
[**deleteResourceCandidate**](docs/resource_candidate_api.md#deleteResourceCandidate) | **DELETE** /resourceCandidate/{id} | Deletes a ResourceCandidate
[**listResourceCandidate**](docs/resource_candidate_api.md#listResourceCandidate) | **GET** /resourceCandidate | List or find ResourceCandidate objects
[**patchResourceCandidate**](docs/resource_candidate_api.md#patchResourceCandidate) | **PATCH** /resourceCandidate/{id} | Updates partially a ResourceCandidate
[**retrieveResourceCandidate**](docs/resource_candidate_api.md#retrieveResourceCandidate) | **GET** /resourceCandidate/{id} | Retrieves a ResourceCandidate by ID
[**createResourceCatalog**](docs/resource_catalog_api.md#createResourceCatalog) | **POST** /resourceCatalog | Creates a ResourceCatalog
[**deleteResourceCatalog**](docs/resource_catalog_api.md#deleteResourceCatalog) | **DELETE** /resourceCatalog/{id} | Deletes a ResourceCatalog
[**listResourceCatalog**](docs/resource_catalog_api.md#listResourceCatalog) | **GET** /resourceCatalog | List or find ResourceCatalog objects
[**patchResourceCatalog**](docs/resource_catalog_api.md#patchResourceCatalog) | **PATCH** /resourceCatalog/{id} | Updates partially a ResourceCatalog
[**retrieveResourceCatalog**](docs/resource_catalog_api.md#retrieveResourceCatalog) | **GET** /resourceCatalog/{id} | Retrieves a ResourceCatalog by ID
[**createResourceCategory**](docs/resource_category_api.md#createResourceCategory) | **POST** /resourceCategory | Creates a ResourceCategory
[**deleteResourceCategory**](docs/resource_category_api.md#deleteResourceCategory) | **DELETE** /resourceCategory/{id} | Deletes a ResourceCategory
[**listResourceCategory**](docs/resource_category_api.md#listResourceCategory) | **GET** /resourceCategory | List or find ResourceCategory objects
[**patchResourceCategory**](docs/resource_category_api.md#patchResourceCategory) | **PATCH** /resourceCategory/{id} | Updates partially a ResourceCategory
[**retrieveResourceCategory**](docs/resource_category_api.md#retrieveResourceCategory) | **GET** /resourceCategory/{id} | Retrieves a ResourceCategory by ID
[**createResourceSpecification**](docs/resource_specification_api.md#createResourceSpecification) | **POST** /resourceSpecification | Creates a ResourceSpecification
[**deleteResourceSpecification**](docs/resource_specification_api.md#deleteResourceSpecification) | **DELETE** /resourceSpecification/{id} | Deletes a ResourceSpecification
[**listResourceSpecification**](docs/resource_specification_api.md#listResourceSpecification) | **GET** /resourceSpecification | List or find ResourceSpecification objects
[**patchResourceSpecification**](docs/resource_specification_api.md#patchResourceSpecification) | **PATCH** /resourceSpecification/{id} | Updates partially a ResourceSpecification
[**retrieveResourceSpecification**](docs/resource_specification_api.md#retrieveResourceSpecification) | **GET** /resourceSpecification/{id} | Retrieves a ResourceSpecification by ID


## Documentation For Models

 - [Addressable](docs/Addressable.md)
 - [Attachment](docs/Attachment.md)
 - [AttachmentRef](docs/AttachmentRef.md)
 - [AttachmentRefOrValue](docs/AttachmentRefOrValue.md)
 - [CharacteristicSpecificationBase](docs/CharacteristicSpecificationBase.md)
 - [CharacteristicValueSpecification](docs/CharacteristicValueSpecification.md)
 - [ConnectionPointSpecificationRef](docs/ConnectionPointSpecificationRef.md)
 - [ConnectionSpecification](docs/ConnectionSpecification.md)
 - [ConstraintRef](docs/ConstraintRef.md)
 - [EndpointSpecificationRef](docs/EndpointSpecificationRef.md)
 - [Entity](docs/Entity.md)
 - [EntityRef](docs/EntityRef.md)
 - [Error](docs/Error.md)
 - [EventSubscription](docs/EventSubscription.md)
 - [EventSubscriptionInput](docs/EventSubscriptionInput.md)
 - [ExportJob](docs/ExportJob.md)
 - [ExportJobCreate](docs/ExportJobCreate.md)
 - [ExportJobCreateEvent](docs/ExportJobCreateEvent.md)
 - [ExportJobCreateEventPayload](docs/ExportJobCreateEventPayload.md)
 - [ExportJobStateChangeEvent](docs/ExportJobStateChangeEvent.md)
 - [ExportJobStateChangeEventPayload](docs/ExportJobStateChangeEventPayload.md)
 - [Extensible](docs/Extensible.md)
 - [FeatureSpecification](docs/FeatureSpecification.md)
 - [FeatureSpecificationCharacteristic](docs/FeatureSpecificationCharacteristic.md)
 - [FeatureSpecificationCharacteristicRelationship](docs/FeatureSpecificationCharacteristicRelationship.md)
 - [FeatureSpecificationRelationship](docs/FeatureSpecificationRelationship.md)
 - [ImportJob](docs/ImportJob.md)
 - [ImportJobCreate](docs/ImportJobCreate.md)
 - [ImportJobCreateEvent](docs/ImportJobCreateEvent.md)
 - [ImportJobCreateEventPayload](docs/ImportJobCreateEventPayload.md)
 - [ImportJobStateChangeEvent](docs/ImportJobStateChangeEvent.md)
 - [ImportJobStateChangeEventPayload](docs/ImportJobStateChangeEventPayload.md)
 - [JobStateType](docs/JobStateType.md)
 - [LogicalResourceSpecification](docs/LogicalResourceSpecification.md)
 - [PhysicalResourceSpecification](docs/PhysicalResourceSpecification.md)
 - [Quantity](docs/Quantity.md)
 - [RelatedParty](docs/RelatedParty.md)
 - [ResourceCandidate](docs/ResourceCandidate.md)
 - [ResourceCandidateChangeEvent](docs/ResourceCandidateChangeEvent.md)
 - [ResourceCandidateChangeEventPayload](docs/ResourceCandidateChangeEventPayload.md)
 - [ResourceCandidateCreate](docs/ResourceCandidateCreate.md)
 - [ResourceCandidateCreateEvent](docs/ResourceCandidateCreateEvent.md)
 - [ResourceCandidateCreateEventPayload](docs/ResourceCandidateCreateEventPayload.md)
 - [ResourceCandidateDeleteEvent](docs/ResourceCandidateDeleteEvent.md)
 - [ResourceCandidateDeleteEventPayload](docs/ResourceCandidateDeleteEventPayload.md)
 - [ResourceCandidateRef](docs/ResourceCandidateRef.md)
 - [ResourceCandidateUpdate](docs/ResourceCandidateUpdate.md)
 - [ResourceCatalog](docs/ResourceCatalog.md)
 - [ResourceCatalogChangeEvent](docs/ResourceCatalogChangeEvent.md)
 - [ResourceCatalogChangeEventPayload](docs/ResourceCatalogChangeEventPayload.md)
 - [ResourceCatalogCreate](docs/ResourceCatalogCreate.md)
 - [ResourceCatalogCreateEvent](docs/ResourceCatalogCreateEvent.md)
 - [ResourceCatalogCreateEventPayload](docs/ResourceCatalogCreateEventPayload.md)
 - [ResourceCatalogDeleteEvent](docs/ResourceCatalogDeleteEvent.md)
 - [ResourceCatalogDeleteEventPayload](docs/ResourceCatalogDeleteEventPayload.md)
 - [ResourceCatalogUpdate](docs/ResourceCatalogUpdate.md)
 - [ResourceCategory](docs/ResourceCategory.md)
 - [ResourceCategoryChangeEvent](docs/ResourceCategoryChangeEvent.md)
 - [ResourceCategoryChangeEventPayload](docs/ResourceCategoryChangeEventPayload.md)
 - [ResourceCategoryCreate](docs/ResourceCategoryCreate.md)
 - [ResourceCategoryCreateEvent](docs/ResourceCategoryCreateEvent.md)
 - [ResourceCategoryCreateEventPayload](docs/ResourceCategoryCreateEventPayload.md)
 - [ResourceCategoryDeleteEvent](docs/ResourceCategoryDeleteEvent.md)
 - [ResourceCategoryDeleteEventPayload](docs/ResourceCategoryDeleteEventPayload.md)
 - [ResourceCategoryRef](docs/ResourceCategoryRef.md)
 - [ResourceCategoryUpdate](docs/ResourceCategoryUpdate.md)
 - [ResourceFunctionSpecification](docs/ResourceFunctionSpecification.md)
 - [ResourceGraphSpecification](docs/ResourceGraphSpecification.md)
 - [ResourceGraphSpecificationRef](docs/ResourceGraphSpecificationRef.md)
 - [ResourceGraphSpecificationRelationship](docs/ResourceGraphSpecificationRelationship.md)
 - [ResourceSpecification](docs/ResourceSpecification.md)
 - [ResourceSpecificationChangeEvent](docs/ResourceSpecificationChangeEvent.md)
 - [ResourceSpecificationChangeEventPayload](docs/ResourceSpecificationChangeEventPayload.md)
 - [ResourceSpecificationCharacteristic](docs/ResourceSpecificationCharacteristic.md)
 - [ResourceSpecificationCharacteristicRelationship](docs/ResourceSpecificationCharacteristicRelationship.md)
 - [ResourceSpecificationCreate](docs/ResourceSpecificationCreate.md)
 - [ResourceSpecificationCreateEvent](docs/ResourceSpecificationCreateEvent.md)
 - [ResourceSpecificationCreateEventPayload](docs/ResourceSpecificationCreateEventPayload.md)
 - [ResourceSpecificationDeleteEvent](docs/ResourceSpecificationDeleteEvent.md)
 - [ResourceSpecificationDeleteEventPayload](docs/ResourceSpecificationDeleteEventPayload.md)
 - [ResourceSpecificationRef](docs/ResourceSpecificationRef.md)
 - [ResourceSpecificationRelationship](docs/ResourceSpecificationRelationship.md)
 - [ResourceSpecificationUpdate](docs/ResourceSpecificationUpdate.md)
 - [TargetResourceSchema](docs/TargetResourceSchema.md)
 - [TimePeriod](docs/TimePeriod.md)
