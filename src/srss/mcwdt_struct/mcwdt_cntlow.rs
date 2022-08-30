#[doc = "Register `MCWDT_CNTLOW` reader"]
pub struct R(crate::R<MCWDT_CNTLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_CNTLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_CNTLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_CNTLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_CNTLOW` writer"]
pub struct W(crate::W<MCWDT_CNTLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_CNTLOW_SPEC>;
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
impl From<crate::W<MCWDT_CNTLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_CNTLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CTR0` reader - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WDT_CTR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_CTR0` writer - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
pub type WDT_CTR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCWDT_CNTLOW_SPEC, u16, u16, 16, O>;
#[doc = "Field `WDT_CTR1` reader - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_CTR1` writer - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCWDT_CNTLOW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&self) -> WDT_CTR0_R {
        WDT_CTR0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&self) -> WDT_CTR1_R {
        WDT_CTR1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of sub-counter 0 for this MCWDT. Software writes are ignored when the sub-counter is enabled."]
    #[inline(always)]
    pub fn wdt_ctr0(&mut self) -> WDT_CTR0_W<0> {
        WDT_CTR0_W::new(self)
    }
    #[doc = "Bits 16:31 - Current value of sub-counter 1 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr1(&mut self) -> WDT_CTR1_W<16> {
        WDT_CTR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Sub-counters 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_cntlow](index.html) module"]
pub struct MCWDT_CNTLOW_SPEC;
impl crate::RegisterSpec for MCWDT_CNTLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_cntlow::R](R) reader structure"]
impl crate::Readable for MCWDT_CNTLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_cntlow::W](W) writer structure"]
impl crate::Writable for MCWDT_CNTLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_CNTLOW to value 0"]
impl crate::Resettable for MCWDT_CNTLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
