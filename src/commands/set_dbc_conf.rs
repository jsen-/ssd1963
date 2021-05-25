use packed_struct::derive::{PackedStruct, PrimitiveEnum_u8};

#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum EnergySaving {
    DbcDisable = 0b00,
    ConservativeMode = 0b01,
    NormalMode = 0b10,
    AgressiveMode = 0b11,
}

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0")]
/// Set the Dynamic Backlight Control configuration
pub struct SetDbcConf {
    /// DBC Manual Brightness enable
    #[packed_field(bits = "1")]
    pub manual_brightness_enable: bool,

    /// Transition effect is used to remove visible backlight flickering.
    /// If rapid brightness change is required, it is recommended to enable this bit.
    #[packed_field(bits = "2")]
    pub transition_effect_enable: bool,

    /// Energy saving selection for DBC
    #[packed_field(bits = "4..=5", ty = "enum")]
    pub energy_saving: EnergySaving,

    /// Master enable of DBC
    #[packed_field(bits = "7")]
    pub master_enable: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use packed_struct::PackedStruct;

    #[test]
    fn set_dbc_conf() {
        let mut zero = SetDbcConf {
            manual_brightness_enable: false,
            transition_effect_enable: false,
            energy_saving: EnergySaving::DbcDisable,
            master_enable: false,
        };
        assert_eq!(zero.pack().unwrap(), [0]);
        zero.manual_brightness_enable = true;
        assert_eq!(zero.pack().unwrap(), [0b01000000]);
        zero.transition_effect_enable = true;
        assert_eq!(zero.pack().unwrap(), [0b01100000]);
        zero.energy_saving = EnergySaving::AgressiveMode;
        assert_eq!(zero.pack().unwrap(), [0b01101100]);
        zero.master_enable = true;
        assert_eq!(zero.pack().unwrap(), [0b01101101]);
    }
}
