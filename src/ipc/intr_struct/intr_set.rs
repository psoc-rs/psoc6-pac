#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELEASE` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type RELEASE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELEASE` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type RELEASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_SET_SPEC, u16, u16, 16, O>;
#[doc = "Field `NOTIFY` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type NOTIFY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NOTIFY` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type NOTIFY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_SET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn release(&self) -> RELEASE_R {
        RELEASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn notify(&self) -> NOTIFY_R {
        NOTIFY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W<0> {
        RELEASE_W::new(self)
    }
    #[doc = "Bits 16:31 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn notify(&mut self) -> NOTIFY_W<16> {
        NOTIFY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
