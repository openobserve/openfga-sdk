# \RelationshipTuplesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read**](RelationshipTuplesApi.md#read) | **POST** /stores/{store_id}/read | Get tuples from the store that matches a query, without following userset rewrite rules
[**read_changes**](RelationshipTuplesApi.md#read_changes) | **GET** /stores/{store_id}/changes | Return a list of all the tuple changes
[**write**](RelationshipTuplesApi.md#write) | **POST** /stores/{store_id}/write | Add or delete tuples from the store



## read

> crate::models::ReadResponse read(store_id, body)
Get tuples from the store that matches a query, without following userset rewrite rules

The Read API will return the tuples for a certain store that match a query filter specified in the body of the request. It is different from the `/stores/{store_id}/expand` API in that it only returns relationship tuples that are stored in the system and satisfy the query.  In the body: 1. `tuple_key` is optional. If not specified, it will return all tuples in the store. 2. `tuple_key.object` is mandatory if `tuple_key` is specified. It can be a full object (e.g., `type:object_id`) or type only (e.g., `type:`). 3. `tuple_key.user` is mandatory if tuple_key is specified in the case the `tuple_key.object` is a type only. ## Examples ### Query for all objects in a type definition To query for all objects that `user:bob` has `reader` relationship in the `document` type definition, call read API with body of ```json {  \"tuple_key\": {      \"user\": \"user:bob\",      \"relation\": \"reader\",      \"object\": \"document:\"   } } ``` The API will return tuples and a continuation token, something like ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ],   \"continuation_token\": \"eyJwayI6IkxBVEVTVF9OU0NPTkZJR19hdXRoMHN0b3JlIiwic2siOiIxem1qbXF3MWZLZExTcUoyN01MdTdqTjh0cWgifQ==\" } ``` This means that `user:bob` has a `reader` relationship with 1 document `document:2021-budget`. Note that this API, unlike the List Objects API, does not evaluate the tuples in the store. The continuation token will be empty if there are no more tuples to query. ### Query for all stored relationship tuples that have a particular relation and object To query for all users that have `reader` relationship with `document:2021-budget`, call read API with body of  ```json {   \"tuple_key\": {      \"object\": \"document:2021-budget\",      \"relation\": \"reader\"    } } ``` The API will return something like  ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ],   \"continuation_token\": \"eyJwayI6IkxBVEVTVF9OU0NPTkZJR19hdXRoMHN0b3JlIiwic2siOiIxem1qbXF3MWZLZExTcUoyN01MdTdqTjh0cWgifQ==\" } ``` This means that `document:2021-budget` has 1 `reader` (`user:bob`).  Note that, even if the model said that all `writers` are also `readers`, the API will not return writers such as `user:anne` because it only returns tuples and does not evaluate them. ### Query for all users with all relationships for a particular document To query for all users that have any relationship with `document:2021-budget`, call read API with body of  ```json {   \"tuple_key\": {       \"object\": \"document:2021-budget\"    } } ``` The API will return something like  ```json {   \"tuples\": [     {       \"key\": {         \"user\": \"user:anne\",         \"relation\": \"writer\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-05T13:42:12.356Z\"     },     {       \"key\": {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       },       \"timestamp\": \"2021-10-06T15:32:11.128Z\"     }   ],   \"continuation_token\": \"eyJwayI6IkxBVEVTVF9OU0NPTkZJR19hdXRoMHN0b3JlIiwic2siOiIxem1qbXF3MWZLZExTcUoyN01MdTdqTjh0cWgifQ==\" } ``` This means that `document:2021-budget` has 1 `reader` (`user:bob`) and 1 `writer` (`user:anne`). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**ReadRequest**](ReadRequest.md) |  | [required] |

### Return type

[**crate::models::ReadResponse**](ReadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_changes

> crate::models::ReadChangesResponse read_changes(store_id, r#type, page_size, continuation_token)
Return a list of all the tuple changes

The ReadChanges API will return a paginated list of tuple changes (additions and deletions) that occurred in a given store, sorted by ascending time. The response will include a continuation token that is used to get the next set of changes. If there are no changes after the provided continuation token, the same token will be returned in order for it to be used when new changes are recorded. If the store never had any tuples added or removed, this token will be empty. You can use the `type` parameter to only get the list of tuple changes that affect objects of that type. When reading a write tuple change, if it was conditioned, the condition will be returned. When reading a delete tuple change, the condition will NOT be returned regardless of whether it was originally conditioned or not. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |
**page_size** | Option<**i32**> |  |  |
**continuation_token** | Option<**String**> |  |  |

### Return type

[**crate::models::ReadChangesResponse**](ReadChangesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write

> serde_json::Value write(store_id, body)
Add or delete tuples from the store

The Write API will update the tuples for a certain store. Tuples and type definitions allow OpenFGA to determine whether a relationship exists between an object and an user. In the body, `writes` adds new tuples and `deletes` removes existing tuples. When deleting a tuple, any `condition` specified with it is ignored. The API is not idempotent: if, later on, you try to add the same tuple key (even if the `condition` is different), or if you try to delete a non-existing tuple, it will throw an error. An `authorization_model_id` may be specified in the body. If it is, it will be used to assert that each written tuple (not deleted) is valid for the model specified. If it is not specified, the latest authorization model ID will be used. ## Example ### Adding relationships To add `user:anne` as a `writer` for `document:2021-budget`, call write API with the following  ```json {   \"writes\": {     \"tuple_keys\": [       {         \"user\": \"user:anne\",         \"relation\": \"writer\",         \"object\": \"document:2021-budget\"       }     ]   },   \"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\" } ``` ### Removing relationships To remove `user:bob` as a `reader` for `document:2021-budget`, call write API with the following  ```json {   \"deletes\": {     \"tuple_keys\": [       {         \"user\": \"user:bob\",         \"relation\": \"reader\",         \"object\": \"document:2021-budget\"       }     ]   } } ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**WriteRequest**](WriteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

