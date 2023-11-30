# ResourceCategory

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the category | [optional] [default to None]
**href** | **String** | Hyperlink reference to the category | [optional] [default to None]
**description** | **String** | Description of the category | [optional] [default to None]
**is_root** | **bool** | If true, this Boolean indicates that the category is a root of categories | [optional] [default to None]
**last_update** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date and time of the last update | [optional] [default to None]
**lifecycle_status** | **String** | Used to indicate the current lifecycle status | [optional] [default to None]
**name** | **String** | Name of the category | [optional] [default to None]
**parent_id** | **String** | Unique identifier of the parent category | [optional] [default to None]
**version** | **String** | Category version | [optional] [default to None]
**category** | [**Vec<models::ResourceCategoryRef>**](ResourceCategoryRef.md) | The category resource is used to group product offerings, service and resource candidates in logical containers. Categories can contain other (sub-)categories and/or product offerings. | [optional] [default to None]
**related_party** | [**Vec<models::RelatedParty>**](RelatedParty.md) | List of parties involved in this category | [optional] [default to None]
**resource_candidate** | [**Vec<models::ResourceCandidateRef>**](ResourceCandidateRef.md) | List of resource candidates accessible via this category | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


