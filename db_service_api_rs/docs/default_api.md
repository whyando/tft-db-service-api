# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**riotApi**](default_api.md#riotApi) | **GET** /riotApi | Make riot api request or use cached result


# **riotApi**
> serde_json::Value riotApi(url, optional)
Make riot api request or use cached result

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **url** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **url** | **String**|  | 
 **force** | **bool**|  | [default to false]

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

