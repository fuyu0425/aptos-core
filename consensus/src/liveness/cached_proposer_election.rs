// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use aptos_infallible::Mutex;
use consensus_types::common::{Author, Round};

use super::proposer_election::ProposerElection;

// Wrapper around ProposerElection.
//
// Function get_valid_proposer can be expensive, and we want to make sure
// it is computed only once for a given round.
// Additionally, provides is_valid_proposal that remembers, and rejects if
// the same leader proposes multiple blocks.
pub struct CachedProposerElection {
    proposer_election: Box<dyn ProposerElection + Send + Sync>,
    recent_elections: Mutex<BTreeMap<Round, Author>>,
    window: Round,
}

impl CachedProposerElection {
    pub fn new(proposer_election: Box<dyn ProposerElection + Send + Sync>, window: Round) -> Self {
        Self {
            proposer_election,
            recent_elections: Mutex::new(BTreeMap::new()),
            window,
        }
    }
}

impl ProposerElection for CachedProposerElection {
    fn get_valid_proposer(&self, round: Round) -> Author {
        let mut recent_elections = self.recent_elections.lock();

        // Once map_first_last stabilized in Rust change:
        //   while recent_elections.iter().next().map_or(false, |(key, _)| key + self.window < round) {
        //     recent_elections.pop_first();
        //   }
        loop {
            let entry = recent_elections.iter().next();
            if entry.map_or(false, |(key, _)| key + self.window < round) {
                let key = *entry.unwrap().0;
                recent_elections.remove(&key);
            } else {
                break;
            }
        }

        *recent_elections
            .entry(round)
            .or_insert_with(|| self.proposer_election.get_valid_proposer(round))
    }
}
