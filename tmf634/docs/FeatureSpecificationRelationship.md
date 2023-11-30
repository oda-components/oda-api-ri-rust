# FeatureSpecificationRelationship

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | unique identifier | [optional] [default to None]
**href** | **String** | Hyperlink reference | [optional] [default to None]
**feature_id** | **String** | Unique identifier of the target feature specification. | [optional] [default to None]
**name** | **String** | This is the name of the target feature specification. | 
**parent_specification_href** | **String** | Hyperlink reference to the parent specification containing the target feature | [optional] [default to None]
**parent_specification_id** | **String** | Unique identifier of the parent specification containing the target feature | [optional] [default to None]
**relationship_type** | **String** | This is the type of the feature specification relationship. | 
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


