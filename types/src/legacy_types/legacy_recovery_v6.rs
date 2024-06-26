//! recovery

use crate::exports::AuthenticationKey;

use crate::legacy_types::burn::{BurnCounterResource, UserBurnPreferenceResource};
use crate::legacy_types::donor_voice::RegistryResource;
use crate::legacy_types::donor_voice_txs::TxScheduleResource;
use crate::legacy_types::fee_maker::{EpochFeeMakerRegistryResource, FeeMakerResource};
use crate::legacy_types::jail::JailResource;
use crate::legacy_types::match_index::MatchIndexResource;
use crate::legacy_types::pledge_account::MyPledgesResource;
use crate::legacy_types::validator_universe::ValidatorUniverseResource;
use crate::legacy_types::vouch::MyVouchesResource;
use crate::legacy_types::{
    ancestry_legacy::LegacyAncestryResource,
    cumulative_deposits::{CumulativeDepositResource, LegacyBalanceResource},
    legacy_currency_info::CurrencyInfoResource,
    receipts::ReceiptsResource,
    wallet::{CommunityWalletsResourceLegacy, SlowWalletListResource, SlowWalletResource},
};
use anyhow::anyhow;
use diem_types::account_state::AccountState;
use diem_types::account_view::AccountView;
use diem_types::validator_config::{ValidatorConfig, ValidatorOperatorConfigResource};
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{fs, path::PathBuf};

use super::ol_account::BurnTrackerResource;
use super::proof_of_fee::ConsensusRewardResource;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Account role
pub enum AccountRole {
    /// System Accounts
    System,
    /// Vals
    Validator,
    /// Opers
    Operator,
    /// Users
    EndUser,

    /// Dropped
    Drop,
}

