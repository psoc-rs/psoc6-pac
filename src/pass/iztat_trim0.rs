#[doc = "Register `IZTAT_TRIM0` reader"]
pub struct R(crate::R<IZTAT_TRIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IZTAT_TRIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IZTAT_TRIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IZTAT_TRIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IZTAT_TRIM0` writer"]
pub struct W(crate::W<IZTAT_TRIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IZTAT_TRIM0_SPEC>;
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
impl From<crate::W<IZTAT_TRIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IZTAT_TRIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IZTAT_ABS_TRIM` reader - N/A"]
pub type IZTAT_ABS_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IZTAT_ABS_TRIM` writer - N/A"]
pub type IZTAT_ABS_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IZTAT_TRIM0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn iztat_abs_trim(&self) -> IZTAT_ABS_TRIM_R {
        IZTAT_ABS_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn iztat_abs_trim(&mut self) -> IZTAT_ABS_TRIM_W<0> {
        IZTAT_ABS_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IZTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iztat_trim0](index.html) module"]
pub struct IZTAT_TRIM0_SPEC;
impl crate::RegisterSpec for IZTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iztat_trim0::R](R) reader structure"]
impl crate::Readable for IZTAT_TRIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iztat_trim0::W](W) writer structure"]
impl crate::Writable for IZTAT_TRIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IZTAT_TRIM0 to value 0"]
impl crate::Resettable for IZTAT_TRIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
