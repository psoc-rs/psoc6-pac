#[doc = "Register `CTDAC_CTRL` reader"]
pub struct R(crate::R<CTDAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTDAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTDAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTDAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTDAC_CTRL` writer"]
pub struct W(crate::W<CTDAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTDAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTDAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTDAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEGLITCH_CNT` reader - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
pub type DEGLITCH_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEGLITCH_CNT` writer - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
pub type DEGLITCH_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTDAC_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `DEGLITCH_CO6` reader - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
pub type DEGLITCH_CO6_R = crate::BitReader<bool>;
#[doc = "Field `DEGLITCH_CO6` writer - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
pub type DEGLITCH_CO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `DEGLITCH_COS` reader - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
pub type DEGLITCH_COS_R = crate::BitReader<bool>;
#[doc = "Field `DEGLITCH_COS` writer - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
pub type DEGLITCH_COS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `OUT_EN` reader - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
pub type OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EN` writer - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
pub type OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `CTDAC_RANGE` reader - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
pub type CTDAC_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `CTDAC_RANGE` writer - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
pub type CTDAC_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "DAC mode, this determines the Value decoding\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTDAC_MODE_A {
    #[doc = "0: Unsigned 12-bit VDAC, i.e. no value decoding."]
    UNSIGNED12 = 0,
    #[doc = "1: Virtual signed 12-bits' VDAC. Value decoding: add 0x800 to the 12-bit Value (=invert MSB), to convert the lowest signed number 0x800 to the lowest unsigned number 0x000. This is the same as the SAR handles 12-bit 'virtual' signed numbers."]
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
#[doc = "Field `CTDAC_MODE` reader - DAC mode, this determines the Value decoding"]
pub type CTDAC_MODE_R = crate::FieldReader<u8, CTDAC_MODE_A>;
impl CTDAC_MODE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `CTDAC_MODE` writer - DAC mode, this determines the Value decoding"]
pub type CTDAC_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTDAC_CTRL_SPEC, u8, CTDAC_MODE_A, 2, O>;
impl<'a, const O: u8> CTDAC_MODE_W<'a, O> {
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
}
#[doc = "Field `DISABLED_MODE` reader - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
pub type DISABLED_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DISABLED_MODE` writer - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
pub type DISABLED_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `DSI_STROBE_EN` reader - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
pub type DSI_STROBE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_STROBE_EN` writer - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
pub type DSI_STROBE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `DSI_STROBE_LEVEL` reader - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
pub type DSI_STROBE_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `DSI_STROBE_LEVEL` writer - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
pub type DSI_STROBE_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTDAC_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&self) -> DEGLITCH_CNT_R {
        DEGLITCH_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&self) -> DEGLITCH_CO6_R {
        DEGLITCH_CO6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&self) -> DEGLITCH_COS_R {
        DEGLITCH_COS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&self) -> CTDAC_RANGE_R {
        CTDAC_RANGE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&self) -> CTDAC_MODE_R {
        CTDAC_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&self) -> DISABLED_MODE_R {
        DISABLED_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&self) -> DSI_STROBE_EN_R {
        DSI_STROBE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&self) -> DSI_STROBE_LEVEL_R {
        DSI_STROBE_LEVEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - To prevent glitches after VALUE changes from propagating the output switch can be opened for DEGLITCH_CNT+1 clk_peri clock cycles."]
    #[inline(always)]
    pub fn deglitch_cnt(&mut self) -> DEGLITCH_CNT_W<0> {
        DEGLITCH_CNT_W::new(self)
    }
    #[doc = "Bit 8 - Force CTDAC.CO6 switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_co6(&mut self) -> DEGLITCH_CO6_W<8> {
        DEGLITCH_CO6_W::new(self)
    }
    #[doc = "Bit 9 - Force CTB.COS switch open after each VALUE change for the set number of clock cycles."]
    #[inline(always)]
    pub fn deglitch_cos(&mut self) -> DEGLITCH_COS_W<9> {
        DEGLITCH_COS_W::new(self)
    }
    #[doc = "Bit 22 - Output enable, intended to be used during the Hold phase of the Sample and Hold when power cycling : 0: output disabled, the output is either: - Tri-state (DISABLED_MODE=0) - or Vssa (DISABLED_MODE=1 && CTDAC_RANGE=0) - or Vref (DISABLED_MODE=1 && CTDAC_RANGE=1) 1: output enabled, CTDAC output drives the programmed VALUE"]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W<22> {
        OUT_EN_W::new(self)
    }
    #[doc = "Bit 23 - By closing the bottom switch in the R2R network the output is lifted by one LSB, effectively adding 1 0: Range is \\[0, 4095\\]
* Vref / 4096 1: Range is \\[1, 4096\\]
* Vref / 4096"]
    #[inline(always)]
    pub fn ctdac_range(&mut self) -> CTDAC_RANGE_W<23> {
        CTDAC_RANGE_W::new(self)
    }
    #[doc = "Bits 24:25 - DAC mode, this determines the Value decoding"]
    #[inline(always)]
    pub fn ctdac_mode(&mut self) -> CTDAC_MODE_W<24> {
        CTDAC_MODE_W::new(self)
    }
    #[doc = "Bit 27 - Select the output value when the output is disabled (OUT_EN=0) (for risk mitigation) 0: Tri-state CTDAC output when disabled 1: output Vssa or Vref when disabled (see OUT_EN description)"]
    #[inline(always)]
    pub fn disabled_mode(&mut self) -> DISABLED_MODE_W<27> {
        DISABLED_MODE_W::new(self)
    }
    #[doc = "Bit 28 - DSI strobe input Enable. This enables CTDAC updates to be further throttled by DSI. 0: Ignore DSI strobe input 1: Only do a CTDAC update if allowed by the DSI strobe (throttle), see below for level or edge"]
    #[inline(always)]
    pub fn dsi_strobe_en(&mut self) -> DSI_STROBE_EN_W<28> {
        DSI_STROBE_EN_W::new(self)
    }
    #[doc = "Bit 29 - Select level or edge detect for DSI strobe - 0: DSI strobe signal is a pulse input, after a positive edge is detected on the DSI strobe signal the next DAC value update is done on the next CTDAC clock - 1: DSI strobe signal is a level input, as long as the DSI strobe signal remains high the CTDAC will do a next DAC value update on each CTDAC clock."]
    #[inline(always)]
    pub fn dsi_strobe_level(&mut self) -> DSI_STROBE_LEVEL_W<29> {
        DSI_STROBE_LEVEL_W::new(self)
    }
    #[doc = "Bit 30 - - 0: CTDAC IP disabled off during DeepSleep power mode - 1: CTDAC IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W<30> {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 31 - 0: CTDAC IP disabled (put analog in power down, open all switches) 1: CTDAC IP enabled"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CTDAC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_ctrl](index.html) module"]
pub struct CTDAC_CTRL_SPEC;
impl crate::RegisterSpec for CTDAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctdac_ctrl::R](R) reader structure"]
impl crate::Readable for CTDAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctdac_ctrl::W](W) writer structure"]
impl crate::Writable for CTDAC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTDAC_CTRL to value 0"]
impl crate::Resettable for CTDAC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
