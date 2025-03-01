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
pub struct ListTokensTransfersByTransactionHashRi {
    /// Represents the contract address of the token, which controls its logic. It is not the address that holds the tokens.
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    /// Defines the block height in which this transaction was confirmed/mined.
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Defines the address to which the recipient receives the transferred tokens.
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    /// Defines the address from which the sender transfers tokens.
    #[serde(rename = "senderAddress")]
    pub sender_address: String,
    /// Defines the decimals of the token, i.e. the number of digits that come after the decimal coma of the token.
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: i32,
    /// Defines the token's name as a string.
    #[serde(rename = "tokenName")]
    pub token_name: String,
    /// Defines the token symbol by which the token contract is known. It is usually 3-4 characters in length.
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    /// Defines the specific token type.
    #[serde(rename = "tokenType")]
    pub token_type: String,
    /// Defines the token amount of the transfer.
    #[serde(rename = "tokensAmount")]
    pub tokens_amount: String,
    /// Represents the hash of the transaction, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    /// Defines the specific time/date when the transaction was created in Unix Timestamp.
    #[serde(rename = "transactionTimestamp")]
    pub transaction_timestamp: i32,
}

impl ListTokensTransfersByTransactionHashRi {
    pub fn new(contract_address: String, mined_in_block_height: i32, recipient_address: String, sender_address: String, token_decimals: i32, token_name: String, token_symbol: String, token_type: String, tokens_amount: String, transaction_hash: String, transaction_timestamp: i32) -> ListTokensTransfersByTransactionHashRi {
        ListTokensTransfersByTransactionHashRi {
            contract_address,
            mined_in_block_height,
            recipient_address,
            sender_address,
            token_decimals,
            token_name,
            token_symbol,
            token_type,
            tokens_amount,
            transaction_hash,
            transaction_timestamp,
        }
    }
}


