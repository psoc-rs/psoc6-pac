#[doc = "Register `RX_MATCH` reader"]
pub struct R(crate::R<RX_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_MATCH` writer"]
pub struct W(crate::W<RX_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_MATCH_SPEC>;
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
impl From<crate::W<RX_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - N/A"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - N/A"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_MATCH_SPEC, u8, u8, 8, O>;
#[doc = "Field `MASK` reader - N/A"]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - N/A"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_MATCH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<16> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address and mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_match](index.html) module"]
pub struct RX_MATCH_SPEC;
impl crate::RegisterSpec for RX_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_match::R](R) reader structure"]
impl crate::Readable for RX_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_match::W](W) writer structure"]
impl crate::Writable for RX_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_MATCH to value 0"]
impl crate::Resettable for RX_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
