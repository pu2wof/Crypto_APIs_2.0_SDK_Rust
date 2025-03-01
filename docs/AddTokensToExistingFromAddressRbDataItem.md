# AddTokensToExistingFromAddressRbDataItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**callback_secret_key** | Option<**String**> | Represents the Secret Key value provided by the customer. This field is used for security purposes during the callback notification, in order to prove the sender of the callback as Crypto APIs. For more information please see our [Documentation](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-security). | [optional]
**callback_url** | **String** | Represents the URL that is set by the customer where the callback will be received at. The callback notification will be received only if and when the event occurs. | 
**confirmations_count** | **i32** | Represents the number of confirmations, i.e. the amount of blocks that have been built on top of this block. | 
**fee_priority** | **String** | Represents the fee priority of the automation, whether it is \"SLOW\", \"STANDARD\" or \"FAST\". | 
**from_address** | **String** | Represents the hash of the address that forwards the tokens. | 
**minimum_transfer_amount** | **String** | Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding. | 
**to_address** | **String** | Represents the hash of the address the currency is forwarded to. | 
**token_data** | [**crate::models::AddTokensToExistingFromAddressRbTokenData**](AddTokensToExistingFromAddressRBTokenData.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


