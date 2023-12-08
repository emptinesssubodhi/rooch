// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::errors::IndexerError;
use crate::types::{
    IndexedEvent, IndexedGlobalState, IndexedTableChangeSet, IndexedTableState, IndexedTransaction,
};

pub trait IndexerStoreTrait: Send + Sync {
    fn persist_or_update_global_states(
        &self,
        states: Vec<IndexedGlobalState>,
    ) -> Result<(), IndexerError>;

    fn delete_global_states(&self, state_pks: Vec<String>) -> Result<(), IndexerError>;

    fn persist_or_update_table_states(
        &self,
        states: Vec<IndexedTableState>,
    ) -> Result<(), IndexerError>;

    fn delete_table_states(&self, state_pks: Vec<(String, String)>) -> Result<(), IndexerError>;

    fn delete_table_states_by_table_handle(
        &self,
        table_handles: Vec<String>,
    ) -> Result<(), IndexerError>;

    fn persist_table_change_sets(
        &self,
        table_change_sets: Vec<IndexedTableChangeSet>,
    ) -> Result<(), IndexerError>;

    fn persist_transactions(
        &self,
        transactions: Vec<IndexedTransaction>,
    ) -> Result<(), IndexerError>;

    fn persist_events(&self, events: Vec<IndexedEvent>) -> Result<(), IndexerError>;
}
