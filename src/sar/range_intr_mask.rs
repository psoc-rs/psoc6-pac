#[doc = "Register `RANGE_INTR_MASK` reader"]
pub struct R(crate::R<RANGE_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANGE_INTR_MASK` writer"]
pub struct W(crate::W<RANGE_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANGE_INTR_MASK_SPEC>;
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
impl From<crate::W<RANGE_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANGE_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RANGE_MASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RANGE_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RANGE_INTR_MASK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_mask(&self) -> RANGE_MASK_R {
        RANGE_MASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_mask(&mut self) -> RANGE_MASK_W<0> {
        RANGE_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Range detect interrupt mask register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_intr_mask](index.html) module"]
pub struct RANGE_INTR_MASK_SPEC;
impl crate::RegisterSpec for RANGE_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_intr_mask::R](R) reader structure"]
impl crate::Readable for RANGE_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [range_intr_mask::W](W) writer structure"]
impl crate::Writable for RANGE_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RANGE_INTR_MASK to value 0"]
impl crate::Resettable for RANGE_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
