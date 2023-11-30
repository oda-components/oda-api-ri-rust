# ResourceCatalogUpdate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | Description of this catalog | [optional] [default to None]
**last_update** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date and time of the last update | [optional] [default to None]
**lifecycle_status** | **String** | Used to indicate the current lifecycle status | [optional] [default to None]
**name** | **String** | Name of the catalog | [optional] [default to None]
**version** | **String** | Catalog version | [optional] [default to None]
**category** | [**Vec<models::ResourceCategoryRef>**](ResourceCategoryRef.md) | List of root categories contained in this catalog | [optional] [default to None]
**related_party** | [**Vec<models::RelatedParty>**](RelatedParty.md) | List of parties involved in this catalog | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


