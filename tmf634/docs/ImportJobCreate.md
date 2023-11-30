# ImportJobCreate

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date at which the job was completed | [optional] [default to None]
**content_type** | **String** | Indicates the format of the imported data | [optional] [default to None]
**creation_date** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Date at which the job was created | [optional] [default to None]
**error_log** | **String** | Reason for failure if status is failed | [optional] [default to None]
**path** | **String** | URL of the root resource where the content of the file specified by the import job must be applied | [optional] [default to None]
**url** | **String** | URL of the file containing the data to be imported | 
**status** | [***models::JobStateType**](JobStateType.md) |  | [optional] [default to None]
**at_base_type** | **String** | When sub-classing, this defines the super-class | [optional] [default to None]
**at_schema_location** | **String** | A URI to a JSON-Schema file that defines additional attributes and relationships | [optional] [default to None]
**at_type** | **String** | When sub-classing, this defines the sub-class Extensible name | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


