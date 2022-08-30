#[doc = "Register `WDT_MATCH` reader"]
pub struct R(crate::R<WDT_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_MATCH` writer"]
pub struct W(crate::W<WDT_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_MATCH_SPEC>;
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
impl From<crate::W<WDT_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH` writer - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
pub type MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDT_MATCH_SPEC, u16, u16, 16, O>;
#[doc = "Field `IGNORE_BITS` reader - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IGNORE_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IGNORE_BITS` writer - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
pub type IGNORE_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDT_MATCH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&self) -> IGNORE_BITS_R {
        IGNORE_BITS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for Watchdog counter. Every time WDT_COUNTER reaches MATCH an interrupt is generated. Two unserviced interrupts will lead to a system reset (i.e. at the third match)."]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    #[doc = "Bits 16:19 - The number of MSB bits of the watchdog timer that are NOT checked against MATCH. This value provides control over the time-to-reset of the watchdog (which happens after 3 successive matches). Up to 12 MSB can be ignored. Settings >12 behave like a setting of 12."]
    #[inline(always)]
    pub fn ignore_bits(&mut self) -> IGNORE_BITS_W<16> {
        IGNORE_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counter Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_match](index.html) module"]
pub struct WDT_MATCH_SPEC;
impl crate::RegisterSpec for WDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_match::R](R) reader structure"]
impl crate::Readable for WDT_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_match::W](W) writer structure"]
impl crate::Writable for WDT_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_MATCH to value 0x1000"]
impl crate::Resettable for WDT_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
