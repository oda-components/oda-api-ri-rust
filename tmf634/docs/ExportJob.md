# ExportJob

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Identifier of the export job | [optional] [default to None]
**href** | **String** | Reference of the export job | [optional] [default to None]
**completion_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Data at which the job was completed | [optional] [default to None]
**content_type** | **String** | The format of the exported data | [optional] [default to None]
**creation_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date at which the job was created | [optional] [default to None]
**error_log** | **String** | Reason for failure | [optional] [default to None]
**path** | **String** | URL of the root resource acting as the source for streaming content to the file specified by the export job | [optional] [default to None]
**query** | **String** | Used to scope the exported data | [optional] [default to None]
**url** | **String** | URL of the file containing the data to be exported | [optional] [default to None]
**status** | [***models::JobStateType**](JobStateType.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


