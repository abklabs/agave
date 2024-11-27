use std::sync::Arc;

use solana_runtime::bank::Bank;
use solana_sdk::{clock::Slot, hash::Hash, pubkey::Pubkey};

use crate::{
    fork_choice::ForkChoice,
    heaviest_subtree_fork_choice::HeaviestSubtreeForkChoice,
    progress_map::{ForkProgress, ProgressMap},
};

pub const SUPERMINORITY_THRESHOLD: f64 = 1f64 / 3f64;
pub const DUPLICATE_LIVENESS_THRESHOLD: f64 = 0.1;
pub const DUPLICATE_THRESHOLD: f64 =
    1.0 - super::SWITCH_FORK_THRESHOLD - DUPLICATE_LIVENESS_THRESHOLD;

#[derive(PartialEq, Eq, Debug)]
pub enum HeaviestForkFailures {
    LockedOut(u64),
    FailedThreshold(
        Slot,
        /* vote depth */ u64,
        /* Observed stake */ u64,
        /* Total stake */ u64,
    ),
    FailedSwitchThreshold(
        Slot,
        /* Observed stake */ u64,
        /* Total stake */ u64,
    ),
    NoPropagatedConfirmation(
        Slot,
        /* Observed stake */ u64,
        /* Total stake */ u64,
    ),
}

pub fn initialize_progress_and_fork_choice(
    root_bank: &Bank,
    mut frozen_banks: Vec<Arc<Bank>>,
    my_pubkey: &Pubkey,
    vote_account: &Pubkey,
    duplicate_slot_hashes: Vec<(Slot, Hash)>,
) -> (ProgressMap, HeaviestSubtreeForkChoice) {
    let mut progress = ProgressMap::default();

    frozen_banks.sort_by_key(|bank| bank.slot());

    // Initialize progress map with any root banks
    for bank in &frozen_banks {
        let prev_leader_slot = progress.get_bank_prev_leader_slot(bank);
        progress.insert(
            bank.slot(),
            ForkProgress::new_from_bank(bank, my_pubkey, vote_account, prev_leader_slot, 0, 0),
        );
    }
    let root = root_bank.slot();
    let mut heaviest_subtree_fork_choice =
        HeaviestSubtreeForkChoice::new_from_frozen_banks((root, root_bank.hash()), &frozen_banks);

    for slot_hash in duplicate_slot_hashes {
        heaviest_subtree_fork_choice.mark_fork_invalid_candidate(&slot_hash);
    }

    (progress, heaviest_subtree_fork_choice)
}
