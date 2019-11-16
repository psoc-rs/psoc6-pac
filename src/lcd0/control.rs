#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LS_EN`"]
pub type LS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LS_EN`"]
pub struct LS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `HS_EN`"]
pub type HS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS_EN`"]
pub struct HS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "HS/LS Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCD_MODE_A {
    #[doc = "0: Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    LS = 0,
    #[doc = "1: Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    HS = 1,
}
impl From<LCD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCD_MODE`"]
pub type LCD_MODE_R = crate::R<bool, LCD_MODE_A>;
impl LCD_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_MODE_A {
        match self.bits {
            false => LCD_MODE_A::LS,
            true => LCD_MODE_A::HS,
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == LCD_MODE_A::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == LCD_MODE_A::HS
    }
}
#[doc = "Write proxy for field `LCD_MODE`"]
pub struct LCD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCD_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(LCD_MODE_A::LS)
    }
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    #[inline(always)]
    pub fn hs(self) -> &'a mut W {
        self.variant(LCD_MODE_A::HS)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "LCD driving waveform type configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE_A {
    #[doc = "0: Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    TYPE_A = 0,
    #[doc = "1: Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    TYPE_B = 1,
}
impl From<TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<bool, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            false => TYPE_A::TYPE_A,
            true => TYPE_A::TYPE_B,
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_A`"]
    #[inline(always)]
    pub fn is_type_a(&self) -> bool {
        *self == TYPE_A::TYPE_A
    }
    #[doc = "Checks if the value of the field is `TYPE_B`"]
    #[inline(always)]
    pub fn is_type_b(&self) -> bool {
        *self == TYPE_A::TYPE_B
    }
}
#[doc = "Write proxy for field `TYPE`"]
pub struct TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    #[inline(always)]
    pub fn type_a(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_A)
    }
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    #[inline(always)]
    pub fn type_b(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_B)
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
#[doc = "Driving mode configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_MODE_A {
    #[doc = "0: PWM Mode"]
    PWM = 0,
    #[doc = "1: Digital Correlation Mode"]
    CORRELATION = 1,
}
impl From<OP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OP_MODE`"]
pub type OP_MODE_R = crate::R<bool, OP_MODE_A>;
impl OP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_MODE_A {
        match self.bits {
            false => OP_MODE_A::PWM,
            true => OP_MODE_A::CORRELATION,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == OP_MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `CORRELATION`"]
    #[inline(always)]
    pub fn is_correlation(&self) -> bool {
        *self == OP_MODE_A::CORRELATION
    }
}
#[doc = "Write proxy for field `OP_MODE`"]
pub struct OP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(OP_MODE_A::PWM)
    }
    #[doc = "Digital Correlation Mode"]
    #[inline(always)]
    pub fn correlation(self) -> &'a mut W {
        self.variant(OP_MODE_A::CORRELATION)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "PWM bias selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_A {
    #[doc = "0: 1/2 Bias"]
    HALF = 0,
    #[doc = "1: 1/3 Bias"]
    THIRD = 1,
    #[doc = "2: 1/4 Bias (not supported by LS generator)"]
    FOURTH = 2,
    #[doc = "3: 1/5 Bias (not supported by LS generator)"]
    FIFTH = 3,
}
impl From<BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BIAS`"]
pub type BIAS_R = crate::R<u8, BIAS_A>;
impl BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::HALF,
            1 => BIAS_A::THIRD,
            2 => BIAS_A::FOURTH,
            3 => BIAS_A::FIFTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == BIAS_A::HALF
    }
    #[doc = "Checks if the value of the field is `THIRD`"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == BIAS_A::THIRD
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == BIAS_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `FIFTH`"]
    #[inline(always)]
    pub fn is_fifth(&self) -> bool {
        *self == BIAS_A::FIFTH
    }
}
#[doc = "Write proxy for field `BIAS`"]
pub struct BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(BIAS_A::HALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn third(self) -> &'a mut W {
        self.variant(BIAS_A::THIRD)
    }
    #[doc = "1/4 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(BIAS_A::FOURTH)
    }
    #[doc = "1/5 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fifth(self) -> &'a mut W {
        self.variant(BIAS_A::FIFTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `COM_NUM`"]
pub type COM_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COM_NUM`"]
pub struct COM_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> COM_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LS_EN_STAT`"]
pub type LS_EN_STAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn ls_en(&self) -> LS_EN_R {
        LS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn hs_en(&self) -> HS_EN_R {
        HS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    pub fn lcd_mode(&self) -> LCD_MODE_R {
        LCD_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    pub fn op_mode(&self) -> OP_MODE_R {
        OP_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn com_num(&self) -> COM_NUM_R {
        COM_NUM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub fn ls_en_stat(&self) -> LS_EN_STAT_R {
        LS_EN_STAT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn ls_en(&mut self) -> LS_EN_W {
        LS_EN_W { w: self }
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn hs_en(&mut self) -> HS_EN_W {
        HS_EN_W { w: self }
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    pub fn lcd_mode(&mut self) -> LCD_MODE_W {
        LCD_MODE_W { w: self }
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W { w: self }
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    pub fn op_mode(&mut self) -> OP_MODE_W {
        OP_MODE_W { w: self }
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W { w: self }
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn com_num(&mut self) -> COM_NUM_W {
        COM_NUM_W { w: self }
    }
}
