use anyhow::{Result, bail};
use rhex::{Rhex, check::CheckStatus};
use transform::{
    descriptor::DescriptorAction, output::TransformOutput, registry::TransformRegistry,
};

use crate::{Scope, manage::ScopeUpdateStatus, membership::Membership, policy::Policy};

impl Scope {
    /// # walk(self, init_key, rhex)
    ///
    /// This walks a scope and collects all the necessary R⬢ to build
    /// the scope. This is called on startup or anytime we get a new
    /// scope in our cache
    ///
    pub fn walk_rhex(
        name: &String,
        init_key: [u8; 32],
        rhex: &Vec<Rhex>,
        trans_reg: &TransformRegistry,
        verbose: bool,
    ) -> Result<Self> {
        // build the inital Scope
        if verbose {
            println!("Constructing scope {}...", name);
        }
        let mut scope = Scope::new(name, init_key);

        // We set the initial policy. These defaults will set the
        // state for that `lattice:genesis`/`scope:genesis` R⬢
        let policy = if scope.name.as_str() == "" {
            scope.add_membership(
                "world_line_zero".to_string(),
                vec![init_key],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: init_key,
                },
            )?;
            Policy::default_lattice_policy()
        } else {
            scope.add_membership(
                "creator".to_string(),
                vec![init_key],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: init_key,
                },
            )?;
            Policy::default_scope_policy()
        };
        scope.policy_map.push((0, policy));

        // If `rhex` has zero R⬢, we're done so return.
        if rhex.len() == 0 {
            return Ok(scope);
        }

        // Now we make sure the first R⬢ is set and signed by the
        // `init_key`, which is either the lattice master key (mine)
        // or the scope creator's.
        if verbose {
            println!("Checking inital author...");
        }
        let first_author = rhex[0].intent.author;
        if init_key != first_author {
            bail!("Chain doesn't start with the specified author");
        }

        // Walk and validate per R⬢, while building the part of the
        // scope and keys
        if verbose {
            println!("Beginning walk...");
        }
        let mut pos = 0;
        for r in rhex {
            let status = scope.final_check(r, &r.context.at)?;
            if status[0] != CheckStatus::Success {
                bail!(format!("Position: {}, Error: {:?}", pos, status[0]));
            }
            let result = scope.process_scope_changes(r)?;
            match result {
                ScopeUpdateStatus::Failed(err) => {
                    bail!(format!("Position: {}, Error: {}", pos, err));
                }
                _ => { /* ??? */ }
            }

            // Get the transforms we need for this scope/rt combo
            let triggers = trans_reg.triggers.get(&DescriptorAction::Validate);
            if triggers.is_some() {
                let mount = triggers.unwrap().get(&r.intent.scope);
                if mount.is_some() {
                    let rt = mount.unwrap().get(&r.intent.rt);
                    if rt.is_some() {
                        // iterate and run
                        for trans in rt.unwrap() {
                            let trans_out_bin = trans_reg.run(trans, &r.data)?;
                            let trans_output: TransformOutput = minicbor::decode(&trans_out_bin)?;
                            if trans_output.fatal_error() {
                                // FIXME: Figure out how we're gonna error out of this shit
                                // As is, it's REALLY BAD

                                bail!(format!(
                                    "R⬢ pos: {}, Transform Errors from {:?}: {:?}",
                                    pos,
                                    trans,
                                    trans_output.err.unwrap()
                                ));
                            }
                        }
                    }
                }
            }

            if verbose {
                println!("Processed R⬢ #{}", pos);
            }
            pos += 1;
        }

        if verbose {
            println!("Scope loaded!");
        }
        Ok(scope)
    }
}
