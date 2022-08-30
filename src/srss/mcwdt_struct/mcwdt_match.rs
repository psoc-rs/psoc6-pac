#[doc = "Register `MCWDT_MATCH` reader"]
pub struct R(crate::R<MCWDT_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_MATCH` writer"]
pub struct W(crate::W<MCWDT_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_MATCH_SPEC>;
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
impl From<crate::W<MCWDT_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_MATCH0` reader - Match value for sub-counter 0 of this MCWDT"]
pub type WDT_MATCH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_MATCH0` writer - Match value for sub-counter 0 of this MCWDT"]
pub type WDT_MATCH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCWDT_MATCH_SPEC, u16, u16, 16, O>;
#[doc = "Field `WDT_MATCH1` reader - Match value for sub-counter 1 of this MCWDT"]
pub type WDT_MATCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_MATCH1` writer - Match value for sub-counter 1 of this MCWDT"]
pub type WDT_MATCH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCWDT_MATCH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&self) -> WDT_MATCH0_R {
        WDT_MATCH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&self) -> WDT_MATCH1_R {
        WDT_MATCH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for sub-counter 0 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match0(&mut self) -> WDT_MATCH0_W<0> {
        WDT_MATCH0_W::new(self)
    }
    #[doc = "Bits 16:31 - Match value for sub-counter 1 of this MCWDT"]
    #[inline(always)]
    pub fn wdt_match1(&mut self) -> WDT_MATCH1_W<16> {
        WDT_MATCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Counter Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_match](index.html) module"]
pub struct MCWDT_MATCH_SPEC;
impl crate::RegisterSpec for MCWDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_match::R](R) reader structure"]
impl crate::Readable for MCWDT_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_match::W](W) writer structure"]
impl crate::Writable for MCWDT_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_MATCH to value 0"]
impl crate::Resettable for MCWDT_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
