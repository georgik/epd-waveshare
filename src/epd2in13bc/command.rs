//! SPI Commands for the Waveshare 2.13" (B/C) E-Ink Display
use crate::traits;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) enum Command {
    PanelSetting = 0x00,

    PowerSetting = 0x01,
    PowerOff = 0x02,
    PowerOn = 0x22,
    BoosterSoftStart = 0x06,
    DeepSleep = 0x07,
    DataStartTransmission1 = 0x10,
    Init = 0x12,
    DataStartTransmission2 = 0x13,

    LutForVcom = 0x20,
    LutWhiteToWhite = 0x21,
    // LutBlackToWhite = 0x22,
    LutWhiteToBlack = 0x23,
    DisplayRefresh = 0x24,

    LutData = 0x32,

    PllControl = 0x30,
    TemperatureSensor = 0x40,
    SetRamArea = 0x44,
    SetRamArea2 = 0x45,
    TemperatureSensorSelection = 0x41,
    SetRamPointer = 0x4e,
    SetRamPointer2 = 0x4f,
    VcomAndDataIntervalSetting = 0x50,
    ResolutionSetting = 0x61,
    VcmDcSetting = 0x82,
    PowerSaving = 0xE3,
}

impl traits::Command for Command {
    /// Returns the address of the command
    fn address(self) -> u8 {
        self as u8
    }
}
