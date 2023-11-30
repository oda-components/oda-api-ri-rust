# ConnectionSpecification

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for graph edge specification. | [optional] [default to None]
**href** | **String** | Hyperlink reference | [optional] [default to None]
**association_type** | **String** | Association type. | 
**name** | **String** | Descriptive name for graph edge specification. | [optional] [default to None]
**endpoint_specification** | [**Vec<models::EndpointSpecificationRef>**](EndpointSpecificationRef.md) | Specifications for resource graph vertices connected by this edge. | 
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


