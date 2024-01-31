# \RelationshipQueriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check**](RelationshipQueriesApi.md#check) | **POST** /stores/{store_id}/check | Check whether a user is authorized to access an object
[**expand**](RelationshipQueriesApi.md#expand) | **POST** /stores/{store_id}/expand | Expand all relationships in userset tree format, and following userset rewrite rules.  Useful to reason about and debug a certain relationship
[**list_objects**](RelationshipQueriesApi.md#list_objects) | **POST** /stores/{store_id}/list-objects | List all objects of the given type that the user has a relation with
[**streamed_list_objects**](RelationshipQueriesApi.md#streamed_list_objects) | **POST** /stores/{store_id}/streamed-list-objects | [EXPERIMENTAL] Stream all objects of the given type that the user has a relation with



## check

> crate::models::CheckResponse check(store_id, body)
Check whether a user is authorized to access an object

The Check API queries to check if the user has a certain relationship with an object in a certain store. A `contextual_tuples` object may also be included in the body of the request. This object contains one field `tuple_keys`, which is an array of tuple keys. Each of these tuples may have an associated `condition`. You may also provide an `authorization_model_id` in the body. This will be used to assert that the input `tuple_key` is valid for the model specified. If not specified, the assertion will be made against the latest authorization model ID. It is strongly recommended to specify authorization model id for better performance. You may also provide a `context` object that will be used to evaluate the conditioned tuples in the system. It is strongly recommended to provide a value for all the input parameters of all the conditions, to ensure that all tuples be evaluated correctly. The response will return whether the relationship exists in the field `allowed`.  ## Example In order to check if user `user:anne` of type `user` has a `reader` relationship with object `document:2021-budget` given the following contextual tuple ```json {   \"user\": \"user:anne\",   \"relation\": \"member\",   \"object\": \"time_slot:office_hours\" } ``` the Check API can be used with the following request body: ```json {   \"tuple_key\": {     \"user\": \"user:anne\",     \"relation\": \"reader\",     \"object\": \"document:2021-budget\"   },   \"contextual_tuples\": {     \"tuple_keys\": [       {         \"user\": \"user:anne\",         \"relation\": \"member\",         \"object\": \"time_slot:office_hours\"       }     ]   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` OpenFGA's response will include `{ \"allowed\": true }` if there is a relationship and `{ \"allowed\": false }` if there isn't.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**CheckRequest**](CheckRequest.md) |  | [required] |

### Return type

[**crate::models::CheckResponse**](CheckResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expand

> crate::models::ExpandResponse expand(store_id, body)
Expand all relationships in userset tree format, and following userset rewrite rules.  Useful to reason about and debug a certain relationship

The Expand API will return all users and usersets that have certain relationship with an object in a certain store. This is different from the `/stores/{store_id}/read` API in that both users and computed usersets are returned. Body parameters `tuple_key.object` and `tuple_key.relation` are all required. The response will return a tree whose leaves are the specific users and usersets. Union, intersection and difference operator are located in the intermediate nodes.  ## Example To expand all users that have the `reader` relationship with object `document:2021-budget`, use the Expand API with the following request body ```json {   \"tuple_key\": {     \"object\": \"document:2021-budget\",     \"relation\": \"reader\"   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` OpenFGA's response will be a userset tree of the users and usersets that have read access to the document. ```json {   \"tree\":{     \"root\":{       \"type\":\"document:2021-budget#reader\",       \"union\":{         \"nodes\":[           {             \"type\":\"document:2021-budget#reader\",             \"leaf\":{               \"users\":{                 \"users\":[                   \"user:bob\"                 ]               }             }           },           {             \"type\":\"document:2021-budget#reader\",             \"leaf\":{               \"computed\":{                 \"userset\":\"document:2021-budget#writer\"               }             }           }         ]       }     }   } } ``` The caller can then call expand API for the `writer` relationship for the `document:2021-budget`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**ExpandRequest**](ExpandRequest.md) |  | [required] |

### Return type

[**crate::models::ExpandResponse**](ExpandResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_objects

> crate::models::ListObjectsResponse list_objects(store_id, body)
List all objects of the given type that the user has a relation with

The ListObjects API returns a list of all the objects of the given type that the user has a relation with. To achieve this, both the store tuples and the authorization model are used. An `authorization_model_id` may be specified in the body. If it is not specified, the latest authorization model ID will be used. It is strongly recommended to specify authorization model id for better performance. You may also specify `contextual_tuples` that will be treated as regular tuples. Each of these tuples may have an associated `condition`. You may also provide a `context` object that will be used to evaluate the conditioned tuples in the system. It is strongly recommended to provide a value for all the input parameters of all the conditions, to ensure that all tuples be evaluated correctly. The response will contain the related objects in an array in the \"objects\" field of the response and they will be strings in the object format `<type>:<id>` (e.g. \"document:roadmap\"). The number of objects in the response array will be limited by the execution timeout specified in the flag OPENFGA_LIST_OBJECTS_DEADLINE and by the upper bound specified in the flag OPENFGA_LIST_OBJECTS_MAX_RESULTS, whichever is hit first. The objects given will not be sorted, and therefore two identical calls can give a given different set of objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**ListObjectsRequest**](ListObjectsRequest.md) |  | [required] |

### Return type

[**crate::models::ListObjectsResponse**](ListObjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## streamed_list_objects

> crate::models::StreamResultOfStreamedListObjectsResponse streamed_list_objects(store_id, body)
[EXPERIMENTAL] Stream all objects of the given type that the user has a relation with

The Streamed ListObjects API is very similar to the the ListObjects API, with two differences:  1. Instead of collecting all objects before returning a response, it streams them to the client as they are collected.  2. The number of results returned is only limited by the execution timeout specified in the flag OPENFGA_LIST_OBJECTS_DEADLINE.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**ListObjectsRequest**](ListObjectsRequest.md) |  | [required] |

### Return type

[**crate::models::StreamResultOfStreamedListObjectsResponse**](Stream_result_of_StreamedListObjectsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

