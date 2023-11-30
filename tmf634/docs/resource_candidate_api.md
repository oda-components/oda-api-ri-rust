# resource_candidate_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createResourceCandidate**](resource_candidate_api.md#createResourceCandidate) | **POST** /resourceCandidate | Creates a ResourceCandidate
**deleteResourceCandidate**](resource_candidate_api.md#deleteResourceCandidate) | **DELETE** /resourceCandidate/{id} | Deletes a ResourceCandidate
**listResourceCandidate**](resource_candidate_api.md#listResourceCandidate) | **GET** /resourceCandidate | List or find ResourceCandidate objects
**patchResourceCandidate**](resource_candidate_api.md#patchResourceCandidate) | **PATCH** /resourceCandidate/{id} | Updates partially a ResourceCandidate
**retrieveResourceCandidate**](resource_candidate_api.md#retrieveResourceCandidate) | **GET** /resourceCandidate/{id} | Retrieves a ResourceCandidate by ID


# **createResourceCandidate**
> models::ResourceCandidate createResourceCandidate(resource_candidate)
Creates a ResourceCandidate

This operation creates a ResourceCandidate entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **resource_candidate** | [**ResourceCandidateCreate**](ResourceCandidateCreate.md)| The ResourceCandidate to be created | 

### Return type

[**models::ResourceCandidate**](ResourceCandidate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteResourceCandidate**
> deleteResourceCandidate(id)
Deletes a ResourceCandidate

This operation deletes a ResourceCandidate entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCandidate | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listResourceCandidate**
> Vec<models::ResourceCandidate> listResourceCandidate(optional)
List or find ResourceCandidate objects

This operation list or find ResourceCandidate entities

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

[**Vec<models::ResourceCandidate>**](ResourceCandidate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchResourceCandidate**
> models::ResourceCandidate patchResourceCandidate(id, resource_candidate)
Updates partially a ResourceCandidate

This operation updates partially a ResourceCandidate entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCandidate | 
  **resource_candidate** | [**ResourceCandidateUpdate**](ResourceCandidateUpdate.md)| The ResourceCandidate to be updated | 

### Return type

[**models::ResourceCandidate**](ResourceCandidate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveResourceCandidate**
> models::ResourceCandidate retrieveResourceCandidate(id, optional)
Retrieves a ResourceCandidate by ID

This operation retrieves a ResourceCandidate entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCandidate | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ResourceCandidate | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ResourceCandidate**](ResourceCandidate.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

