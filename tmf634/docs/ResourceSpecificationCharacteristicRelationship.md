# ResourceSpecificationCharacteristicRelationship

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**characteristic_specification_id** | **String** | Unique identifier of the characteristic within the specification | [optional] [default to None]
**name** | **String** | Name of the target characteristic within the specification | [optional] [default to None]
**relationship_type** | **String** | Type of relationship such as aggregation, migration, substitution, dependency, exclusivity | [optional] [default to None]
**resource_specification_href** | **String** | Hyperlink reference to the resource specification containing the target characteristic | [optional] [default to None]
**resource_specification_id** | **String** | Unique identifier of the resource specification containing the target characteristic | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


