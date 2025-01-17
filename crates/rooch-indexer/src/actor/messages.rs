// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use coerce::actor::message::Message;
use moveos_types::moveos_std::event::Event;
use moveos_types::transaction::{TransactionExecutionInfo, VerifiedMoveOSTransaction};
use rooch_types::indexer::event_filter::{EventFilter, IndexerEvent, IndexerEventID};
use rooch_types::indexer::transaction_filter::TransactionFilter;
use rooch_types::transaction::{TransactionSequenceInfo, TransactionWithInfo, TypedTransaction};
use serde::{Deserialize, Serialize};

/// Indexer Transaction write Message
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerTransactionMessage {
    pub transaction: TypedTransaction,
    pub sequence_info: TransactionSequenceInfo,
    pub execution_info: TransactionExecutionInfo,
    pub moveos_tx: VerifiedMoveOSTransaction,
}

impl Message for IndexerTransactionMessage {
    type Result = Result<()>;
}

/// Indexer Event write Message
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerEventsMessage {
    pub events: Vec<Event>,
    pub transaction: TypedTransaction,
    pub sequence_info: TransactionSequenceInfo,
    pub moveos_tx: VerifiedMoveOSTransaction,
}

impl Message for IndexerEventsMessage {
    type Result = Result<()>;
}

/// Query Indexer Transactions Message
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryIndexerTransactionsMessage {
    pub filter: TransactionFilter,
    // exclusive cursor if `Some`, otherwise start from the beginning
    pub cursor: Option<u64>,
    pub limit: usize,
    pub descending_order: bool,
}

impl Message for QueryIndexerTransactionsMessage {
    type Result = Result<Vec<TransactionWithInfo>>;
}

/// Query Indexer Events Message
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryIndexerEventsMessage {
    pub filter: EventFilter,
    // exclusive cursor if `Some`, otherwise start from the beginning
    pub cursor: Option<IndexerEventID>,
    pub limit: usize,
    pub descending_order: bool,
}

impl Message for QueryIndexerEventsMessage {
    type Result = Result<Vec<IndexerEvent>>;
}
