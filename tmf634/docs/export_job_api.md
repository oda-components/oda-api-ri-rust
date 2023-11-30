# export_job_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createExportJob**](export_job_api.md#createExportJob) | **POST** /exportJob | Creates a ExportJob
**deleteExportJob**](export_job_api.md#deleteExportJob) | **DELETE** /exportJob/{id} | Deletes a ExportJob
**listExportJob**](export_job_api.md#listExportJob) | **GET** /exportJob | List or find ExportJob objects
**retrieveExportJob**](export_job_api.md#retrieveExportJob) | **GET** /exportJob/{id} | Retrieves a ExportJob by ID


# **createExportJob**
> models::ExportJob createExportJob(export_job)
Creates a ExportJob

This operation creates a ExportJob entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **export_job** | [**ExportJobCreate**](ExportJobCreate.md)| The ExportJob to be created | 

### Return type

[**models::ExportJob**](ExportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteExportJob**
> deleteExportJob(id)
Deletes a ExportJob

This operation deletes a ExportJob entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ExportJob | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listExportJob**
> Vec<models::ExportJob> listExportJob(optional)
List or find ExportJob objects

This operation list or find ExportJob entities

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **fields** | **String**| Comma-separated properties to be provided in response | 
 **offset** | **i32**| Requested index for start of resources to be provided in response | 
 **limit** | **i32**| Requested number of resources to be provided in response | 

### Return type

[**Vec<models::ExportJob>**](ExportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveExportJob**
> models::ExportJob retrieveExportJob(id, optional)
Retrieves a ExportJob by ID

This operation retrieves a ExportJob entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ExportJob | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ExportJob | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ExportJob**](ExportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

