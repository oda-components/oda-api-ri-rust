# notification_listeners_client_side_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**listenToExportJobCreateEvent**](notification_listeners_client_side_api.md#listenToExportJobCreateEvent) | **POST** /listener/exportJobCreateEvent | Client listener for entity ExportJobCreateEvent
**listenToExportJobStateChangeEvent**](notification_listeners_client_side_api.md#listenToExportJobStateChangeEvent) | **POST** /listener/exportJobStateChangeEvent | Client listener for entity ExportJobStateChangeEvent
**listenToImportJobCreateEvent**](notification_listeners_client_side_api.md#listenToImportJobCreateEvent) | **POST** /listener/importJobCreateEvent | Client listener for entity ImportJobCreateEvent
**listenToImportJobStateChangeEvent**](notification_listeners_client_side_api.md#listenToImportJobStateChangeEvent) | **POST** /listener/importJobStateChangeEvent | Client listener for entity ImportJobStateChangeEvent
**listenToResourceCandidateChangeEvent**](notification_listeners_client_side_api.md#listenToResourceCandidateChangeEvent) | **POST** /listener/resourceCandidateChangeEvent | Client listener for entity ResourceCandidateChangeEvent
**listenToResourceCandidateCreateEvent**](notification_listeners_client_side_api.md#listenToResourceCandidateCreateEvent) | **POST** /listener/resourceCandidateCreateEvent | Client listener for entity ResourceCandidateCreateEvent
**listenToResourceCandidateDeleteEvent**](notification_listeners_client_side_api.md#listenToResourceCandidateDeleteEvent) | **POST** /listener/resourceCandidateDeleteEvent | Client listener for entity ResourceCandidateDeleteEvent
**listenToResourceCatalogChangeEvent**](notification_listeners_client_side_api.md#listenToResourceCatalogChangeEvent) | **POST** /listener/resourceCatalogChangeEvent | Client listener for entity ResourceCatalogChangeEvent
**listenToResourceCatalogCreateEvent**](notification_listeners_client_side_api.md#listenToResourceCatalogCreateEvent) | **POST** /listener/resourceCatalogCreateEvent | Client listener for entity ResourceCatalogCreateEvent
**listenToResourceCatalogDeleteEvent**](notification_listeners_client_side_api.md#listenToResourceCatalogDeleteEvent) | **POST** /listener/resourceCatalogDeleteEvent | Client listener for entity ResourceCatalogDeleteEvent
**listenToResourceCategoryChangeEvent**](notification_listeners_client_side_api.md#listenToResourceCategoryChangeEvent) | **POST** /listener/resourceCategoryChangeEvent | Client listener for entity ResourceCategoryChangeEvent
**listenToResourceCategoryCreateEvent**](notification_listeners_client_side_api.md#listenToResourceCategoryCreateEvent) | **POST** /listener/resourceCategoryCreateEvent | Client listener for entity ResourceCategoryCreateEvent
**listenToResourceCategoryDeleteEvent**](notification_listeners_client_side_api.md#listenToResourceCategoryDeleteEvent) | **POST** /listener/resourceCategoryDeleteEvent | Client listener for entity ResourceCategoryDeleteEvent
**listenToResourceSpecificationChangeEvent**](notification_listeners_client_side_api.md#listenToResourceSpecificationChangeEvent) | **POST** /listener/resourceSpecificationChangeEvent | Client listener for entity ResourceSpecificationChangeEvent
**listenToResourceSpecificationCreateEvent**](notification_listeners_client_side_api.md#listenToResourceSpecificationCreateEvent) | **POST** /listener/resourceSpecificationCreateEvent | Client listener for entity ResourceSpecificationCreateEvent
**listenToResourceSpecificationDeleteEvent**](notification_listeners_client_side_api.md#listenToResourceSpecificationDeleteEvent) | **POST** /listener/resourceSpecificationDeleteEvent | Client listener for entity ResourceSpecificationDeleteEvent


# **listenToExportJobCreateEvent**
> models::EventSubscription listenToExportJobCreateEvent(data)
Client listener for entity ExportJobCreateEvent

Example of a client listener for receiving the notification ExportJobCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ExportJobCreateEvent**](ExportJobCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToExportJobStateChangeEvent**
> models::EventSubscription listenToExportJobStateChangeEvent(data)
Client listener for entity ExportJobStateChangeEvent

Example of a client listener for receiving the notification ExportJobStateChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ExportJobStateChangeEvent**](ExportJobStateChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToImportJobCreateEvent**
> models::EventSubscription listenToImportJobCreateEvent(data)
Client listener for entity ImportJobCreateEvent

Example of a client listener for receiving the notification ImportJobCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ImportJobCreateEvent**](ImportJobCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToImportJobStateChangeEvent**
> models::EventSubscription listenToImportJobStateChangeEvent(data)
Client listener for entity ImportJobStateChangeEvent

Example of a client listener for receiving the notification ImportJobStateChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ImportJobStateChangeEvent**](ImportJobStateChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCandidateChangeEvent**
> models::EventSubscription listenToResourceCandidateChangeEvent(data)
Client listener for entity ResourceCandidateChangeEvent

Example of a client listener for receiving the notification ResourceCandidateChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCandidateChangeEvent**](ResourceCandidateChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCandidateCreateEvent**
> models::EventSubscription listenToResourceCandidateCreateEvent(data)
Client listener for entity ResourceCandidateCreateEvent

Example of a client listener for receiving the notification ResourceCandidateCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCandidateCreateEvent**](ResourceCandidateCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCandidateDeleteEvent**
> models::EventSubscription listenToResourceCandidateDeleteEvent(data)
Client listener for entity ResourceCandidateDeleteEvent

Example of a client listener for receiving the notification ResourceCandidateDeleteEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCandidateDeleteEvent**](ResourceCandidateDeleteEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCatalogChangeEvent**
> models::EventSubscription listenToResourceCatalogChangeEvent(data)
Client listener for entity ResourceCatalogChangeEvent

Example of a client listener for receiving the notification ResourceCatalogChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCatalogChangeEvent**](ResourceCatalogChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCatalogCreateEvent**
> models::EventSubscription listenToResourceCatalogCreateEvent(data)
Client listener for entity ResourceCatalogCreateEvent

Example of a client listener for receiving the notification ResourceCatalogCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCatalogCreateEvent**](ResourceCatalogCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCatalogDeleteEvent**
> models::EventSubscription listenToResourceCatalogDeleteEvent(data)
Client listener for entity ResourceCatalogDeleteEvent

Example of a client listener for receiving the notification ResourceCatalogDeleteEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCatalogDeleteEvent**](ResourceCatalogDeleteEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCategoryChangeEvent**
> models::EventSubscription listenToResourceCategoryChangeEvent(data)
Client listener for entity ResourceCategoryChangeEvent

Example of a client listener for receiving the notification ResourceCategoryChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCategoryChangeEvent**](ResourceCategoryChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCategoryCreateEvent**
> models::EventSubscription listenToResourceCategoryCreateEvent(data)
Client listener for entity ResourceCategoryCreateEvent

Example of a client listener for receiving the notification ResourceCategoryCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCategoryCreateEvent**](ResourceCategoryCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceCategoryDeleteEvent**
> models::EventSubscription listenToResourceCategoryDeleteEvent(data)
Client listener for entity ResourceCategoryDeleteEvent

Example of a client listener for receiving the notification ResourceCategoryDeleteEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceCategoryDeleteEvent**](ResourceCategoryDeleteEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceSpecificationChangeEvent**
> models::EventSubscription listenToResourceSpecificationChangeEvent(data)
Client listener for entity ResourceSpecificationChangeEvent

Example of a client listener for receiving the notification ResourceSpecificationChangeEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceSpecificationChangeEvent**](ResourceSpecificationChangeEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceSpecificationCreateEvent**
> models::EventSubscription listenToResourceSpecificationCreateEvent(data)
Client listener for entity ResourceSpecificationCreateEvent

Example of a client listener for receiving the notification ResourceSpecificationCreateEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceSpecificationCreateEvent**](ResourceSpecificationCreateEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listenToResourceSpecificationDeleteEvent**
> models::EventSubscription listenToResourceSpecificationDeleteEvent(data)
Client listener for entity ResourceSpecificationDeleteEvent

Example of a client listener for receiving the notification ResourceSpecificationDeleteEvent

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**ResourceSpecificationDeleteEvent**](ResourceSpecificationDeleteEvent.md)| The event data | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

