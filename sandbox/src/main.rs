use std::{collections::HashMap, sync::Arc};

use solana_core::consensus::{
    fork_choice::select_vote_and_reset_forks,
    heaviest_subtree_fork_choice::HeaviestSubtreeForkChoice,
    progress_map::{ForkProgress, ProgressMap},
};
use solana_runtime::bank::Bank;

fn main() {
    afl::fuzz!(|data: Data| run(data));
}

fn arb_bank_frozen(arb: u64) -> Arc<Bank> {
    let b = Bank::new_for_tests(&solana_genesis_config::create_genesis_config(arb).0);
    b.freeze();
    b.into()
}

type Data = (
    u64,
    Vec<u64>,
    usize,
    usize,
    usize,
    (u64, u64),
);
fn run((bank, banks, bank1_idx, bank2_idx, bank3_idx, pmap): Data) {
    let bank = arb_bank_frozen(bank);
    let mut banks = banks.into_iter().map(arb_bank_frozen).collect::<Vec<_>>();
    banks.push(bank);

    let bank3 = &banks[bank3_idx % banks.len()];
    let fork_choice = HeaviestSubtreeForkChoice::new_from_frozen_banks(
        (bank3.slot(), bank3.hash()),
        &banks,
    );

    let (nblks, ndropped) = pmap;
    let mut pmap = ProgressMap::default();

    for bank in &banks {
        pmap.insert(
            bank.slot(),
            ForkProgress::new(bank.hash(), None, None, nblks, ndropped),
        );
    }

    let _res = select_vote_and_reset_forks(
        &banks[bank1_idx % banks.len()],
        Some(&banks[bank2_idx % banks.len()]),
        &HashMap::new(),
        &HashMap::new(),
        &pmap,
        &mut Default::default(),
        &Default::default(),
        &fork_choice,
    );
}
