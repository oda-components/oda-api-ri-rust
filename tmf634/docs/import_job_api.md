# import_job_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createImportJob**](import_job_api.md#createImportJob) | **POST** /importJob | Creates a ImportJob
**deleteImportJob**](import_job_api.md#deleteImportJob) | **DELETE** /importJob/{id} | Deletes a ImportJob
**listImportJob**](import_job_api.md#listImportJob) | **GET** /importJob | List or find ImportJob objects
**retrieveImportJob**](import_job_api.md#retrieveImportJob) | **GET** /importJob/{id} | Retrieves a ImportJob by ID


# **createImportJob**
> models::ImportJob createImportJob(import_job)
Creates a ImportJob

This operation creates a ImportJob entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **import_job** | [**ImportJobCreate**](ImportJobCreate.md)| The ImportJob to be created | 

### Return type

[**models::ImportJob**](ImportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteImportJob**
> deleteImportJob(id)
Deletes a ImportJob

This operation deletes a ImportJob entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ImportJob | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listImportJob**
> Vec<models::ImportJob> listImportJob(optional)
List or find ImportJob objects

This operation list or find ImportJob entities

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

[**Vec<models::ImportJob>**](ImportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveImportJob**
> models::ImportJob retrieveImportJob(id, optional)
Retrieves a ImportJob by ID

This operation retrieves a ImportJob entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ImportJob | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ImportJob | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ImportJob**](ImportJob.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

