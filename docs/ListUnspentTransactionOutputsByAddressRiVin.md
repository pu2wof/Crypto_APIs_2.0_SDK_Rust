# ListUnspentTransactionOutputsByAddressRiVin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | **Vec<String>** |  | 
**coinbase** | Option<**String**> | Represents the coinbase hex. | [optional]
**script_sig** | [**crate::models::GetTransactionDetailsByTransactionIdribsbScriptSig**](GetTransactionDetailsByTransactionIDRIBSB_scriptSig.md) |  | 
**sequence** | **String** | Represents the script sequence number. | 
**txid** | **String** | Represents the reference transaction identifier. | 
**txinwitness** | Option<**Vec<String>**> |  | [optional]
**value** | **String** | Represents the sent/received amount. | 
**vout** | Option<**i32**> | Defines the vout of the transaction output, i.e. which output to spend. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


