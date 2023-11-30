# CharacteristicValueSpecification

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_default** | **bool** | If true, the Boolean Indicates if the value is the default value for a characteristic | [optional] [default to None]
**range_interval** | **String** | An indicator that specifies the inclusion or exclusion of the valueFrom and valueTo attributes. If applicable, possible values are \"open\", \"closed\", \"closedBottom\" and \"closedTop\". | [optional] [default to None]
**regex** | **String** | A regular expression constraint for given value | [optional] [default to None]
**unit_of_measure** | **String** | A length, surface, volume, dry measure, liquid measure, money, weight, time, and the like. In general, a determinate quantity or magnitude of the kind designated, taken as a standard of comparison for others of the same kind, in assigning to them numerical values, as 1 foot, 1 yard, 1 mile, 1 square foot. | [optional] [default to None]
**value_from** | **i32** | The low range value that a characteristic can take on | [optional] [default to None]
**value_to** | **i32** | The upper range value that a characteristic can take on | [optional] [default to None]
**value_type** | **String** | A kind of value that the characteristic value can take on, such as numeric, text and so forth | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**value** | [***serde_json::Value**](.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


