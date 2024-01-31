# \AssertionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_assertions**](AssertionsApi.md#read_assertions) | **GET** /stores/{store_id}/assertions/{authorization_model_id} | Read assertions for an authorization model ID
[**write_assertions**](AssertionsApi.md#write_assertions) | **PUT** /stores/{store_id}/assertions/{authorization_model_id} | Upsert assertions for an authorization model ID



## read_assertions

> crate::models::ReadAssertionsResponse read_assertions(store_id, authorization_model_id)
Read assertions for an authorization model ID

The ReadAssertions API will return, for a given authorization model id, all the assertions stored for it. An assertion is an object that contains a tuple key, and the expectation of whether a call to the Check API of that tuple key will return true or false. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**authorization_model_id** | **String** |  | [required] |

### Return type

[**crate::models::ReadAssertionsResponse**](ReadAssertionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_assertions

> write_assertions(store_id, authorization_model_id, body)
Upsert assertions for an authorization model ID

The WriteAssertions API will upsert new assertions for an authorization model id, or overwrite the existing ones. An assertion is an object that contains a tuple key, and the expectation of whether a call to the Check API of that tuple key will return true or false. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**authorization_model_id** | **String** |  | [required] |
**body** | [**WriteAssertionsRequest**](WriteAssertionsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

