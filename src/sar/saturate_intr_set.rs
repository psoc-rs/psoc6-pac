#[doc = "Register `SATURATE_INTR_SET` reader"]
pub struct R(crate::R<SATURATE_INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SATURATE_INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SATURATE_INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SATURATE_INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SATURATE_INTR_SET` writer"]
pub struct W(crate::W<SATURATE_INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SATURATE_INTR_SET_SPEC>;
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
impl From<crate::W<SATURATE_INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SATURATE_INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SATURATE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SATURATE_SET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SATURATE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SATURATE_SET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SATURATE_INTR_SET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_set(&self) -> SATURATE_SET_R {
        SATURATE_SET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_set(&mut self) -> SATURATE_SET_W<0> {
        SATURATE_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Saturate interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_set](index.html) module"]
pub struct SATURATE_INTR_SET_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saturate_intr_set::R](R) reader structure"]
impl crate::Readable for SATURATE_INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saturate_intr_set::W](W) writer structure"]
impl crate::Writable for SATURATE_INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SATURATE_INTR_SET to value 0"]
impl crate::Resettable for SATURATE_INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
