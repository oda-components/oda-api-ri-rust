# events_subscription_api

All URIs are relative to *https://serverRoot/tmf-api/resourceCatalog/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
**registerListener**](events_subscription_api.md#registerListener) | **POST** /hub | Register a listener
**unregisterListener**](events_subscription_api.md#unregisterListener) | **DELETE** /hub/{id} | Unregister a listener


# **registerListener**
> models::EventSubscription registerListener(data)
Register a listener

Sets the communication endpoint address the service instance must use to deliver information about its health state, execution state, failures and metrics.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **data** | [**EventSubscriptionInput**](EventSubscriptionInput.md)| Data containing the callback endpoint to deliver the information | 

### Return type

[**models::EventSubscription**](EventSubscription.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json;charset=utf-8
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **unregisterListener**
> unregisterListener(id)
Unregister a listener

Resets the communication endpoint address the service instance must use to deliver information about its health state, execution state, failures and metrics.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| The id of the registered listener | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json;charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

