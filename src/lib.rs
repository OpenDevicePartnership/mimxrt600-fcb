#![no_std]

use bitfield::bitfield;
use {FlexSpiLutOpcode::*, FlexSpiNumPads::*};

#[repr(C)]
#[repr(packed)]
pub struct FlexSPIFlashConfigurationBlock {
    _tag: u32,
    _version: u32,
    _reserved0: [u8; 4],
    _read_sample_clk: ReadSampleClk,
    _cs_hold_time: u8,
    _cs_setup_time: u8,
    _column_address_width: ColumnAddressWidth,
    _device_mode_cfg_enable: u8,
    _reserved1: [u8; 1],
    _wait_time_cfg_commands: u16,
    _device_mode_seq: DeviceModeSeq,
    _device_mode_arg: [u8; 4],
    _config_cmd_enable: u8,
    _config_mode_type: [u8; 3],
    _config_cmd_seqs: [u8; 12],
    _reserved2: [u8; 4],
    _config_cmd_args: [u8; 12],
    _reserved3: [u8; 4],
    _controller_misc_option: ControllerMiscOption,
    _device_type: DeviceType,
    _sflash_pad_type: SFlashPadType,
    _serial_clk_freq: SerialClkFreq,
    _lut_custom_seq_enable: u8,
    _reserved4: [u8; 8],
    _sflash_a1_size: u32,
    _sflash_a2_size: u32,
    _sflash_b1_size: u32,
    _sflash_b2_size: u32,
    _cs_pad_override: u32,
    _sclk_pad_setting_override: u32,
    _data_pad_setting_override: u32,
    _dqs_pad_setting_override: u32,
    _timeout_in_ms: u32,
    _command_interval: u32,
    _data_valid_time: u32,
    _busy_offset: u16,
    _busy_bit_polarity: BusyBitPolarity,
    _lookup_table: [u32; 64],
    _lut_custom_seq: [u8; 48],
    _reserved5: [u8; 16],
    _page_size: u32,
    _sector_size: u32,
    _ipcmd_serial_clk_freq: u8,
    _is_uniform_block_size: u8,
    _is_data_order_swapped: u8,
    _reserved6: [u8; 1],
    _serial_nor_type: SerialNORType,
    _need_exit_no_cmd_mode: u8,
    _half_clk_for_non_read_cmd: u8,
    _need_restore_no_cmd_mode: u8,
    _block_size: u32,
    _flash_state_ctx: u32,
    _reserved7: [u8; 40],
}

impl FlexSPIFlashConfigurationBlock {
    const TAG: u32 = 0x42464346;
    const VERSION: u32 = 0x56020000;

