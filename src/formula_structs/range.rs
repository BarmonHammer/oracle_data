use super::firing_data::StatLerp;
use crate::enums::Range;

struct RangeData {
    fall_off_start: StatLerp<Range>,
    fall_off_end: StatLerp<Range>,
    ads_multiplier: f64,
    
}
