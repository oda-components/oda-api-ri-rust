# resource_specification_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createResourceSpecification**](resource_specification_api.md#createResourceSpecification) | **POST** /resourceSpecification | Creates a ResourceSpecification
**deleteResourceSpecification**](resource_specification_api.md#deleteResourceSpecification) | **DELETE** /resourceSpecification/{id} | Deletes a ResourceSpecification
**listResourceSpecification**](resource_specification_api.md#listResourceSpecification) | **GET** /resourceSpecification | List or find ResourceSpecification objects
**patchResourceSpecification**](resource_specification_api.md#patchResourceSpecification) | **PATCH** /resourceSpecification/{id} | Updates partially a ResourceSpecification
**retrieveResourceSpecification**](resource_specification_api.md#retrieveResourceSpecification) | **GET** /resourceSpecification/{id} | Retrieves a ResourceSpecification by ID


# **createResourceSpecification**
> models::ResourceSpecification createResourceSpecification(resource_specification)
Creates a ResourceSpecification

This operation creates a ResourceSpecification entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **resource_specification** | [**ResourceSpecificationCreate**](ResourceSpecificationCreate.md)| The ResourceSpecification to be created | 

### Return type

[**models::ResourceSpecification**](ResourceSpecification.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteResourceSpecification**
> deleteResourceSpecification(id)
Deletes a ResourceSpecification

This operation deletes a ResourceSpecification entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceSpecification | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listResourceSpecification**
> Vec<models::ResourceSpecification> listResourceSpecification(optional)
List or find ResourceSpecification objects

This operation list or find ResourceSpecification entities

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

[**Vec<models::ResourceSpecification>**](ResourceSpecification.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchResourceSpecification**
> models::ResourceSpecification patchResourceSpecification(id, resource_specification)
Updates partially a ResourceSpecification

This operation updates partially a ResourceSpecification entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceSpecification | 
  **resource_specification** | [**ResourceSpecificationUpdate**](ResourceSpecificationUpdate.md)| The ResourceSpecification to be updated | 

### Return type

[**models::ResourceSpecification**](ResourceSpecification.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveResourceSpecification**
> models::ResourceSpecification retrieveResourceSpecification(id, optional)
Retrieves a ResourceSpecification by ID

This operation retrieves a ResourceSpecification entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceSpecification | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ResourceSpecification | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ResourceSpecification**](ResourceSpecification.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