    pub const fn build() -> Self {
        Self {
            _tag: Self::TAG,
            _version: Self::VERSION,
            _reserved0: [0; 4],
            _read_sample_clk: ReadSampleClk::InternalLoopback,
            _cs_hold_time: 3,
            _cs_setup_time: 3,
            _column_address_width: ColumnAddressWidth::Other,
            _device_mode_cfg_enable: 1,
            _reserved1: [2; 1],
            _wait_time_cfg_commands: 1,
            _device_mode_seq: DeviceModeSeq::build(),
            _device_mode_arg: [2, 0, 0, 0],
            _config_cmd_enable: 0,
            _config_mode_type: [0; 3],
            _config_cmd_seqs: [0; 12],
            _reserved2: [0; 4],
            _config_cmd_args: [0; 12],
            _reserved3: [0; 4],
            _controller_misc_option: ControllerMiscOption(0x50),
            _device_type: DeviceType::SerialNOR,
            _sflash_pad_type: SFlashPadType::OctalPads,
            _serial_clk_freq: SerialClkFreq::SdrDdr30mhz,
            _lut_custom_seq_enable: 0,
            _reserved4: [0; 8],
            _sflash_a1_size: 0,
            _sflash_a2_size: 0,
            _sflash_b1_size: 0x04000000,
            _sflash_b2_size: 0,
            _cs_pad_override: 0,
            _sclk_pad_setting_override: 0,
            _data_pad_setting_override: 0,
            _dqs_pad_setting_override: 0,
            _timeout_in_ms: 0,
            _command_interval: 0,
            _data_valid_time: 0,
            _busy_offset: 0,
            _busy_bit_polarity: BusyBitPolarity::Normal,
            _lookup_table: [
                // Read
                flexspi_lut_seq(CMD_DDR, Octal, 0xee, CMD_DDR, Octal, 0x11),
                flexspi_lut_seq(RADDR_DDR, Octal, 0x20, DUMMY_DDR, Octal, 0x29),
                flexspi_lut_seq(READ_DDR, Octal, 0x04, STOP, Single, 0x00),
                0,
                // Read status SPI
                flexspi_lut_seq(CMD_SDR, Single, 0x05, READ_SDR, Single, 0x04),
                0,
                0,
                0,
                // Read status OPI
                flexspi_lut_seq(CMD_DDR, Octal, 0x05, CMD_DDR, Octal, 0xFA),
                flexspi_lut_seq(RADDR_DDR, Octal, 0x20, DUMMY_DDR, Octal, 0x14),
                flexspi_lut_seq(READ_DDR, Octal, 0x04, STOP, Single, 0x00),
                0,
                // Write enable
                flexspi_lut_seq(CMD_SDR, Single, 0x06, STOP, Single, 0x00),
                0,
                0,
                0,
                // Write enable - OPI
                flexspi_lut_seq(CMD_DDR, Octal, 0x06, CMD_DDR, Octal, 0xF9),
                0,
                0,
                0,
                // Erase Sector
                flexspi_lut_seq(CMD_DDR, Octal, 0x21, CMD_DDR, Octal, 0xDE),
                flexspi_lut_seq(RADDR_DDR, Octal, 0x20, STOP, Single, 0x00),
                0,
                0,
                // Enable OPI DDR mode
                flexspi_lut_seq(CMD_SDR, Single, 0x72, CMD_SDR, Single, 0x00),
                flexspi_lut_seq(CMD_SDR, Single, 0x00, CMD_SDR, Single, 0x00),
                flexspi_lut_seq(CMD_SDR, Single, 0x00, WRITE_SDR, Single, 0x01),
                0,
                // Unused
                0,
                0,
                0,
                0,
                // Erase block
                flexspi_lut_seq(CMD_DDR, Octal, 0xDC, CMD_DDR, Octal, 0x23),
                flexspi_lut_seq(RADDR_DDR, Octal, 0x20, STOP, Single, 0x00),
                0,
                0,
                // Page program
                flexspi_lut_seq(CMD_DDR, Octal, 0x12, CMD_DDR, Octal, 0xED),
                flexspi_lut_seq(RADDR_DDR, Octal, 0x20, WRITE_DDR, Octal, 0x04),
                0,
                0,
                // Unused
                0,
                0,
                0,
                0,
                // Erase chip
                flexspi_lut_seq(CMD_DDR, Octal, 0x60, CMD_DDR, Octal, 0x9F),
                0,
                0,
                0,
                // Remainder is unused
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            _lut_custom_seq: [0; 48],
            _reserved5: [0; 16],
            _page_size: 256,
            _sector_size: 4096,
            _ipcmd_serial_clk_freq: 1,
            _is_uniform_block_size: 0,
            _is_data_order_swapped: 0,
            _reserved6: [0; 1],
            _serial_nor_type: SerialNORType::Xpi,
            _need_exit_no_cmd_mode: 0,
            _half_clk_for_non_read_cmd: 0,
            _need_restore_no_cmd_mode: 0,
            _block_size: 0x10_000,
            _flash_state_ctx: 0x07008200,
            _reserved7: [0; 40],
        }
    }

    pub const fn read_sample_clk(self, _read_sample_clk: ReadSampleClk) -> Self {
        Self {
            _read_sample_clk,
            ..self
        }
    }

    pub const fn cs_hold_time(self, _cs_hold_time: u8) -> Self {
        Self { _cs_hold_time, ..self }
    }

    pub const fn cs_setup_time(self, _cs_setup_time: u8) -> Self {
        Self { _cs_setup_time, ..self }
    }

    pub const fn column_address_width(self, _column_address_width: ColumnAddressWidth) -> Self {
        Self {
            _column_address_width,
            ..self
        }
    }

    pub const fn device_mode_cfg_enable(self, _device_mode_cfg_enable: u8) -> Self {
        Self {
            _device_mode_cfg_enable,
            ..self
        }
    }

    pub const fn wait_time_cfg_commands(self, _wait_time_cfg_commands: u16) -> Self {
        Self {
            _wait_time_cfg_commands,
            ..self
        }
    }

    pub const fn device_mode_seq(self, _device_mode_seq: DeviceModeSeq) -> Self {
        Self {
            _device_mode_seq,
            ..self
        }
    }

    pub const fn device_mode_arg(self, _device_mode_arg: [u8; 4]) -> Self {
        Self {
            _device_mode_arg,
            ..self
        }
    }

    pub const fn config_cmd_enable(self, _config_cmd_enable: u8) -> Self {
        Self {
            _config_cmd_enable,
            ..self
        }
    }

    pub const fn config_mode_type(self, _config_mode_type: [u8; 3]) -> Self {
        Self {
            _config_mode_type,
            ..self
        }
    }

    pub const fn config_cmd_seqs(self, _config_cmd_seqs: [u8; 12]) -> Self {
        Self {
            _config_cmd_seqs,
            ..self
        }
    }

    pub const fn config_cmd_args(self, _config_cmd_args: [u8; 12]) -> Self {
        Self {
            _config_cmd_args,
            ..self
        }
    }

    pub const fn controller_misc_option(self, _controller_misc_option: ControllerMiscOption) -> Self {
        Self {
            _controller_misc_option,
            ..self
        }
    }

    pub const fn device_type(self, _device_type: DeviceType) -> Self {
        Self { _device_type, ..self }
    }

    pub const fn sflash_pad_type(self, _sflash_pad_type: SFlashPadType) -> Self {
        Self {
            _sflash_pad_type,
            ..self
        }
    }

    pub const fn serial_clk_freq(self, _serial_clk_freq: SerialClkFreq) -> Self {
        Self {
            _serial_clk_freq,
            ..self
        }
    }

    pub const fn lut_custom_seq_enable(self, _lut_custom_seq_enable: u8) -> Self {
        Self {
            _lut_custom_seq_enable,
            ..self
        }
    }

    pub const fn sflash_a1_size(self, _sflash_a1_size: u32) -> Self {
        Self {
            _sflash_a1_size,
            ..self
        }
    }

    pub const fn sflash_a2_size(self, _sflash_a2_size: u32) -> Self {
        Self {
            _sflash_a2_size,
            ..self
        }
    }

    pub const fn sflash_b1_size(self, _sflash_b1_size: u32) -> Self {
        Self {
            _sflash_b1_size,
            ..self
        }
    }

    pub const fn sflash_b2_size(self, _sflash_b2_size: u32) -> Self {
        Self {
            _sflash_b2_size,
            ..self
        }
    }

    pub const fn cs_pad_override(self, _cs_pad_override: u32) -> Self {
        Self {
            _cs_pad_override,
            ..self
        }
    }

    pub const fn sclk_pad_setting_override(self, _sclk_pad_setting_override: u32) -> Self {
        Self {
            _sclk_pad_setting_override,
            ..self
        }
    }

    pub const fn data_pad_setting_override(self, _data_pad_setting_override: u32) -> Self {
        Self {
            _data_pad_setting_override,
            ..self
        }
    }

    pub const fn dqs_pad_setting_override(self, _dqs_pad_setting_override: u32) -> Self {
        Self {
            _dqs_pad_setting_override,
            ..self
        }
    }

    pub const fn timeout_in_ms(self, _timeout_in_ms: u32) -> Self {
        Self { _timeout_in_ms, ..self }
    }

    pub const fn command_interval(self, _command_interval: u32) -> Self {
        Self {
            _command_interval,
            ..self
        }
    }

    pub const fn data_valid_time(self, _data_valid_time: u32) -> Self {
        Self {
            _data_valid_time,
            ..self
        }
    }

    pub const fn busy_offset(self, _busy_offset: u16) -> Self {
        Self { _busy_offset, ..self }
    }

    pub const fn busy_bit_polarity(self, _busy_bit_polarity: BusyBitPolarity) -> Self {
        Self {
            _busy_bit_polarity,
            ..self
        }
    }

    pub const fn lookup_table(self, _lookup_table: [u32; 64]) -> Self {
        Self { _lookup_table, ..self }
    }

    pub const fn lut_custom_seq(self, _lut_custom_seq: [u8; 48]) -> Self {
        Self {
            _lut_custom_seq,
            ..self
        }
    }

    pub const fn page_size(self, _page_size: u32) -> Self {
        Self { _page_size, ..self }
    }

    pub const fn sector_size(self, _sector_size: u32) -> Self {
        Self { _sector_size, ..self }
    }

    pub const fn ipcmd_serial_clk_freq(self, _ipcmd_serial_clk_freq: u8) -> Self {
        Self {
            _ipcmd_serial_clk_freq,
            ..self
        }
    }

    pub const fn is_uniform_block_size(self, _is_uniform_block_size: u8) -> Self {
        Self {
            _is_uniform_block_size,
            ..self
        }
    }

    pub const fn is_data_order_swapped(self, _is_data_order_swapped: u8) -> Self {
        Self {
            _is_data_order_swapped,
            ..self
        }
    }

    pub const fn serial_nor_type(self, _serial_nor_type: SerialNORType) -> Self {
        Self {
            _serial_nor_type,
            ..self
        }
    }

    pub const fn need_exit_no_cmd_mode(self, _need_exit_no_cmd_mode: u8) -> Self {
        Self {
            _need_exit_no_cmd_mode,
            ..self
        }
    }

    pub const fn half_clk_for_non_read_cmd(self, _half_clk_for_non_read_cmd: u8) -> Self {
        Self {
            _half_clk_for_non_read_cmd,
            ..self
        }
    }

    pub const fn need_restore_no_cmd_mode(self, _need_restore_no_cmd_mode: u8) -> Self {
        Self {
            _need_restore_no_cmd_mode,
            ..self
        }
    }

    pub const fn block_size(self, _block_size: u32) -> Self {
        Self { _block_size, ..self }
    }

    pub const fn flash_state_ctx(self, _flash_state_ctx: u32) -> Self {
        Self {
            _flash_state_ctx,
            ..self
        }
    }
}

#[repr(u8)]
pub enum ReadSampleClk {
    InternalLoopback = 0,
    LoopBackFromDQSPad = 1,
    LoopBackFromSCKPad = 2,
    ExternalDQSSignal = 3,
}

#[repr(u8)]
pub enum ColumnAddressWidth {
    Other = 0,
    HyperFlash = 3,
}

#[repr(C)]
#[repr(packed)]
pub struct DeviceModeSeq {
    num_seq: u8,
    seq_index: u8,
    reserved: [u8; 2],
}

impl DeviceModeSeq {
    pub const fn build() -> Self {
        Self {
            num_seq: 1,
            seq_index: 6,
            reserved: [0; 2],
        }
    }
}

bitfield! {
    pub struct ControllerMiscOption(u32);
    impl Debug;

    pub differential_clock, set_differential_clock: 0;
    pub word_addressable, set_word_addressable: 3;
    pub safe_config_freq, set_safe_config_freq: 4;
    pub ddr_mode, set_ddr_mode: 6;
}

#[repr(u8)]
pub enum DeviceType {
    SerialNOR = 1,
}

#[repr(u8)]
pub enum SFlashPadType {
    SinglePad = 1,
    DualPads = 2,
    QuadPads = 4,
    OctalPads = 8,
}

#[repr(u8)]
pub enum SerialClkFreq {
    SdrDdr30mhz = 1,
    SdrDdr50mhz = 2,
    SdrDdr60mhz = 3,
    SdrDdr80mhz = 4,
    SdrDdr100mhz = 5,
    SdrDdr120mhz = 6,
    SdrDdr133mhz = 7,
    SdrDdr166mhz = 8,
    SdrDdr200mhz = 9,
}

#[repr(u16)]
pub enum BusyBitPolarity {
    Normal = 0,
    Inverted = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum FlexSpiLutOpcode {
    CMD_SDR = 0x01,
    CMD_DDR = 0x21,
    RADDR_SDR = 0x02,
    RADDR_DDR = 0x22,
    CADDR_SDR = 0x03,
    CADDR_DDR = 0x23,
    MODE1_SDR = 0x04,
    MODE1_DDR = 0x24,
    MODE2_SDR = 0x05,
    MODE2_DDR = 0x25,
    MODE4_SDR = 0x06,
    MODE4_DDR = 0x26,
    MODE8_SDR = 0x07,
    MODE8_DDR = 0x27,
    WRITE_SDR = 0x08,
    WRITE_DDR = 0x28,
    READ_SDR = 0x09,
    READ_DDR = 0x29,
    LEARN_SDR = 0x0a,
    LEARN_DDR = 0x2a,
    DATSZ_SDR = 0x0b,
    DATSZ_DDR = 0x2b,
    DUMMY_SDR = 0x0c,
    DUMMY_DDR = 0x2c,
    DUMMY_RWDS_SDR = 0x0d,
    DUMMY_RWDS_DDR = 0x2d,
    JMP_ON_CS = 0x1f,
    STOP = 0x00,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FlexSpiNumPads {
    Single = 0,
    Dual = 1,
    Quad = 2,
    Octal = 3,
}

#[repr(u8)]
pub enum SerialNORType {
    StandardSpi = 0,
    HyperBus = 1,
    Xpi = 2,
    NoCmd = 3,
}

#[inline(always)]
pub const fn flexspi_lut_seq(
    opcode0: FlexSpiLutOpcode,
    num_pads0: FlexSpiNumPads,
    operand0: u8,
    opcode1: FlexSpiLutOpcode,
    num_pads1: FlexSpiNumPads,
    operand1: u8,
) -> u32 {
    let opcode0_raw = ((opcode0 as u32) << 10) & 0xfc00;
    let num_pads0_raw = ((num_pads0 as u32) << 8) & 0x0300;
    let operand0_raw = ((operand0 as u32) << 0) & 0xff;
    let opcode1_raw = ((opcode1 as u32) << 26) & 0xfc000000;
    let num_pads1_raw = ((num_pads1 as u32) << 24) & 0x3000000;
    let operand1_raw = ((operand1 as u32) << 16) & 0xff0000;

    opcode0_raw | num_pads0_raw | operand0_raw | opcode1_raw | num_pads1_raw | operand1_raw
}
