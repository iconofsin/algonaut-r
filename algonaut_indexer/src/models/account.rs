/*
 * Indexer
 *
 * Algorand ledger analytics API.
 *
 * The version of the OpenAPI document: 2.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Account : Account information at a given round.  Definition: data/basics/userBalance.go : AccountData

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Account {
    /// the account public key
    #[serde(rename = "address")]
    pub address: String,
    /// \\[algo\\] total number of MicroAlgos in the account
    #[serde(rename = "amount")]
    pub amount: u64,
    /// specifies the amount of MicroAlgos in the account, without the pending rewards.
    #[serde(rename = "amount-without-pending-rewards")]
    pub amount_without_pending_rewards: u64,
    /// \\[appl\\] applications local data stored in this account.  Note the raw object uses `map[int] -> AppLocalState` for this type.
    #[serde(rename = "apps-local-state", skip_serializing_if = "Option::is_none")]
    pub apps_local_state: Option<Vec<crate::models::ApplicationLocalState>>,
    /// \\[teap\\] the sum of all extra application program pages for this account.
    #[serde(
        rename = "apps-total-extra-pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub apps_total_extra_pages: Option<u64>,
    #[serde(rename = "apps-total-schema", skip_serializing_if = "Option::is_none")]
    pub apps_total_schema: Option<Box<crate::models::ApplicationStateSchema>>,
    /// \\[asset\\] assets held by this account.  Note the raw object uses `map[int] -> AssetHolding` for this type.
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<crate::models::AssetHolding>>,
    /// \\[spend\\] the address against which signing should be checked. If empty, the address of the current account is used. This field can be updated in any transaction by setting the RekeyTo field.
    #[serde(rename = "auth-addr", skip_serializing_if = "Option::is_none")]
    pub auth_addr: Option<String>,
    /// Round during which this account was most recently closed.
    #[serde(rename = "closed-at-round", skip_serializing_if = "Option::is_none")]
    pub closed_at_round: Option<u64>,
    /// \\[appp\\] parameters of applications created by this account including app global data.  Note: the raw account uses `map[int] -> AppParams` for this type.
    #[serde(rename = "created-apps", skip_serializing_if = "Option::is_none")]
    pub created_apps: Option<Vec<crate::models::Application>>,
    /// \\[apar\\] parameters of assets created by this account.  Note: the raw account uses `map[int] -> Asset` for this type.
    #[serde(rename = "created-assets", skip_serializing_if = "Option::is_none")]
    pub created_assets: Option<Vec<crate::models::Asset>>,
    /// Round during which this account first appeared in a transaction.
    #[serde(rename = "created-at-round", skip_serializing_if = "Option::is_none")]
    pub created_at_round: Option<u64>,
    /// Whether or not this account is currently closed.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "participation", skip_serializing_if = "Option::is_none")]
    pub participation: Option<Box<crate::models::AccountParticipation>>,
    /// amount of MicroAlgos of pending rewards in this account.
    #[serde(rename = "pending-rewards")]
    pub pending_rewards: u64,
    /// \\[ebase\\] used as part of the rewards computation. Only applicable to accounts which are participating.
    #[serde(rename = "reward-base", skip_serializing_if = "Option::is_none")]
    pub reward_base: Option<u64>,
    /// \\[ern\\] total rewards of MicroAlgos the account has received, including pending rewards.
    #[serde(rename = "rewards")]
    pub rewards: u64,
    /// The round for which this information is relevant.
    #[serde(rename = "round")]
    pub round: u64,
    /// Indicates what type of signature is used by this account, must be one of: * sig * msig * lsig * or null if unknown
    #[serde(rename = "sig-type", skip_serializing_if = "Option::is_none")]
    pub sig_type: Option<SigType>,
    /// \\[onl\\] delegation status of the account's MicroAlgos * Offline - indicates that the associated account is delegated. *  Online  - indicates that the associated account used as part of the delegation pool. *   NotParticipating - indicates that the associated account is neither a delegator nor a delegate.
    #[serde(rename = "status")]
    pub status: String,
    /// The count of all applications that have been opted in, equivalent to the count of application local data (AppLocalState objects) stored in this account.
    #[serde(rename = "total-apps-opted-in")]
    pub total_apps_opted_in: u64,
    /// The count of all assets that have been opted in, equivalent to the count of AssetHolding objects held by this account.
    #[serde(rename = "total-assets-opted-in")]
    pub total_assets_opted_in: u64,
    /// For app-accounts only. The total number of bytes allocated for the keys and values of boxes which belong to the associated application.
    #[serde(rename = "total-box-bytes")]
    pub total_box_bytes: u64,
    /// For app-accounts only. The total number of boxes which belong to the associated application.
    #[serde(rename = "total-boxes")]
    pub total_boxes: u64,
    /// The count of all apps (AppParams objects) created by this account.
    #[serde(rename = "total-created-apps")]
    pub total_created_apps: u64,
    /// The count of all assets (AssetParams objects) created by this account.
    #[serde(rename = "total-created-assets")]
    pub total_created_assets: u64,
}

impl Account {
    /// Account information at a given round.  Definition: data/basics/userBalance.go : AccountData
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        address: String,
        amount: u64,
        amount_without_pending_rewards: u64,
        pending_rewards: u64,
        rewards: u64,
        round: u64,
        status: String,
        total_apps_opted_in: u64,
        total_assets_opted_in: u64,
        total_box_bytes: u64,
        total_boxes: u64,
        total_created_apps: u64,
        total_created_assets: u64,
    ) -> Account {
        Account {
            address,
            amount,
            amount_without_pending_rewards,
            apps_local_state: None,
            apps_total_extra_pages: None,
            apps_total_schema: None,
            assets: None,
            auth_addr: None,
            closed_at_round: None,
            created_apps: None,
            created_assets: None,
            created_at_round: None,
            deleted: None,
            participation: None,
            pending_rewards,
            reward_base: None,
            rewards,
            round,
            sig_type: None,
            status,
            total_apps_opted_in,
            total_assets_opted_in,
            total_box_bytes,
            total_boxes,
            total_created_apps,
            total_created_assets,
        }
    }
}

/// Indicates what type of signature is used by this account, must be one of: * sig * msig * lsig * or null if unknown
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SigType {
    #[serde(rename = "sig")]
    Sig,
    #[serde(rename = "msig")]
    Msig,
    #[serde(rename = "lsig")]
    Lsig,
}

impl Default for SigType {
    fn default() -> SigType {
        Self::Sig
    }
}