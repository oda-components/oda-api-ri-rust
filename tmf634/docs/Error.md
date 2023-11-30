# Error

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | Application relevant detail, defined in the API or a common list. | 
**reason** | **String** | Explanation of the reason for the error which can be shown to a client user. | 
**message** | **String** | More details and corrective actions related to the error which can be shown to a client user. | [optional] [default to None]
**status** | **String** | HTTP Error code extension | [optional] [default to None]
**reference_error** | **String** | URI of documentation describing the error. | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class. | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class entity name. | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


