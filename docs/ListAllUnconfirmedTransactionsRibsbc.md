# ListAllUnconfirmedTransactionsRibsbc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**locktime** | **i32** | Represents the locktime on the transaction on the specific blockchain, i.e. the blockheight at which the transaction is valid. | 
**size** | **i32** | Represents the total size of this transaction. | 
**version** | **i32** | Represents the transaction's version number. | 
**vin** | [**Vec<crate::models::ListAllUnconfirmedTransactionsRibsbcVin>**](ListAllUnconfirmedTransactionsRIBSBC_vin.md) | Represents the transaction inputs. | 
**vout** | [**Vec<crate::models::ListAllUnconfirmedTransactionsRibsbcVout>**](ListAllUnconfirmedTransactionsRIBSBC_vout.md) | Object Array representation of transaction outputs | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


