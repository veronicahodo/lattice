use crate::policy::Policy;

impl Policy {
    /// # Policy Can Submit
    /// This returns true if the record type is appendable
    /// by the specified group. This will also match wildcards
    /// like request:*.
    ///
    pub fn can_submit(&self, rt: &str, groups: &Vec<String>) -> bool {
        for rule in &self.rules {
            for each_rt in rule.rt.clone() {
                println!("Record Type: {}", each_rt);
                if each_rt.ends_with(&":*".to_string()) {
                    let rt_super = each_rt[..each_rt.len() - 1].to_string();
                    if rt.starts_with(&rt_super) {
                        for g in rule.append.clone() {
                            if groups.contains(&g) {
                                return true;
                            }
                        }
                    }
                }
                if each_rt.contains(&rt.to_string()) {
                    for g in rule.append.clone() {
                        if groups.contains(&g) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
