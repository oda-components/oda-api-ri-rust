# CharacteristicSpecificationBase

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique ID for the characteristic | [optional] [default to None]
**configurable** | **bool** | If true, the Boolean indicates that the target Characteristic is configurable | [optional] [default to None]
**description** | **String** | A narrative that explains the CharacteristicSpecification. | [optional] [default to None]
**extensible** | **bool** | An indicator that specifies that the values for the characteristic can be extended by adding new values when instantiating a characteristic for a resource. | [optional] [default to None]
**is_unique** | **bool** | An indicator that specifies if a value is unique for the specification. Possible values are; \"unique while value is in effect\" and \"unique whether value is in effect or not\" | [optional] [default to None]
**max_cardinality** | **i32** | The maximum number of instances a CharacteristicValue can take on. For example, zero to five phone numbers in a group calling plan, where five is the value for the maxCardinality. | [optional] [default to None]
**min_cardinality** | **i32** | The minimum number of instances a CharacteristicValue can take on. For example, zero to five phone numbers in a group calling plan, where zero is the value for the minCardinality. | [optional] [default to None]
**name** | **String** | A word, term, or phrase by which this characteristic specification is known and distinguished from other characteristic specifications. | [optional] [default to None]
**regex** | **String** | A rule or principle represented in regular expression used to derive the value of a characteristic value. | [optional] [default to None]
**value_type** | **String** | A kind of value that the characteristic can take on, such as numeric, text and so forth | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]
**at_value_schema_location** | **String** | This (optional) field provides a link to the schema describing the value type. | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


