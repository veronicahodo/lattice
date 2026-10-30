use std::sync::OnceLock;

/// The Global Reference Epoch (Unix MS)
static GLOBAL_ZERO_MS: OnceLock<i64> = OnceLock::new();

/// Denomiator derive from the length of a sidereal day.
/// (86164090.5308 ms * 10^11 . 10^9 ticks)
const SIDEREAL_SCALE_NUMERATOR: u128 = 1_000_000_000;
const SIDEREAL_SCALE_DENOMINATOR: u128 = 86_164_090;

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

pub fn print_gt(time: &u64) -> String {
    // Periods:
    //  Micromarks: 1/1,000,000,000th of a Turn
    //  Submarks: 1/1,000,000th of a Turn
    //  Marks: 1/1,000th of a Turn
    //  Turn: 1 sidereal day
    //  Luna: 28 Turns
    //  Sol: 13 Luna + 1 Turn
    // GT Format: GT[Sol.Luna.Turn@Mark.Submark.Micromarks]
    // Constants for time conversion
    const MICROMARKS_PER_MARK: u64 = 1_000_000;
    const MICROMARKS_PER_SUBMARK: u64 = 1_000;
    const MICROMARKS_PER_TURN: u64 = 1_000_000_000;
    const TURNS_PER_LUNA: u64 = 28;
    const TURNS_PER_SOL: u64 = (13 * 28) + 1; // 365 Turns

    // Extract Turn and sub-turn fractions
    let total_turns = time / MICROMARKS_PER_TURN;
    let turn_remainder = time % MICROMARKS_PER_TURN;

    let mark = turn_remainder / MICROMARKS_PER_MARK;
    let submark = (turn_remainder % MICROMARKS_PER_MARK) / MICROMARKS_PER_SUBMARK;
    let micromarks = turn_remainder % MICROMARKS_PER_SUBMARK;

    // Extract Sol, Luna, and Turn
    let sol = total_turns / TURNS_PER_SOL;
    let sol_remainder = total_turns % TURNS_PER_SOL;
    let luna = sol_remainder / TURNS_PER_LUNA;
    let turn = sol_remainder % TURNS_PER_LUNA;

    // Format according to GT[Sol.Luna.Turn@Mark.Submark.Micromarks]
    format!(
        "GT[{}.{}.{}@{}.{}.{}]",
        sol, luna, turn, mark, submark, micromarks
    )
}
