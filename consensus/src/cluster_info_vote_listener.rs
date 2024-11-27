use std::collections::HashMap;

use solana_sdk::{pubkey::Pubkey, hash::Hash};

use crate::vote_stake_tracker::VoteStakeTracker;

#[derive(Default)]
pub struct SlotVoteTracker {
    // Maps pubkeys that have voted for this slot
    // to whether or not we've seen the vote on gossip.
    // True if seen on gossip, false if only seen in replay.
    pub voted: HashMap<Pubkey, bool>,
    pub optimistic_votes_tracker: HashMap<Hash, VoteStakeTracker>,
    pub voted_slot_updates: Option<Vec<Pubkey>>,
    pub gossip_only_stake: u64,
}

impl SlotVoteTracker {
    pub fn get_voted_slot_updates(&mut self) -> Option<Vec<Pubkey>> {
        self.voted_slot_updates.take()
    }

    pub fn get_or_insert_optimistic_votes_tracker(&mut self, hash: Hash) -> &mut VoteStakeTracker {
        self.optimistic_votes_tracker.entry(hash).or_default()
    }
    pub fn optimistic_votes_tracker(&self, hash: &Hash) -> Option<&VoteStakeTracker> {
        self.optimistic_votes_tracker.get(hash)
    }
}
