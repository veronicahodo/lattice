use anyhow::Result;
use lattice::rhex::{context::RhexContext, intent::RhexIntent, signature::RhexSignature};

use crate::comms::chatter::{ChatterResponse, ChatterResponseType};

pub mod sign;

pub fn sign_as_quorum(
    intent: &RhexIntent,
    context: &RhexContext,
    sigs: &Vec<RhexSignature>,
) -> Result<ChatterResponse> {
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::Rejected(vec![]),
        payload: vec![],
    })
}
