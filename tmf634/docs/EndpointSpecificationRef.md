# EndpointSpecificationRef

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | unique identifier | 
**href** | **String** | Hyperlink reference | [optional] [default to None]
**is_root** | **bool** | Directionality: true when endpoint is a source, false when a sink. If true for all endpoints connectivity is bidirectional. Default is true. | [optional] [default to Some(true)]
**name** | **String** | Name of the related entity. | [optional] [default to None]
**role** | **String** | Role of the Resource Function. | [optional] [default to None]
**connection_point_specification** | [***models::ConnectionPointSpecificationRef**](ConnectionPointSpecificationRef.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]
**at_referred_type** | **String** | The actual type of the target instance when needed for disambiguation. | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


