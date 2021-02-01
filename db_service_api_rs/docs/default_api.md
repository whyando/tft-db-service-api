# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**riotApi**](default_api.md#riotApi) | **GET** /riot/ | Make riot api request or use cached result
****](default_api.md#) | **GET** /{Server}/challenger | Get Challenger League
****](default_api.md#) | **GET** /{Server}/grandmaster | Get Grandmaster League
****](default_api.md#) | **GET** /{Server}/matchList | Get Grandmaster League


# **riotApi**
> serde_json::Value riotApi(riot_api_url)
Make riot api request or use cached result

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **riot_api_url** | **String**|  | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (server)
Get Challenger League

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **server** | **String**|  | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (server)
Get Grandmaster League

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **server** | **String**|  | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (server, player)
Get Grandmaster League

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **server** | **String**|  | 
  **player** | **String**|  | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

