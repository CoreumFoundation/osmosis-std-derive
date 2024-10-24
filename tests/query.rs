use coreum_std_derive::CosmwasmExt;
use cosmwasm_std::AnyMsg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
#[repr(i32)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Feature {
    Minting = 0,
    Burning = 1,
    Freezing = 2,
    Whitelisting = 3,
    Ibc = 4,
    BlockSmartContracts = 5,
    Clawback = 6,
    Extension = 7,
    DexBlock = 8,
    DexWhitelistedDenoms = 9,
    DexOrderCancellation = 10,
    DexUnifiedRefAmountChange = 11,
}
impl Feature {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Feature::Minting => "minting",
            Feature::Burning => "burning",
            Feature::Freezing => "freezing",
            Feature::Whitelisting => "whitelisting",
            Feature::Ibc => "ibc",
            Feature::BlockSmartContracts => "block_smart_contracts",
            Feature::Clawback => "clawback",
            Feature::Extension => "extension",
            Feature::DexBlock => "dex_block",
            Feature::DexWhitelistedDenoms => "dex_whitelisted_denoms",
            Feature::DexOrderCancellation => "dex_order_cancellation",
            Feature::DexUnifiedRefAmountChange => "dex_unified_ref_amount_change",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "minting" => Some(Self::Minting),
            "burning" => Some(Self::Burning),
            "freezing" => Some(Self::Freezing),
            "whitelisting" => Some(Self::Whitelisting),
            "ibc" => Some(Self::Ibc),
            "block_smart_contracts" => Some(Self::BlockSmartContracts),
            "clawback" => Some(Self::Clawback),
            "extension" => Some(Self::Extension),
            "dex_block" => Some(Self::DexBlock),
            "dex_whitelisted_denoms" => Some(Self::DexWhitelistedDenoms),
            "dex_order_cancellation" => Some(Self::DexOrderCancellation),
            "dex_unified_ref_amount_change" => Some(Self::DexUnifiedRefAmountChange),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, CosmwasmExt)]
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenRequest")]
#[proto_query(
    path = "/coreum.asset.ft.v1.Query/Token",
    response_type = QueryTokenResponse
)]
pub struct QueryTokenRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, CosmwasmExt)]
#[proto_message(type_url = "/coreum.asset.ft.v1.Token")]
pub struct Token {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub subunit: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub precision: u32,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub globally_frozen: bool,
    #[prost(enumeration = "Feature", repeated, tag = "8")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// burn_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// burn_amount. This value will be burnt on top of the send amount.
    #[prost(string, tag = "9")]
    pub burn_rate: ::prost::alloc::string::String,
    /// send_commission_rate is a number between 0 and 1 which will be multiplied by send amount to determine
    /// amount sent to the token admin account.
    #[prost(string, tag = "10")]
    pub send_commission_rate: ::prost::alloc::string::String,
    #[prost(uint32, tag = "11")]
    pub version: u32,
    #[prost(string, tag = "12")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub extension_cw_address: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub admin: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, CosmwasmExt)]
#[proto_message(type_url = "/coreum.asset.ft.v1.QueryTokenResponse")]
pub struct QueryTokenResponse {
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<Token>,
}

fn main() {
    let _: AnyMsg = QueryTokenRequest {
        denom: "ucore".to_string(),
    }
    .to_any();
}
