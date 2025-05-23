//!
//! [`GeneratorSummary`] is a struct that holds the summary
//! of a [`Generator`](crate::tx::Generator) output after transaction generation.
//! The summary includes total amount, total fees consumed,
//! total UTXOs consumed etc.
//!

use crate::utils::*;
use borsh::{BorshDeserialize, BorshSerialize};
use entropyx_consensus_core::network::{NetworkId, NetworkType};
use entropyx_consensus_core::tx::TransactionId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct GeneratorSummary {
    pub network_id: NetworkId,
    pub aggregated_utxos: usize,
    pub aggregated_fees: u64,
    pub number_of_generated_transactions: usize,
    pub final_transaction_amount: Option<u64>,
    pub final_transaction_id: Option<TransactionId>,
    pub burn_fee: u64, // ENX-CHANGE-BURN:
}

impl GeneratorSummary {
    pub fn network_type(&self) -> NetworkType {
        self.network_id.into()
    }

    pub fn network_id(&self) -> NetworkId {
        self.network_id
    }

    pub fn aggregated_utxos(&self) -> usize {
        self.aggregated_utxos
    }

    pub fn aggregated_fees(&self) -> u64 {
        self.aggregated_fees
    }

    pub fn number_of_generated_transactions(&self) -> usize {
        self.number_of_generated_transactions
    }

    pub fn final_transaction_amount(&self) -> Option<u64> {
        self.final_transaction_amount
    }

    pub fn final_transaction_id(&self) -> Option<TransactionId> {
        self.final_transaction_id
    }

    /// ENX-CHANGE-BURN:
    /// Get the burn fee
    pub fn burn_fee(&self) -> u64 {
        self.burn_fee
    }

    /// ENX-CHANGE-BURN:
    /// Combine multiple [`GeneratorSummary`] into one.
    pub fn combine(summaries: Vec<GeneratorSummary>) -> Self {
        let mut combined = GeneratorSummary {
            network_id: summaries[0].network_id,
            aggregated_utxos: 0,
            aggregated_fees: 0,
            number_of_generated_transactions: summaries.len(),
            final_transaction_amount: None,
            final_transaction_id: None,
            burn_fee: 0,
        };
        for summary in summaries {
            combined.aggregated_utxos += summary.aggregated_utxos;
            combined.aggregated_fees += summary.aggregated_fees;
            combined.final_transaction_amount = Some(combined.final_transaction_amount.unwrap_or(0) + summary.final_transaction_amount.unwrap_or(0));
            combined.burn_fee = combined.burn_fee.saturating_add(summary.burn_fee);
            combined.final_transaction_id = summary.final_transaction_id;
        }
        combined
    }
}

impl fmt::Display for GeneratorSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let transactions = if self.number_of_generated_transactions == 1 {
            "".to_string()
        } else {
            format!("Batch Transactions: {}", self.number_of_generated_transactions)
        };

        // ENX-CHANGE-BURN: Safe calculation of regular fees to avoid overflow  
        let regular_fees = if self.aggregated_fees > self.burn_fee {
            self.aggregated_fees - self.burn_fee
        } else {
            0 
        };

        if let Some(final_transaction_amount) = self.final_transaction_amount {
            let total = final_transaction_amount + self.aggregated_fees;
            // ENX-CHANGE-BURN: Add the burn fee to the total
            write!(
                f,
                "Amount: {} Burn: {} Fees: {}  Total: {}  UTXOs: {}  {}",
                sompi_to_entropyx_string_with_suffix(final_transaction_amount, &self.network_id),
                sompi_to_entropyx_string_with_suffix(self.burn_fee, &self.network_id),
                sompi_to_entropyx_string_with_suffix(regular_fees, &self.network_id),
                sompi_to_entropyx_string_with_suffix(total, &self.network_id),
                self.aggregated_utxos,
                transactions
            )?;
        } else {
            // ENX-CHANGE-BURN: Add the burn fee to the total
            write!(
                f,
                "Burn: {} Fees: {}  UTXOs: {}  {}",
                sompi_to_entropyx_string_with_suffix(self.burn_fee, &self.network_id),
                sompi_to_entropyx_string_with_suffix(regular_fees, &self.network_id),
                self.aggregated_utxos,
                transactions
            )?;
        }
        Ok(())
    }
}
