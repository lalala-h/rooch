// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use coerce::actor::message::Message;
use moveos_types::moveos_std::event::Event;
use moveos_types::state::StateChangeSet;
use moveos_types::transaction::{TransactionExecutionInfo, VerifiedMoveOSTransaction};
use rooch_types::indexer::event_filter::{EventFilter, IndexerEvent, IndexerEventID};
use rooch_types::indexer::state::{
    GlobalStateFilter, IndexerGlobalState, IndexerStateID, IndexerTableChangeSet,
    IndexerTableState, StateSyncFilter, TableStateFilter,
};
use rooch_types::indexer::transaction_filter::TransactionFilter;
use rooch_types::transaction::{
    rooch::RoochTransaction, TransactionSequenceInfo, TransactionWithInfo,
};
use serde::{Deserialize, Serialize};

/// Indexer Transaction write Message
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexerTransactionMessage {
    pub transaction: RoochTransaction,
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
    pub transaction: RoochTransaction,
    pub sequence_info: TransactionSequenceInfo,
    pub moveos_tx: VerifiedMoveOSTransaction,
}

impl Message for IndexerEventsMessage {
    type Result = Result<()>;
}

/// Indexer State write Message
// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct IndexerStatesMessage {
    pub tx_order: u64,
    pub state_change_set: StateChangeSet,
}

impl Message for IndexerStatesMessage {
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

/// Query Indexer Global States Message
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryIndexerGlobalStatesMessage {
    pub filter: GlobalStateFilter,
    // exclusive cursor if `Some`, otherwise start from the beginning
    pub cursor: Option<IndexerStateID>,
    pub limit: usize,
    pub descending_order: bool,
}

impl Message for QueryIndexerGlobalStatesMessage {
    type Result = Result<Vec<IndexerGlobalState>>;
}

/// Query Indexer Table States Message
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryIndexerTableStatesMessage {
    pub filter: TableStateFilter,
    // exclusive cursor if `Some`, otherwise start from the beginning
    pub cursor: Option<IndexerStateID>,
    pub limit: usize,
    pub descending_order: bool,
}

impl Message for QueryIndexerTableStatesMessage {
    type Result = Result<Vec<IndexerTableState>>;
}

/// Sync Indexer State change sets Message
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncIndexerStatesMessage {
    pub filter: Option<StateSyncFilter>,
    // exclusive cursor if `Some`, otherwise start from the beginning
    pub cursor: Option<IndexerStateID>,
    pub limit: usize,
    pub descending_order: bool,
}

impl Message for SyncIndexerStatesMessage {
    type Result = Result<Vec<IndexerTableChangeSet>>;
}
