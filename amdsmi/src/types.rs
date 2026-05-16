use crate::error::AmdSmiError;

pub(crate) type Result<T> = std::result::Result<T, AmdSmiError>;

#[derive(Debug)]
pub struct EnergyCount {
    pub energy_accumulator: u64,
    pub counter_resolution: f32,
    pub timestamp: u64,
}