impl Default for AccountRole {
    fn default() -> Self {
        Self::EndUser
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LegacyRecoveryV6 {
    ///
    pub account: Option<AccountAddress>,
    ///
    pub auth_key: Option<AuthenticationKey>,
    ///
    pub role: AccountRole,
    ///
    pub balance: Option<LegacyBalanceResource>,
    ///
    pub val_cfg: Option<ValidatorConfig>,
    ///
    pub val_operator_cfg: Option<ValidatorOperatorConfigResource>,
    ///
    pub comm_wallet: Option<CommunityWalletsResourceLegacy>,
    ///
    pub currency_info: Option<CurrencyInfoResource>,
    ///
    pub ancestry: Option<LegacyAncestryResource>,
    ///
    pub receipts: Option<ReceiptsResource>,
    ///
    pub cumulative_deposits: Option<CumulativeDepositResource>,
    ///
    pub slow_wallet: Option<SlowWalletResource>,
    ///
    pub slow_wallet_list: Option<SlowWalletListResource>,
    ///
    pub user_burn_preference: Option<UserBurnPreferenceResource>,
    ///
    pub burn_tracker: Option<BurnTrackerResource>,
    ///
    pub my_vouches: Option<MyVouchesResource>,
    ///
    pub tx_schedule: Option<TxScheduleResource>,
    ///
    pub fee_maker: Option<FeeMakerResource>,
    ///
    pub jail: Option<JailResource>,
    ///
    pub my_pledge: Option<MyPledgesResource>,
    ///
    pub burn_counter: Option<BurnCounterResource>,
    ///
    pub donor_voice_registry: Option<RegistryResource>,
    ///
    pub epoch_fee_maker_registry: Option<EpochFeeMakerRegistryResource>,
    ///
    pub match_index: Option<MatchIndexResource>,
    ///
    pub validator_universe: Option<ValidatorUniverseResource>,
    ///
    pub consensus_reward: Option<ConsensusRewardResource>,
}

pub fn strip_system_address(list: &mut Vec<LegacyRecoveryV6>) {
    list.retain(|e| {
        !e.account
            .unwrap()
            .to_string()
            .contains("000000000000000000000000000000000000000000000000000000000000000")
    })
}

/// Read from genesis recovery file
pub fn read_from_recovery_file(path: &PathBuf) -> Vec<LegacyRecoveryV6> {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse")
}

pub fn get_legacy_recovery(account_state: &AccountState) -> anyhow::Result<LegacyRecoveryV6> {
    let mut legacy_recovery = LegacyRecoveryV6 {
        account: account_state.get_account_address()?,
        auth_key: None,
        role: AccountRole::EndUser,
        balance: None,
        val_cfg: None,
        val_operator_cfg: None,
        comm_wallet: None,
        currency_info: None, // TODO: DO WE NEED THIS
        ancestry: None,
        receipts: None,
        cumulative_deposits: None,
        slow_wallet: None,
        slow_wallet_list: None,
        user_burn_preference: None,
        burn_tracker: None,
        my_vouches: None,
        tx_schedule: None,
        fee_maker: None,
        jail: None,
        my_pledge: None,
        burn_counter: None,
        donor_voice_registry: None,
        epoch_fee_maker_registry: None,
        match_index: None,
        validator_universe: None,
        consensus_reward: None,
    };
    let account_resource = account_state.get_account_resource()?;

    if let Some(account_resource) = account_resource {
        let byte_slice: [u8; 32] = account_resource
            .authentication_key()
            .to_vec()
            .try_into()
            .map_err(|err| anyhow!("error: {:?}", err))?;

        if account_state.get_account_address()? == Some(AccountAddress::from_str("0x1")?) {
            legacy_recovery.role = AccountRole::System;
        }

        // auth key
        legacy_recovery.auth_key = Some(AuthenticationKey::new(byte_slice));

        // balance
        legacy_recovery.balance = account_state
            .get_coin_store_resource()?
            .map(|r| LegacyBalanceResource { coin: r.coin() });

        // validator config
        legacy_recovery.val_cfg = account_state.get_validator_config_resource()?;
        if legacy_recovery.val_cfg.is_some() {
            legacy_recovery.role = AccountRole::Validator;
        }

        // validator operator config
        legacy_recovery.val_operator_cfg =
            account_state.get_validator_operator_config_resource()?;
        if legacy_recovery.val_operator_cfg.is_some() {
            legacy_recovery.role = AccountRole::Operator;
        }

        // comm_wallet
        legacy_recovery.comm_wallet =
            account_state.get_move_resource::<CommunityWalletsResourceLegacy>()?;

        // ancestry
        legacy_recovery.ancestry = account_state.get_move_resource::<LegacyAncestryResource>()?;

        // receipts
        legacy_recovery.receipts = account_state.get_move_resource::<ReceiptsResource>()?;

        // cumulative_deposits
        legacy_recovery.cumulative_deposits =
            account_state.get_move_resource::<CumulativeDepositResource>()?;

        // slow wallet
        legacy_recovery.slow_wallet = account_state.get_move_resource::<SlowWalletResource>()?;

        // slow wallet list
        legacy_recovery.slow_wallet_list =
            account_state.get_move_resource::<SlowWalletListResource>()?;

        // user burn preference
        legacy_recovery.user_burn_preference =
            account_state.get_move_resource::<UserBurnPreferenceResource>()?;

        // burn tracker
        legacy_recovery.burn_tracker = account_state.get_move_resource::<BurnTrackerResource>()?;

        // my vouches
        legacy_recovery.my_vouches = account_state.get_move_resource::<MyVouchesResource>()?;

        // tx schedule
        legacy_recovery.tx_schedule = account_state.get_move_resource::<TxScheduleResource>()?;

        // fee maker
        legacy_recovery.fee_maker = account_state.get_move_resource::<FeeMakerResource>()?;

        // jail
        legacy_recovery.jail = account_state.get_move_resource::<JailResource>()?;

        // my pledge
        legacy_recovery.my_pledge = account_state.get_move_resource::<MyPledgesResource>()?;

        // burn counter
        legacy_recovery.burn_counter = account_state.get_move_resource::<BurnCounterResource>()?;

        // donor voice registry
        legacy_recovery.donor_voice_registry =
            account_state.get_move_resource::<RegistryResource>()?;

        // epoch fee maker registry
        legacy_recovery.epoch_fee_maker_registry =
            account_state.get_move_resource::<EpochFeeMakerRegistryResource>()?;

        // match index
        legacy_recovery.match_index = account_state.get_move_resource::<MatchIndexResource>()?;

        // validator universe
        legacy_recovery.validator_universe =
            account_state.get_move_resource::<ValidatorUniverseResource>()?;

        // consensus reward
        legacy_recovery.consensus_reward =
            account_state.get_move_resource::<ConsensusRewardResource>()?;
    }

    Ok(legacy_recovery)
}
