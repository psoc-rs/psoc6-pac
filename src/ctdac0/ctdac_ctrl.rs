#[doc = "Reader of register CTDAC_CTRL"]
pub type R = crate::R<u32, super::CTDAC_CTRL>;
#[doc = "Writer for register CTDAC_CTRL"]
pub type W = crate::W<u32, super::CTDAC_CTRL>;
#[doc = "Register CTDAC_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTDAC_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEGLITCH_CNT`"]
pub type DEGLITCH_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEGLITCH_CNT`"]
pub struct DEGLITCH_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `DEGLITCH_CO6`"]
pub type DEGLITCH_CO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEGLITCH_CO6`"]
pub struct DEGLITCH_CO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_CO6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DEGLITCH_COS`"]
pub type DEGLITCH_COS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEGLITCH_COS`"]
pub struct DEGLITCH_COS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEGLITCH_COS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `OUT_EN`"]
pub type OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EN`"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CTDAC_RANGE`"]
pub type CTDAC_RANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTDAC_RANGE`"]
pub struct CTDAC_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDAC_RANGE_W<'a> {
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
#[doc = "DAC mode, this determines the Value decoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTDAC_MODE_A {
    #[doc = "0: Unsigned 12-bit VDAC, i.e. no value decoding."]
    UNSIGNED12 = 0,
    #[doc = "1: Virtual signed 12-bits' VDAC. Value decoding:\nadd 0x800 to the 12-bit Value (=invert MSB), to convert the lowest signed number 0x800 to the lowest unsigned number 0x000. This is the same as the SAR handles 12-bit 'virtual' signed numbers."]
    VIRT_SIGNED12 = 1,
    #[doc = "2: N/A"]
    RSVD2 = 2,
    #[doc = "3: N/A"]
    RSVD3 = 3,
}
impl From<CTDAC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTDAC_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CTDAC_MODE`"]
pub type CTDAC_MODE_R = crate::R<u8, CTDAC_MODE_A>;
impl CTDAC_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTDAC_MODE_A {
        match self.bits {
            0 => CTDAC_MODE_A::UNSIGNED12,
            1 => CTDAC_MODE_A::VIRT_SIGNED12,
            2 => CTDAC_MODE_A::RSVD2,
            3 => CTDAC_MODE_A::RSVD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED12`"]
    #[inline(always)]
    pub fn is_unsigned12(&self) -> bool {
        *self == CTDAC_MODE_A::UNSIGNED12
    }
    #[doc = "Checks if the value of the field is `VIRT_SIGNED12`"]
    #[inline(always)]
    pub fn is_virt_signed12(&self) -> bool {
        *self == CTDAC_MODE_A::VIRT_SIGNED12
    }
    #[doc = "Checks if the value of the field is `RSVD2`"]
    #[inline(always)]
    pub fn is_rsvd2(&self) -> bool {
        *self == CTDAC_MODE_A::RSVD2
    }
    #[doc = "Checks if the value of the field is `RSVD3`"]
    #[inline(always)]
    pub fn is_rsvd3(&self) -> bool {
        *self == CTDAC_MODE_A::RSVD3
    }
}
#[doc = "Write proxy for field `CTDAC_MODE`"]
pub struct CTDAC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDAC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTDAC_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Unsigned 12-bit VDAC, i.e. no value decoding."]
    #[inline(always)]
    pub fn unsigned12(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::UNSIGNED12)
    }
    #[doc = "Virtual signed 12-bits' VDAC. Value decoding: add 0x800 to the 12-bit Value (=invert MSB), to convert the lowest signed number 0x800 to the lowest unsigned number 0x000. This is the same as the SAR handles 12-bit 'virtual' signed numbers."]
    #[inline(always)]
    pub fn virt_signed12(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::VIRT_SIGNED12)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd2(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::RSVD2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd3(self) -> &'a mut W {
        self.variant(CTDAC_MODE_A::RSVD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DISABLED_MODE`"]
pub type DISABLED_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLED_MODE`"]
pub struct DISABLED_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DSI_STROBE_EN`"]
pub type DSI_STROBE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_STROBE_EN`"]
pub struct DSI_STROBE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_STROBE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DSI_STROBE_LEVEL`"]
pub type DSI_STROBE_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_STROBE_LEVEL`"]
pub struct DSI_STROBE_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_STROBE_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DEEPSLEEP_ON`"]
pub type DEEPSLEEP_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEEPSLEEP_ON`"]
pub struct DEEPSLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&self) -> DEGLITCH_CNT_R {
        DEGLITCH_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&self) -> DEGLITCH_CO6_R {
        DEGLITCH_CO6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&self) -> DEGLITCH_COS_R {
        DEGLITCH_COS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\] * Vref / 4096 1: Range is \\[1, 4096\\] * Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&self) -> CTDAC_RANGE_R {
        CTDAC_RANGE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&self) -> CTDAC_MODE_R {
        CTDAC_MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&self) -> DISABLED_MODE_R {
        DISABLED_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if alllowed by the DSI stobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&self) -> DSI_STROBE_EN_R {
        DSI_STROBE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&self) -> DSI_STROBE_LEVEL_R {
        DSI_STROBE_LEVEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&mut self) -> DEGLITCH_CNT_W {
        DEGLITCH_CNT_W { w: self }
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&mut self) -> DEGLITCH_CO6_W {
        DEGLITCH_CO6_W { w: self }
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&mut self) -> DEGLITCH_COS_W {
        DEGLITCH_COS_W { w: self }
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\] * Vref / 4096 1: Range is \\[1, 4096\\] * Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&mut self) -> CTDAC_RANGE_W {
        CTDAC_RANGE_W { w: self }
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&mut self) -> CTDAC_MODE_W {
        CTDAC_MODE_W { w: self }
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&mut self) -> DISABLED_MODE_W {
        DISABLED_MODE_W { w: self }
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if alllowed by the DSI stobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&mut self) -> DSI_STROBE_EN_W {
        DSI_STROBE_EN_W { w: self }
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&mut self) -> DSI_STROBE_LEVEL_W {
        DSI_STROBE_LEVEL_W { w: self }
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W { w: self }
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
