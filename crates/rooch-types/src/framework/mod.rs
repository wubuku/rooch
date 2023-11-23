// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::framework::transaction_validator::TransactionValidator;
use moveos_types::transaction::FunctionCall;

pub mod account_authentication;
pub mod account_coin_store;
/// Types mapping from Framework Move types to Rust types
/// Module binding for the Framework
//TODO there modules maybe generated by ABI in the future
pub mod address_mapping;
pub mod auth_validator;
pub mod bitcoin_light_client;
pub mod coin;
pub mod coin_store;
pub mod empty;
pub mod ethereum_address;
pub mod ethereum_light_client;
pub mod ethereum_validator;
pub mod gas_coin;
pub mod genesis;
pub mod native_validator;
pub mod session_key;
pub mod timestamp;
pub mod transaction_validator;
pub mod transfer;

/// MoveOS system pre_execute functions registry.
/// The registry is used to filter out system pre_execute functions.
/// All system pre_execute functions are called before other pre_execute functions.
/// TODO: is there a better way to construct this registry? Should we register
/// system pre_execute functions on-chain and dynamically?
#[inline]
pub fn system_pre_execute_functions() -> Vec<FunctionCall> {
    vec![TransactionValidator::pre_execute_function_call()]
}
/// MoveOS system post_execute functions registry.
/// The registry is used to filter out system post_execute functions.
/// All system post_execute functions are called after other post_execute functions.
/// TODO: is there a better way to construct this registry? Should we register
/// system pre_execute functions on-chain and dynamically?
#[inline]
pub fn system_post_execute_functions() -> Vec<FunctionCall> {
    vec![TransactionValidator::post_execute_function_call()]
}
