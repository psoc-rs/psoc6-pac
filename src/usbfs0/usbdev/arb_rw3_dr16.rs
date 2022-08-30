#[doc = "Register `ARB_RW3_DR16` reader"]
pub struct R(crate::R<ARB_RW3_DR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_RW3_DR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_RW3_DR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_RW3_DR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARB_RW3_DR16` writer"]
pub struct W(crate::W<ARB_RW3_DR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARB_RW3_DR16_SPEC>;
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
impl From<crate::W<ARB_RW3_DR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARB_RW3_DR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR16` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DR16` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type DR16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARB_RW3_DR16_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&mut self) -> DR16_W<0> {
        DR16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_rw3_dr16](index.html) module"]
pub struct ARB_RW3_DR16_SPEC;
impl crate::RegisterSpec for ARB_RW3_DR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_rw3_dr16::R](R) reader structure"]
impl crate::Readable for ARB_RW3_DR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arb_rw3_dr16::W](W) writer structure"]
impl crate::Writable for ARB_RW3_DR16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARB_RW3_DR16 to value 0"]
impl crate::Resettable for ARB_RW3_DR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
