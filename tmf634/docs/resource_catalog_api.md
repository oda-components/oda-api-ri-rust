# resource_catalog_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createResourceCatalog**](resource_catalog_api.md#createResourceCatalog) | **POST** /resourceCatalog | Creates a ResourceCatalog
**deleteResourceCatalog**](resource_catalog_api.md#deleteResourceCatalog) | **DELETE** /resourceCatalog/{id} | Deletes a ResourceCatalog
**listResourceCatalog**](resource_catalog_api.md#listResourceCatalog) | **GET** /resourceCatalog | List or find ResourceCatalog objects
**patchResourceCatalog**](resource_catalog_api.md#patchResourceCatalog) | **PATCH** /resourceCatalog/{id} | Updates partially a ResourceCatalog
**retrieveResourceCatalog**](resource_catalog_api.md#retrieveResourceCatalog) | **GET** /resourceCatalog/{id} | Retrieves a ResourceCatalog by ID


# **createResourceCatalog**
> models::ResourceCatalog createResourceCatalog(resource_catalog)
Creates a ResourceCatalog

This operation creates a ResourceCatalog entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **resource_catalog** | [**ResourceCatalogCreate**](ResourceCatalogCreate.md)| The ResourceCatalog to be created | 

### Return type

[**models::ResourceCatalog**](ResourceCatalog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteResourceCatalog**
> deleteResourceCatalog(id)
Deletes a ResourceCatalog

This operation deletes a ResourceCatalog entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCatalog | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listResourceCatalog**
> Vec<models::ResourceCatalog> listResourceCatalog(optional)
List or find ResourceCatalog objects

This operation list or find ResourceCatalog entities

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

[**Vec<models::ResourceCatalog>**](ResourceCatalog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchResourceCatalog**
> models::ResourceCatalog patchResourceCatalog(id, resource_catalog)
Updates partially a ResourceCatalog

This operation updates partially a ResourceCatalog entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCatalog | 
  **resource_catalog** | [**ResourceCatalogUpdate**](ResourceCatalogUpdate.md)| The ResourceCatalog to be updated | 

### Return type

[**models::ResourceCatalog**](ResourceCatalog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveResourceCatalog**
> models::ResourceCatalog retrieveResourceCatalog(id, optional)
Retrieves a ResourceCatalog by ID

This operation retrieves a ResourceCatalog entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCatalog | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ResourceCatalog | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ResourceCatalog**](ResourceCatalog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

