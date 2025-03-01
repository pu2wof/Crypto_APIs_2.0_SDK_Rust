/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListTokensTransfersByTransactionHashRData {
    /// The starting index of the response items, i.e. where the response should start listing the returned items.
    #[serde(rename = "offset")]
    pub offset: i32,
    /// Defines how many items should be returned in the response per page basis.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Defines the total number of items returned in the response.
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "items")]
    pub items: Vec<crate::models::ListTokensTransfersByTransactionHashRi>,
}

impl ListTokensTransfersByTransactionHashRData {
    pub fn new(offset: i32, limit: i32, total: i32, items: Vec<crate::models::ListTokensTransfersByTransactionHashRi>) -> ListTokensTransfersByTransactionHashRData {
        ListTokensTransfersByTransactionHashRData {
            offset,
            limit,
            total,
            items,
        }
    }
}


