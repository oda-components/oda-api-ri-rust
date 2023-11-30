# ResourceSpecificationCreate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** | Category of the target resource like NetworkConnectivity, PhysicalLinks, Generic, L2Network and so on. | [optional] [default to None]
**description** | **String** | Description of this REST resource | [optional] [default to None]
**is_bundle** | **bool** | A flag indicates that if this resource specification is a bundled specification (true) or single (false). | [optional] [default to None]
**last_update** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date and time of the last update of this REST resource | [optional] [default to None]
**lifecycle_status** | **String** | Used to indicate the current lifecycle status of the resource specification | [optional] [default to None]
**name** | **String** | Name given to this REST resource | 
**version** | **String** | Resource Specification version | [optional] [default to None]
**attachment** | [**Vec<models::AttachmentRefOrValue>**](AttachmentRefOrValue.md) | Complements the description of an element (for instance a resource) through video, pictures ... | [optional] [default to None]
**feature_specification** | [**Vec<models::FeatureSpecification>**](FeatureSpecification.md) | A list of Features for this specification. | [optional] [default to None]
**related_party** | [**Vec<models::RelatedParty>**](RelatedParty.md) | A related party defines party or party role linked to a specific entity. | [optional] [default to None]
**resource_spec_characteristic** | [**Vec<models::ResourceSpecificationCharacteristic>**](ResourceSpecificationCharacteristic.md) | A characteristic quality or distinctive feature of a ResourceSpecification.  The characteristic can be take on a discrete value, such as color, can take on a range of values, (for example, sensitivity of 100-240 mV), or can be derived from a formula (for example, usage time (hrs) = 30 - talk time *3). Certain characteristics, such as color, may be configured during the ordering or some other process. | [optional] [default to None]
**resource_spec_relationship** | [**Vec<models::ResourceSpecificationRelationship>**](ResourceSpecificationRelationship.md) | A migration, substitution, dependency or exclusivity relationship between/among resource specifications. | [optional] [default to None]
**target_resource_schema** | [***models::TargetResourceSchema**](TargetResourceSchema.md) |  | [optional] [default to None]
**valid_for** | [***models::TimePeriod**](TimePeriod.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


