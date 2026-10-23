use anyhow::Result;
use lattice::{
    Rhex,
    rhex::{check::CheckStatus, intent::RhexIntent},
};
use transform::{
    context::TransformContext, descriptor::DescriptorAction, output::TransformOutput,
    registry::TransformRegistry,
};

/// # fire_transforms
/// This has to be the worst possible way to do this. This is the damn
/// CRE creeping into this. I can't help it. I just feel like the
/// modular execution is important to the Lattice.
pub fn fire_transforms(
    rhex: &Rhex,
    trans_registry: &TransformRegistry,
    action: DescriptorAction,
) -> Result<(CheckStatus, Vec<RhexIntent>)> {
    let mut storage = Vec::new();
    let mut output_intents = Vec::new();
    let action_set = trans_registry.triggers.get(&action);
    if action_set.is_some() {
        let triggers = action_set.unwrap().get(&rhex.intent.scope.to_string());
        if triggers.is_some() {
            let triggers = triggers.unwrap();
            let keys: Vec<&String> = triggers.keys().collect();
            let parent = &format!("{}:*", rhex.intent.rt.split(":").collect::<Vec<&str>>()[0]);
            for key in keys {
                if key == parent || *key == rhex.intent.rt {
                    for trans in triggers.get(key).unwrap() {
                        if let Some(entry) = trans_registry.registry.get(trans) {
                            // TODO: build additional inputs here
                            let mut transform_output: Option<Vec<u8>> = None;
                            let mut transform_diag: Option<Vec<u8>> = None;
                            let cbor = cbor2::to_canonical_vec(&rhex)?;
                            let mut ctx = TransformContext {
                                input: &cbor.clone(),
                                output: &mut transform_output,
                                diag: &mut transform_diag,
                            };
                            let result = (entry.entry.entry)(&mut ctx);
                            if result > 0 {
                                let errs = ctx.output.clone().unwrap();
                                let errs: TransformOutput = cbor2::from_slice(&errs)?;
                                if errs.fatal_error() {
                                    return Ok((
                                        CheckStatus::InteractionAborted {
                                            transform: entry.descriptor.name.clone(),
                                            exitcode: result as usize,
                                        },
                                        vec![],
                                    ));
                                }
                            } else {
                                if let Some(output) = transform_output {
                                    storage.push(output);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for buf in storage.iter() {
        let oi: TransformOutput = cbor2::from_slice(buf)?;
        if let Some(intents) = oi.outbound_intents {
            for intent in intents {
                output_intents.push(intent);
            }
        }
    }
    Ok((CheckStatus::Success, output_intents.clone()))
}
