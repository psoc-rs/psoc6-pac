#[doc = "Register `LE_PING_TIMER_ADDR` reader"]
pub struct R(crate::R<LE_PING_TIMER_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_PING_TIMER_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_PING_TIMER_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_PING_TIMER_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LE_PING_TIMER_ADDR` writer"]
pub struct W(crate::W<LE_PING_TIMER_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LE_PING_TIMER_ADDR_SPEC>;
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
impl From<crate::W<LE_PING_TIMER_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LE_PING_TIMER_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_PING_TIMER_ADDR` reader - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
pub type CONN_PING_TIMER_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONN_PING_TIMER_ADDR` writer - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
pub type CONN_PING_TIMER_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LE_PING_TIMER_ADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
    #[inline(always)]
    pub fn conn_ping_timer_addr(&self) -> CONN_PING_TIMER_ADDR_R {
        CONN_PING_TIMER_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
    #[inline(always)]
    pub fn conn_ping_timer_addr(&mut self) -> CONN_PING_TIMER_ADDR_W<0> {
        CONN_PING_TIMER_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LE Ping connection timer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_ping_timer_addr](index.html) module"]
pub struct LE_PING_TIMER_ADDR_SPEC;
impl crate::RegisterSpec for LE_PING_TIMER_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le_ping_timer_addr::R](R) reader structure"]
impl crate::Readable for LE_PING_TIMER_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [le_ping_timer_addr::W](W) writer structure"]
impl crate::Writable for LE_PING_TIMER_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LE_PING_TIMER_ADDR to value 0"]
impl crate::Resettable for LE_PING_TIMER_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
