# resource_category_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**createResourceCategory**](resource_category_api.md#createResourceCategory) | **POST** /resourceCategory | Creates a ResourceCategory
**deleteResourceCategory**](resource_category_api.md#deleteResourceCategory) | **DELETE** /resourceCategory/{id} | Deletes a ResourceCategory
**listResourceCategory**](resource_category_api.md#listResourceCategory) | **GET** /resourceCategory | List or find ResourceCategory objects
**patchResourceCategory**](resource_category_api.md#patchResourceCategory) | **PATCH** /resourceCategory/{id} | Updates partially a ResourceCategory
**retrieveResourceCategory**](resource_category_api.md#retrieveResourceCategory) | **GET** /resourceCategory/{id} | Retrieves a ResourceCategory by ID


# **createResourceCategory**
> models::ResourceCategory createResourceCategory(resource_category)
Creates a ResourceCategory

This operation creates a ResourceCategory entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **resource_category** | [**ResourceCategoryCreate**](ResourceCategoryCreate.md)| The ResourceCategory to be created | 

### Return type

[**models::ResourceCategory**](ResourceCategory.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteResourceCategory**
> deleteResourceCategory(id)
Deletes a ResourceCategory

This operation deletes a ResourceCategory entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCategory | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listResourceCategory**
> Vec<models::ResourceCategory> listResourceCategory(optional)
List or find ResourceCategory objects

This operation list or find ResourceCategory entities

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

[**Vec<models::ResourceCategory>**](ResourceCategory.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patchResourceCategory**
> models::ResourceCategory patchResourceCategory(id, resource_category)
Updates partially a ResourceCategory

This operation updates partially a ResourceCategory entity.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCategory | 
  **resource_category** | [**ResourceCategoryUpdate**](ResourceCategoryUpdate.md)| The ResourceCategory to be updated | 

### Return type

[**models::ResourceCategory**](ResourceCategory.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **retrieveResourceCategory**
> models::ResourceCategory retrieveResourceCategory(id, optional)
Retrieves a ResourceCategory by ID

This operation retrieves a ResourceCategory entity. Attribute selection is enabled for all first level attributes.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Identifier of the ResourceCategory | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Identifier of the ResourceCategory | 
 **fields** | **String**| Comma-separated properties to provide in response | 

### Return type

[**models::ResourceCategory**](ResourceCategory.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

