# GetWalletTransactionDetailsByTransactionIdribsdVin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | **Vec<String>** |  | 
**coinbase** | Option<**String**> | Represents the coinbase hex. | [optional]
**script_sig** | [**crate::models::GetWalletTransactionDetailsByTransactionIdribsdScriptSig**](GetWalletTransactionDetailsByTransactionIDRIBSD_scriptSig.md) |  | 
**sequence** | **i32** | Represents the script sequence number. | 
**txid** | Option<**String**> | Represents the reference transaction identifier. | [optional]
**txinwitness** | Option<**Vec<String>**> |  | [optional]
**value** | **String** | Represents the sent/received amount. | 
**vout** | Option<**i32**> | It refers to the index of the output address of this transaction. The index starts from 0. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


