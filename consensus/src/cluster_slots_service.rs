pub mod cluster_slots {
    use std::collections::HashMap;

    use solana_sdk::pubkey::Pubkey;

    pub(crate) type SlotPubkeys = HashMap</*node:*/ Pubkey, /*stake:*/ u64>;
}
