#[doc = "Register `ARB_RW1_DR` reader"]
pub struct R(crate::R<ARB_RW1_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW1_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW1_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW1_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW1_DR` writer"]
pub struct W(crate::W<ARB_RW1_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW1_DR_SPEC>;
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
impl From<crate::W<ARB_RW1_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW1_DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DR` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARB_RW1_DR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<0> {
        DR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw1_dr](index.html) module"]
pub struct ARB_RW1_DR_SPEC;
impl crate::RegisterSpec for ARB_RW1_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw1_dr::R](R) reader structure"]
impl crate::Readable for ARB_RW1_DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw1_dr::W](W) writer structure"]
impl crate::Writable for ARB_RW1_DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_RW1_DR to value 0"]
impl crate::Resettable for ARB_RW1_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
