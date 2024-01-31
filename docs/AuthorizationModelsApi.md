# \AuthorizationModelsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_authorization_model**](AuthorizationModelsApi.md#read_authorization_model) | **GET** /stores/{store_id}/authorization-models/{id} | Return a particular version of an authorization model
[**read_authorization_models**](AuthorizationModelsApi.md#read_authorization_models) | **GET** /stores/{store_id}/authorization-models | Return all the authorization models for a particular store
[**write_authorization_model**](AuthorizationModelsApi.md#write_authorization_model) | **POST** /stores/{store_id}/authorization-models | Create a new authorization model



## read_authorization_model

> crate::models::ReadAuthorizationModelResponse read_authorization_model(store_id, id)
Return a particular version of an authorization model

The ReadAuthorizationModel API returns an authorization model by its identifier. The response will return the authorization model for the particular version.  ## Example To retrieve the authorization model with ID `01G5JAVJ41T49E9TT3SKVS7X1J` for the store, call the GET authorization-models by ID API with `01G5JAVJ41T49E9TT3SKVS7X1J` as the `id` path parameter.  The API will return: ```json {   \"authorization_model\":{     \"id\":\"01G5JAVJ41T49E9TT3SKVS7X1J\",     \"type_definitions\":[       {         \"type\":\"user\"       },       {         \"type\":\"document\",         \"relations\":{           \"reader\":{             \"union\":{               \"child\":[                 {                   \"this\":{}                 },                 {                   \"computedUserset\":{                     \"object\":\"\",                     \"relation\":\"writer\"                   }                 }               ]             }           },           \"writer\":{             \"this\":{}           }         }       }     ]   } } ``` In the above example, there are 2 types (`user` and `document`). The `document` type has 2 relations (`writer` and `reader`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::ReadAuthorizationModelResponse**](ReadAuthorizationModelResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_authorization_models

> crate::models::ReadAuthorizationModelsResponse read_authorization_models(store_id, page_size, continuation_token)
Return all the authorization models for a particular store

The ReadAuthorizationModels API will return all the authorization models for a certain store. OpenFGA's response will contain an array of all authorization models, sorted in descending order of creation.  ## Example Assume that a store's authorization model has been configured twice. To get all the authorization models that have been created in this store, call GET authorization-models. The API will return a response that looks like: ```json {   \"authorization_models\": [     {       \"id\": \"01G50QVV17PECNVAHX1GG4Y5NC\",       \"type_definitions\": [...]     },     {       \"id\": \"01G4ZW8F4A07AKQ8RHSVG9RW04\",       \"type_definitions\": [...]     },   ],   \"continuation_token\": \"eyJwayI6IkxBVEVTVF9OU0NPTkZJR19hdXRoMHN0b3JlIiwic2siOiIxem1qbXF3MWZLZExTcUoyN01MdTdqTjh0cWgifQ==\" } ``` If there are no more authorization models available, the `continuation_token` field will be empty ```json {   \"authorization_models\": [     {       \"id\": \"01G50QVV17PECNVAHX1GG4Y5NC\",       \"type_definitions\": [...]     },     {       \"id\": \"01G4ZW8F4A07AKQ8RHSVG9RW04\",       \"type_definitions\": [...]     },   ],   \"continuation_token\": \"\" } ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**page_size** | Option<**i32**> |  |  |
**continuation_token** | Option<**String**> |  |  |

### Return type

[**crate::models::ReadAuthorizationModelsResponse**](ReadAuthorizationModelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_authorization_model

> crate::models::WriteAuthorizationModelResponse write_authorization_model(store_id, body)
Create a new authorization model

The WriteAuthorizationModel API will add a new authorization model to a store. Each item in the `type_definitions` array is a type definition as specified in the field `type_definition`. The response will return the authorization model's ID in the `id` field.  ## Example To add an authorization model with `user` and `document` type definitions, call POST authorization-models API with the body:  ```json {   \"type_definitions\":[     {       \"type\":\"user\"     },     {       \"type\":\"document\",       \"relations\":{         \"reader\":{           \"union\":{             \"child\":[               {                 \"this\":{}               },               {                 \"computedUserset\":{                   \"object\":\"\",                   \"relation\":\"writer\"                 }               }             ]           }         },         \"writer\":{           \"this\":{}         }       }     }   ] } ``` OpenFGA's response will include the version id for this authorization model, which will look like  ``` {\"authorization_model_id\": \"01G50QVV17PECNVAHX1GG4Y5NC\"} ``` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** |  | [required] |
**body** | [**WriteAuthorizationModelRequest**](WriteAuthorizationModelRequest.md) |  | [required] |

### Return type

[**crate::models::WriteAuthorizationModelResponse**](WriteAuthorizationModelResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

