use std::collections::HashMap;

pub use scope;
pub use scope::rhex;
pub use scope::rhex::Rhex;
pub use usher;

pub mod bootstrap;
pub mod startup;

pub struct Lattice {
    pub scopes: HashMap<String, scope::Scope>,
    pub ushers: usher::UsherMap,
    pub gt: u64,
}

impl Lattice {
    /// # Genesis Master Key
    /// This is the only assumption, other than the bootstrapped
    /// addresses of the root scope ushers.
    pub const GENESIS_KEY: [u8; 32] = [
        87, 135, 128, 79, 106, 64, 254, 179, 160, 51, 227, 90, 193, 88, 114, 178, 76, 135, 38, 102,
        215, 90, 22, 212, 232, 186, 192, 17, 42, 29, 161, 109,
    ];

    pub fn new() -> Self {
        Self {
            scopes: HashMap::new(),
            ushers: HashMap::new(),
            gt: 0,
        }
    }

    pub fn add_scope(&mut self, scope: &scope::Scope) {
        self.scopes.insert(scope.name.clone(), scope.clone());
    }

    pub fn add_usher(&mut self, usher: usher::Usher) {
        self.ushers.insert(usher.pk, usher);
    }
}
