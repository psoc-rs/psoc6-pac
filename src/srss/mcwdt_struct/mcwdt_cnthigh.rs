#[doc = "Register `MCWDT_CNTHIGH` reader"]
pub struct R(crate::R<MCWDT_CNTHIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_CNTHIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_CNTHIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_CNTHIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_CNTHIGH` writer"]
pub struct W(crate::W<MCWDT_CNTHIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_CNTHIGH_SPEC>;
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
impl From<crate::W<MCWDT_CNTHIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_CNTHIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CTR2` reader - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_CTR2` writer - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
pub type WDT_CTR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCWDT_CNTHIGH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&self) -> WDT_CTR2_R {
        WDT_CTR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current value of sub-counter 2 for this MCWDT. Software writes are ignored when the sub-counter is enabled"]
    #[inline(always)]
    pub fn wdt_ctr2(&mut self) -> WDT_CTR2_W<0> {
        WDT_CTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Sub-counter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_cnthigh](index.html) module"]
pub struct MCWDT_CNTHIGH_SPEC;
impl crate::RegisterSpec for MCWDT_CNTHIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_cnthigh::R](R) reader structure"]
impl crate::Readable for MCWDT_CNTHIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_cnthigh::W](W) writer structure"]
impl crate::Writable for MCWDT_CNTHIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_CNTHIGH to value 0"]
impl crate::Resettable for MCWDT_CNTHIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
