# ResourceSpecificationRelationship

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of target ResourceSpecification | [optional] [default to None]
**href** | **String** | Reference of the target ResourceSpecification | [optional] [default to None]
**default_quantity** | **i32** | The default number of the related resource that should be instantiated, for example a rack would typically have 4 cards, although it could support more. | [optional] [default to None]
**maximum_quantity** | **i32** | The maximum number of the related resource that should be instantiated, for example a rack supports a maximum of 16 cards | [optional] [default to None]
**minimum_quantity** | **i32** | The minimum number of the related resource that should be instantiated, for example a rack must have at least 1 card | [optional] [default to None]
**name** | **String** | The name given to the target resource specification instance | [optional] [default to None]
**relationship_type** | **String** | Type of relationship such as migration, substitution, dependency, exclusivity | [optional] [default to None]
**role** | **String** | The association role for this resource specification | [optional] [default to None]
**characteristic** | [**Vec<models::ResourceSpecificationCharacteristic>**](ResourceSpecificationCharacteristic.md) | A characteristic that refines the relationship. For example, consider the relationship between a slot and a card. For a half-height card it is important to know the position at which the card is inserted, so a characteristic Position might be defined on the relationship to allow capturing of this in the inventory | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


