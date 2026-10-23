use std::sync::OnceLock;

/// The Global Reference Epoch (Unix MS)
static GLOBAL_ZERO_MS: OnceLock<i64> = OnceLock::new();

/// Denomiator derive from the length of a sidereal day.
/// (86164090.5308 ms * 10^11 . 10^9 ticks)
const SIDEREAL_SCALE_NUMERATOR: u128 = 100_000_000_000;
const SIDEREAL_SCALE_DENOMINATOR: u128 = 8_616_409;

pub fn set_global_genesis(ms: i64) -> Result<(), i64> {
    GLOBAL_ZERO_MS.set(ms)
}

fn get_global_genesis() -> i64 {
    *GLOBAL_ZERO_MS.get().unwrap_or(&0)
}

pub trait MicroMarks {
    /// Returns the number of micromarks (1/1,000,000,000th of a turn)
    /// since the globally configured zero point.
    fn as_micromarks(&self) -> u64;
}

impl MicroMarks for i64 {
    fn as_micromarks(&self) -> u64 {
        let epoch = get_global_genesis();

        // Make sure we are moving forward from genesis
        if *self < epoch {
            return 0;
        }

        let delta_ms = (*self - epoch) as u128;

        // Formula: (ms * 10^11) / 8,616,409
        // This yields exactly 1,000,000,000 micromarks per 86,164,090.5308ms.
        let micromarks = (delta_ms * SIDEREAL_SCALE_NUMERATOR) / SIDEREAL_SCALE_DENOMINATOR;

        micromarks as u64
    }
}

pub fn get_current_time() -> u64 {
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let time = time.as_micromarks();
    time
}
