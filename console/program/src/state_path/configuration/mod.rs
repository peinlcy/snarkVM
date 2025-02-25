// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use snarkvm_console_collections::merkle_tree::MerklePath;
use snarkvm_console_network::BHPMerkleTree;

/// The depth of the Merkle tree for the blocks.
pub const BLOCKS_DEPTH: u8 = 32;
/// The depth of the Merkle tree for the block header.
pub const HEADER_DEPTH: u8 = 3;
/// The depth of the Merkle tree for transactions in a block.
pub const TRANSACTIONS_DEPTH: u8 = 16;
/// The depth of the Merkle tree for the transaction.
pub const TRANSACTION_DEPTH: u8 = 5;
/// The depth of the Merkle tree for the transition.
pub const TRANSITION_DEPTH: u8 = 5;

/// The Merkle tree for the block state.
pub type BlockTree<N> = BHPMerkleTree<N, BLOCKS_DEPTH>;
/// The Merkle path for the state tree blocks.
pub type BlockPath<N> = MerklePath<N, BLOCKS_DEPTH>;

/// The Merkle tree for the block header.
pub type HeaderTree<N> = BHPMerkleTree<N, HEADER_DEPTH>;
/// The Merkle path for the block header.
pub type HeaderPath<N> = MerklePath<N, HEADER_DEPTH>;

/// The Merkle tree for transactions in a block.
pub type TransactionsTree<N> = BHPMerkleTree<N, TRANSACTIONS_DEPTH>;
/// The Merkle path for transaction in a block.
pub type TransactionsPath<N> = MerklePath<N, TRANSACTIONS_DEPTH>;

/// The Merkle tree for the transaction.
pub type TransactionTree<N> = BHPMerkleTree<N, TRANSACTION_DEPTH>;
/// The Merkle path for a function or transition in the transaction.
pub type TransactionPath<N> = MerklePath<N, TRANSACTION_DEPTH>;

/// The Merkle tree for the transition.
pub type TransitionTree<N> = BHPMerkleTree<N, TRANSITION_DEPTH>;
/// The Merkle path for an input or output ID in the transition.
pub type TransitionPath<N> = MerklePath<N, TRANSITION_DEPTH>;

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Network;

    type CurrentNetwork = snarkvm_console_network::Testnet3;

    #[test]
    fn test_transaction_depth_is_correct() {
        // We ensure 2^TRANSACTION_DEPTH - 1 == MAX_FUNCTIONS.
        // The "- 1" is for the fee transition.
        assert_eq!((2u32.checked_pow(TRANSACTION_DEPTH as u32).unwrap() - 1) as usize, CurrentNetwork::MAX_FUNCTIONS);
    }

    #[test]
    fn test_transition_depth_is_correct() {
        // We ensure 2^TRANSITION_DEPTH == (MAX_INPUTS + MAX_OUTPUTS).
        assert_eq!(
            2u32.checked_pow(TRANSITION_DEPTH as u32).unwrap() as usize,
            CurrentNetwork::MAX_INPUTS + CurrentNetwork::MAX_OUTPUTS
        );
    }
}
