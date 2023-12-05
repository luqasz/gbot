use fugit::{HertzU32, RateExtU32};
use num_enum::{self, IntoPrimitive, TryFromPrimitive};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;

struct SerHz32;

impl serde_with::SerializeAs<HertzU32> for SerHz32 {
    fn serialize_as<S>(value: &HertzU32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_u32(value.to_Hz());
    }
}

impl<'de> serde_with::DeserializeAs<'de, HertzU32> for SerHz32 {
    fn deserialize_as<D>(deserializer: D) -> Result<HertzU32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u32::deserialize(deserializer)?;
        return Ok(val.Hz());
    }
}

// Do not change numbers !
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
    E0 = 3,
}

// Do not change numbers !
#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Delay = 2,
}

// Do not change numbers !
#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum CommandId {
    FirmwareInfo = 0,
    FanInfo = 1,
    SetFanPWM = 2,
    AnalogRead = 3,
    SpinStepper = 4,
    StepperInfo = 5,
}

// Do not change numbers !
#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, PartialEq, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u16)]
pub enum CommandResult {
    Ok = 0,
    InvalidId = 1,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct FirmwareInfo {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub protocol_version: u8,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct FanInfo {
    pub fan: u8,
    pub max_pwm: u16,
    pub pwm_val: u16,
    pub rpm: u16,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct SetFanPWM {
    pub fan: u8,
    pub pwm: u16,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct AnalogRead {
    pub channel: u8,
    pub val: u16,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct SpinStepper {
    #[serde_as(as = "SerHz32")]
    pub frequency: HertzU32,
    pub countt: u16,
    pub axis: Axis,
    pub direction: Direction,
}

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct StepperInfo {
    #[serde_as(as = "SerHz32")]
    pub resolution: HertzU32,
    pub slots: u16,
    pub usteps: u8,
    pub interpolate: u8,
}

#[cfg(test)]
mod tests {

    use super::*;
}
