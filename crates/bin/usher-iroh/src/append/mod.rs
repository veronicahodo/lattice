use anyhow::{Result, bail};
use comms::chatter::{ChatterResponse, ChatterResponseType};
use lattice::{Lattice, Rhex, rhex::check::CheckStatus};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{append::store::append_disk, config::UsherIrohConfig, request, trans};

pub mod store;

pub async fn submit_rhex(
    config: &UsherIrohConfig,
    rhex: &Rhex,
    lattice: &mut Lattice,
    trans_reg: &TransformRegistry,
) -> Result<ChatterResponse> {
    let scope = lattice.scopes.get(&rhex.intent.scope);
    if scope.is_none() {
        bail!("Can't find scope.")
    };
    let scope = scope.unwrap();
    let time = time::get_current_time();

    if is_request(&rhex.intent.rt) {
        let result = request::process_request(lattice, rhex, config)?;
        return Ok(ChatterResponse {
            version: 0,
            response: ChatterResponseType::Accepted(rhex.intent.get_hash()),
            payload: result,
        });
    }

    let status = scope.final_check(&rhex, &time)?;
    if (status.len() != 1) || (status[0] != CheckStatus::Success) {
        // Build error out response here.
        return Ok(ChatterResponse {
            version: 0,
            response: ChatterResponseType::Rejected(status),
            payload: vec![],
        });
    }

    let output = trans::fire_transforms(rhex, trans_reg, DescriptorAction::Appending)?;
    if output.0 != CheckStatus::Success {
        bail!("Failed transform actions");
    }
    // TODO: Sign transform outputs if necessary.

    // Store
    /*if rhex.intent.rt.as_str() == "lattice:genesis" {
        let data: RhexData = cbor2::from_slice(&rhex.data)?;
        let genesis = match data {
            RhexData::Mixed { meta, binary } => {
                let json: Value = serde_json::from_slice(&meta)?;
                let time = json.get("at");
                if time.is_none() {
                    bail!("Record doesn't contain lattice start time");
                }
                let time = time
                    .unwrap()
                    .as_i64()
                    .ok_or_else(|| anyhow!("Time conversion failed"))?;
                let _ = time::set_global_genesis(time);
                // TODO: Add keys to "" scope.
                true
            }
            _ => {
                // Reject here
                bail!("Yet another failure. Do you feel better?");
            }
        };
    }*/
    append_disk(rhex, &config.scopes).await?;
    Ok(ChatterResponse {
        version: 0,
        response: ChatterResponseType::Accepted(rhex.curr.unwrap()),
        payload: vec![],
    })
}

fn is_request(rt: &String) -> bool {
    let request_types = ["request:rhex", "request:data", "request:full"];
    request_types.contains(&rt.as_str())
}
