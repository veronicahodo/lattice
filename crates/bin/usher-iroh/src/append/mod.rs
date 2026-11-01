use anyhow::{Result, anyhow, bail};
use comms::chatter::{ChatterResponse, ChatterResponseType};
use key::enclave::Enclave;
use lattice::{Lattice, Rhex, rhex::check::CheckStatus};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{append::store::append_disk, config::UsherIrohConfig, process, request, trans};

pub mod store;

pub async fn submit_rhex(
    config: &UsherIrohConfig,
    rhex: &Rhex,
    lattice: &mut Lattice,
    trans_reg: &mut TransformRegistry,
    enclave: &Enclave,
) -> Result<ChatterResponse> {
    let time = time::get_current_time();

    if is_request(&rhex) {
        let result = request::process_request(lattice, rhex, config, enclave)?;
        return Ok(ChatterResponse {
            version: 0,
            response: ChatterResponseType::Accepted(rhex.intent.get_hash()),
            payload: result,
        });
    }

    {
        let scope = lattice
            .scopes
            .get(&rhex.intent.scope)
            .ok_or_else(|| anyhow!("Can't find scope."))?;

        let status = scope.final_check(rhex, &time)?;
        if (status.len() != 1) || (status[0] != CheckStatus::Success) {
            return Ok(ChatterResponse {
                version: 0,
                response: ChatterResponseType::Rejected(status),
                payload: vec![],
            });
        }
    }
    process::action::process_action(rhex, lattice)?;

    let output = trans::fire_transforms(rhex, trans_reg, DescriptorAction::Appending)?;
    if output.0 != CheckStatus::Success {
        bail!("Failed transform actions");
    }

    append_disk(rhex, &config.scopes).await?;
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::Accepted(rhex.curr.unwrap()),
        payload: vec![],
    })
}

fn is_request(rhex: &Rhex) -> bool {
    if rhex.intent.prev.is_none() {
        return false;
    }
    let request_types = ["request:rhex", "request:data", "request:full"];
    request_types.contains(&rhex.intent.rt.as_str())
}
