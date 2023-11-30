# FeatureSpecification

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the feature specification. Must be locally unique within the containing specification, thus allowing direct access to the feature spec. | [optional] [default to None]
**href** | **String** | Hyperlink reference | [optional] [default to None]
**is_bundle** | **bool** | A flag indicating if this is a feature group (true) or not (false) | [optional] [default to None]
**is_enabled** | **bool** | A flag indicating if the feature is enabled (true) or not (false) | [optional] [default to None]
**name** | **String** | Unique name given to the feature specification | [optional] [default to None]
**version** | **String** | Version of the feature specification | [optional] [default to None]
**constraint** | [**Vec<models::ConstraintRef>**](ConstraintRef.md) | This is a list of feature constraints | [optional] [default to None]
**feature_spec_characteristic** | [**Vec<models::FeatureSpecificationCharacteristic>**](FeatureSpecificationCharacteristic.md) | This is a list of characteristics for a particular feature | [optional] [default to None]
**feature_spec_relationship** | [**Vec<models::FeatureSpecificationRelationship>**](FeatureSpecificationRelationship.md) | A dependency, exclusivity or aggratation relationship between/among feature specifications. | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


