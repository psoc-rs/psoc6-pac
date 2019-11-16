#[doc = "Reader of register RX_CTL"]
pub type R = crate::R<u32, super::RX_CTL>;
#[doc = "Writer for register RX_CTL"]
pub type W = crate::W<u32, super::RX_CTL>;
#[doc = "Register RX_CTL `reset()`'s with value 0x0044_0510"]
impl crate::ResetValue for super::RX_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0044_0510
    }
}
#[doc = "Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B_CLOCK_INV_A {
    #[doc = "0: SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    RISING_EDGE_RX = 0,
    #[doc = "1: SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    FALLING_EDGE_RX = 1,
}
impl From<B_CLOCK_INV_A> for bool {
    #[inline(always)]
    fn from(variant: B_CLOCK_INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B_CLOCK_INV`"]
pub type B_CLOCK_INV_R = crate::R<bool, B_CLOCK_INV_A>;
impl B_CLOCK_INV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B_CLOCK_INV_A {
        match self.bits {
            false => B_CLOCK_INV_A::RISING_EDGE_RX,
            true => B_CLOCK_INV_A::FALLING_EDGE_RX,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE_RX`"]
    #[inline(always)]
    pub fn is_rising_edge_rx(&self) -> bool {
        *self == B_CLOCK_INV_A::RISING_EDGE_RX
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE_RX`"]
    #[inline(always)]
    pub fn is_falling_edge_rx(&self) -> bool {
        *self == B_CLOCK_INV_A::FALLING_EDGE_RX
    }
}
#[doc = "Write proxy for field `B_CLOCK_INV`"]
pub struct B_CLOCK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> B_CLOCK_INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B_CLOCK_INV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn rising_edge_rx(self) -> &'a mut W {
        self.variant(B_CLOCK_INV_A::RISING_EDGE_RX)
    }
    #[doc = "SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn falling_edge_rx(self) -> &'a mut W {
        self.variant(B_CLOCK_INV_A::FALLING_EDGE_RX)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH_NR_A {
    #[doc = "0: 1 channel"]
    CH_NUM1 = 0,
    #[doc = "1: 2 channels"]
    CH_NUM2 = 1,
    #[doc = "2: 3 channels"]
    CH_NUM3 = 2,
    #[doc = "3: 4 channels"]
    CH_NUM4 = 3,
    #[doc = "4: 5 channels"]
    CH_NUM5 = 4,
    #[doc = "5: 6 channels"]
    CH_NUM6 = 5,
    #[doc = "6: 7 channels"]
    CH_NUM7 = 6,
    #[doc = "7: 8 channels"]
    CH_NUM8 = 7,
}
impl From<CH_NR_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_NR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH_NR`"]
pub type CH_NR_R = crate::R<u8, CH_NR_A>;
impl CH_NR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH_NR_A {
        match self.bits {
            0 => CH_NR_A::CH_NUM1,
            1 => CH_NR_A::CH_NUM2,
            2 => CH_NR_A::CH_NUM3,
            3 => CH_NR_A::CH_NUM4,
            4 => CH_NR_A::CH_NUM5,
            5 => CH_NR_A::CH_NUM6,
            6 => CH_NR_A::CH_NUM7,
            7 => CH_NR_A::CH_NUM8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH_NUM1`"]
    #[inline(always)]
    pub fn is_ch_num1(&self) -> bool {
        *self == CH_NR_A::CH_NUM1
    }
    #[doc = "Checks if the value of the field is `CH_NUM2`"]
    #[inline(always)]
    pub fn is_ch_num2(&self) -> bool {
        *self == CH_NR_A::CH_NUM2
    }
    #[doc = "Checks if the value of the field is `CH_NUM3`"]
    #[inline(always)]
    pub fn is_ch_num3(&self) -> bool {
        *self == CH_NR_A::CH_NUM3
    }
    #[doc = "Checks if the value of the field is `CH_NUM4`"]
    #[inline(always)]
    pub fn is_ch_num4(&self) -> bool {
        *self == CH_NR_A::CH_NUM4
    }
    #[doc = "Checks if the value of the field is `CH_NUM5`"]
    #[inline(always)]
    pub fn is_ch_num5(&self) -> bool {
        *self == CH_NR_A::CH_NUM5
    }
    #[doc = "Checks if the value of the field is `CH_NUM6`"]
    #[inline(always)]
    pub fn is_ch_num6(&self) -> bool {
        *self == CH_NR_A::CH_NUM6
    }
    #[doc = "Checks if the value of the field is `CH_NUM7`"]
    #[inline(always)]
    pub fn is_ch_num7(&self) -> bool {
        *self == CH_NR_A::CH_NUM7
    }
    #[doc = "Checks if the value of the field is `CH_NUM8`"]
    #[inline(always)]
    pub fn is_ch_num8(&self) -> bool {
        *self == CH_NR_A::CH_NUM8
    }
}
#[doc = "Write proxy for field `CH_NR`"]
pub struct CH_NR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_NR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_NR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 channel"]
    #[inline(always)]
    pub fn ch_num1(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM1)
    }
    #[doc = "2 channels"]
    #[inline(always)]
    pub fn ch_num2(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM2)
    }
    #[doc = "3 channels"]
    #[inline(always)]
    pub fn ch_num3(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM3)
    }
    #[doc = "4 channels"]
    #[inline(always)]
    pub fn ch_num4(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM4)
    }
    #[doc = "5 channels"]
    #[inline(always)]
    pub fn ch_num5(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM5)
    }
    #[doc = "6 channels"]
    #[inline(always)]
    pub fn ch_num6(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM6)
    }
    #[doc = "7 channels"]
    #[inline(always)]
    pub fn ch_num7(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM7)
    }
    #[doc = "8 channels"]
    #[inline(always)]
    pub fn ch_num8(self) -> &'a mut W {
        self.variant(CH_NR_A::CH_NUM8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MS_A {
    #[doc = "0: Slave"]
    SLAVE = 0,
    #[doc = "1: Master"]
    MASTER = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<bool, MS_A>;
impl MS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            false => MS_A::SLAVE,
            true => MS_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
}
#[doc = "Write proxy for field `MS`"]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MS_A::SLAVE)
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MS_A::MASTER)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S_MODE_A {
    #[doc = "0: Left Justified"]
    LEFT_JUSTIFIED = 0,
    #[doc = "1: I2S mode"]
    I2S = 1,
    #[doc = "2: TDM mode A, the 1st Channel align to WSO\nRising Edge"]
    TDM_A = 2,
    #[doc = "3: TDM mode B, the 1st Channel align to WSO\nRising edge with1 SCK Delay"]
    TDM_B = 3,
}
impl From<I2S_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2S_MODE`"]
pub type I2S_MODE_R = crate::R<u8, I2S_MODE_A>;
impl I2S_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_MODE_A {
        match self.bits {
            0 => I2S_MODE_A::LEFT_JUSTIFIED,
            1 => I2S_MODE_A::I2S,
            2 => I2S_MODE_A::TDM_A,
            3 => I2S_MODE_A::TDM_B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_JUSTIFIED`"]
    #[inline(always)]
    pub fn is_left_justified(&self) -> bool {
        *self == I2S_MODE_A::LEFT_JUSTIFIED
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2S_MODE_A::I2S
    }
    #[doc = "Checks if the value of the field is `TDM_A`"]
    #[inline(always)]
    pub fn is_tdm_a(&self) -> bool {
        *self == I2S_MODE_A::TDM_A
    }
    #[doc = "Checks if the value of the field is `TDM_B`"]
    #[inline(always)]
    pub fn is_tdm_b(&self) -> bool {
        *self == I2S_MODE_A::TDM_B
    }
}
#[doc = "Write proxy for field `I2S_MODE`"]
pub struct I2S_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Left Justified"]
    #[inline(always)]
    pub fn left_justified(self) -> &'a mut W {
        self.variant(I2S_MODE_A::LEFT_JUSTIFIED)
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(I2S_MODE_A::I2S)
    }
    #[doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"]
    #[inline(always)]
    pub fn tdm_a(self) -> &'a mut W {
        self.variant(I2S_MODE_A::TDM_A)
    }
    #[doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    #[inline(always)]
    pub fn tdm_b(self) -> &'a mut W {
        self.variant(I2S_MODE_A::TDM_B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WS_PULSE_A {
    #[doc = "0: Pulse width is 1 SCK period"]
    SCK_PERIOD = 0,
    #[doc = "1: Pulse width is 1 channel length"]
    CH_LENGTH = 1,
}
impl From<WS_PULSE_A> for bool {
    #[inline(always)]
    fn from(variant: WS_PULSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WS_PULSE`"]
pub type WS_PULSE_R = crate::R<bool, WS_PULSE_A>;
impl WS_PULSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WS_PULSE_A {
        match self.bits {
            false => WS_PULSE_A::SCK_PERIOD,
            true => WS_PULSE_A::CH_LENGTH,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_PERIOD`"]
    #[inline(always)]
    pub fn is_sck_period(&self) -> bool {
        *self == WS_PULSE_A::SCK_PERIOD
    }
    #[doc = "Checks if the value of the field is `CH_LENGTH`"]
    #[inline(always)]
    pub fn is_ch_length(&self) -> bool {
        *self == WS_PULSE_A::CH_LENGTH
    }
}
#[doc = "Write proxy for field `WS_PULSE`"]
pub struct WS_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_PULSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WS_PULSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pulse width is 1 SCK period"]
    #[inline(always)]
    pub fn sck_period(self) -> &'a mut W {
        self.variant(WS_PULSE_A::SCK_PERIOD)
    }
    #[doc = "Pulse width is 1 channel length"]
    #[inline(always)]
    pub fn ch_length(self) -> &'a mut W {
        self.variant(WS_PULSE_A::CH_LENGTH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `WD_EN`"]
pub type WD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WD_EN`"]
pub struct WD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH_LEN_A {
    #[doc = "0: 8-bit"]
    BIT_LEN8 = 0,
    #[doc = "1: 16-bit"]
    BIT_LEN16 = 1,
    #[doc = "2: 18-bit"]
    BIT_LEN18 = 2,
    #[doc = "3: 20-bit"]
    BIT_LEN20 = 3,
    #[doc = "4: 24-bit"]
    BIT_LEN24 = 4,
    #[doc = "5: 32-bit"]
    BIT_LEN32 = 5,
}
impl From<CH_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CH_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CH_LEN`"]
pub type CH_LEN_R = crate::R<u8, CH_LEN_A>;
impl CH_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH_LEN_A::BIT_LEN8),
            1 => Val(CH_LEN_A::BIT_LEN16),
            2 => Val(CH_LEN_A::BIT_LEN18),
            3 => Val(CH_LEN_A::BIT_LEN20),
            4 => Val(CH_LEN_A::BIT_LEN24),
            5 => Val(CH_LEN_A::BIT_LEN32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_LEN8`"]
    #[inline(always)]
    pub fn is_bit_len8(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN8
    }
    #[doc = "Checks if the value of the field is `BIT_LEN16`"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN16
    }
    #[doc = "Checks if the value of the field is `BIT_LEN18`"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN18
    }
    #[doc = "Checks if the value of the field is `BIT_LEN20`"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN20
    }
    #[doc = "Checks if the value of the field is `BIT_LEN24`"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN24
    }
    #[doc = "Checks if the value of the field is `BIT_LEN32`"]
    #[inline(always)]
    pub fn is_bit_len32(&self) -> bool {
        *self == CH_LEN_A::BIT_LEN32
    }
}
#[doc = "Write proxy for field `CH_LEN`"]
pub struct CH_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bit_len8(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN8)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN24)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bit_len32(self) -> &'a mut W {
        self.variant(CH_LEN_A::BIT_LEN32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORD_LEN_A {
    #[doc = "0: 8-bit"]
    BIT_LEN8 = 0,
    #[doc = "1: 16-bit"]
    BIT_LEN16 = 1,
    #[doc = "2: 18-bit"]
    BIT_LEN18 = 2,
    #[doc = "3: 20-bit"]
    BIT_LEN20 = 3,
    #[doc = "4: 24-bit"]
    BIT_LEN24 = 4,
    #[doc = "5: 32-bit"]
    BIT_LEN32 = 5,
}
impl From<WORD_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WORD_LEN`"]
pub type WORD_LEN_R = crate::R<u8, WORD_LEN_A>;
impl WORD_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WORD_LEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WORD_LEN_A::BIT_LEN8),
            1 => Val(WORD_LEN_A::BIT_LEN16),
            2 => Val(WORD_LEN_A::BIT_LEN18),
            3 => Val(WORD_LEN_A::BIT_LEN20),
            4 => Val(WORD_LEN_A::BIT_LEN24),
            5 => Val(WORD_LEN_A::BIT_LEN32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_LEN8`"]
    #[inline(always)]
    pub fn is_bit_len8(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN8
    }
    #[doc = "Checks if the value of the field is `BIT_LEN16`"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN16
    }
    #[doc = "Checks if the value of the field is `BIT_LEN18`"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN18
    }
    #[doc = "Checks if the value of the field is `BIT_LEN20`"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN20
    }
    #[doc = "Checks if the value of the field is `BIT_LEN24`"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN24
    }
    #[doc = "Checks if the value of the field is `BIT_LEN32`"]
    #[inline(always)]
    pub fn is_bit_len32(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN32
    }
}
#[doc = "Write proxy for field `WORD_LEN`"]
pub struct WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_LEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bit_len8(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN8)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN24)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bit_len32(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `BIT_EXTENSION`"]
pub type BIT_EXTENSION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT_EXTENSION`"]
pub struct BIT_EXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_EXTENSION_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SCKO_POL`"]
pub type SCKO_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKO_POL`"]
pub struct SCKO_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKO_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SCKI_POL`"]
pub type SCKI_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCKI_POL`"]
pub struct SCKI_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKI_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    pub fn b_clock_inv(&self) -> B_CLOCK_INV_R {
        B_CLOCK_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    pub fn ch_nr(&self) -> CH_NR_R {
        CH_NR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    pub fn i2s_mode(&self) -> I2S_MODE_R {
        I2S_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub fn ws_pulse(&self) -> WS_PULSE_R {
        WS_PULSE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn wd_en(&self) -> WD_EN_R {
        WD_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    pub fn ch_len(&self) -> CH_LEN_R {
        CH_LEN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&self) -> BIT_EXTENSION_R {
        BIT_EXTENSION_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub fn scko_pol(&self) -> SCKO_POL_R {
        SCKO_POL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    pub fn scki_pol(&self) -> SCKI_POL_R {
        SCKI_POL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    pub fn b_clock_inv(&mut self) -> B_CLOCK_INV_W {
        B_CLOCK_INV_W { w: self }
    }
    #[doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    pub fn ch_nr(&mut self) -> CH_NR_W {
        CH_NR_W { w: self }
    }
    #[doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    pub fn i2s_mode(&mut self) -> I2S_MODE_W {
        I2S_MODE_W { w: self }
    }
    #[doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub fn ws_pulse(&mut self) -> WS_PULSE_W {
        WS_PULSE_W { w: self }
    }
    #[doc = "Bit 13 - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn wd_en(&mut self) -> WD_EN_W {
        WD_EN_W { w: self }
    }
    #[doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    pub fn ch_len(&mut self) -> CH_LEN_W {
        CH_LEN_W { w: self }
    }
    #[doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    pub fn word_len(&mut self) -> WORD_LEN_W {
        WORD_LEN_W { w: self }
    }
    #[doc = "Bit 23 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&mut self) -> BIT_EXTENSION_W {
        BIT_EXTENSION_W { w: self }
    }
    #[doc = "Bit 24 - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub fn scko_pol(&mut self) -> SCKO_POL_W {
        SCKO_POL_W { w: self }
    }
    #[doc = "Bit 25 - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    pub fn scki_pol(&mut self) -> SCKI_POL_W {
        SCKI_POL_W { w: self }
    }
}
