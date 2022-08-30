#[doc = "Register `TIMER_CTL` reader"]
pub struct R(crate::R<TIMER_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_CTL` writer"]
pub struct W(crate::W<TIMER_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CTL_SPEC>;
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
impl From<crate::W<TIMER_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERIOD` writer - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `SCALE` reader - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
pub type SCALE_R = crate::BitReader<bool>;
#[doc = "Field `SCALE` writer - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
pub type SCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `PUMP_CLOCK_SEL` reader - Pump clock select: '0': internal clock. '1': external clock."]
pub type PUMP_CLOCK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PUMP_CLOCK_SEL` writer - Pump clock select: '0': internal clock. '1': external clock."]
pub type PUMP_CLOCK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `PRE_PROG` reader - '1' during pre-program operation"]
pub type PRE_PROG_R = crate::BitReader<bool>;
#[doc = "Field `PRE_PROG` writer - '1' during pre-program operation"]
pub type PRE_PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `PRE_PROG_CSL` reader - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
pub type PRE_PROG_CSL_R = crate::BitReader<bool>;
#[doc = "Field `PRE_PROG_CSL` writer - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
pub type PRE_PROG_CSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `PUMP_EN` reader - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PUMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PUMP_EN` writer - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PUMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `ACLK_EN` reader - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type ACLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACLK_EN` writer - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type ACLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
#[doc = "Field `TIMER_EN` reader - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TIMER_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_EN` writer - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TIMER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&self) -> PUMP_CLOCK_SEL_R {
        PUMP_CLOCK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&self) -> PRE_PROG_R {
        PRE_PROG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&self) -> PRE_PROG_CSL_R {
        PRE_PROG_CSL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&self) -> PUMP_EN_R {
        PUMP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&self) -> ACLK_EN_R {
        ACLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Bit 16 - Timer tick scale: '0': 1 microsecond. '1': 100 microseconds."]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W<16> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 24 - Pump clock select: '0': internal clock. '1': external clock."]
    #[inline(always)]
    pub fn pump_clock_sel(&mut self) -> PUMP_CLOCK_SEL_W<24> {
        PUMP_CLOCK_SEL_W::new(self)
    }
    #[doc = "Bit 25 - '1' during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&mut self) -> PRE_PROG_W<25> {
        PRE_PROG_W::new(self)
    }
    #[doc = "Bit 26 - '0' CSL lines driven by CSL_DAC '1' CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&mut self) -> PRE_PROG_CSL_W<26> {
        PRE_PROG_CSL_W::new(self)
    }
    #[doc = "Bit 29 - Pump enable: '0': disabled '1': enabled (also requires FM_CTL.IF_SEL to be '1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&mut self) -> PUMP_EN_W<29> {
        PUMP_EN_W::new(self)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): '0': disabled '1': enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&mut self) -> ACLK_EN_W<30> {
        ACLK_EN_W::new(self)
    }
    #[doc = "Bit 31 - Timer enable: '0': disabled '1': enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W<31> {
        TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_ctl](index.html) module"]
pub struct TIMER_CTL_SPEC;
impl crate::RegisterSpec for TIMER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_ctl::R](R) reader structure"]
impl crate::Readable for TIMER_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_ctl::W](W) writer structure"]
impl crate::Writable for TIMER_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_CTL to value 0x0400_0000"]
impl crate::Resettable for TIMER_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
