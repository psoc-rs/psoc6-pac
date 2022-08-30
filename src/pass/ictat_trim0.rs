#[doc = "Register `ICTAT_TRIM0` reader"]
pub struct R(crate::R<ICTAT_TRIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICTAT_TRIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICTAT_TRIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICTAT_TRIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICTAT_TRIM0` writer"]
pub struct W(crate::W<ICTAT_TRIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICTAT_TRIM0_SPEC>;
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
impl From<crate::W<ICTAT_TRIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICTAT_TRIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICTAT_TRIM` reader - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type ICTAT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICTAT_TRIM` writer - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type ICTAT_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ICTAT_TRIM0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&self) -> ICTAT_TRIM_R {
        ICTAT_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&mut self) -> ICTAT_TRIM_W<0> {
        ICTAT_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICTAT Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictat_trim0](index.html) module"]
pub struct ICTAT_TRIM0_SPEC;
impl crate::RegisterSpec for ICTAT_TRIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ictat_trim0::R](R) reader structure"]
impl crate::Readable for ICTAT_TRIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ictat_trim0::W](W) writer structure"]
impl crate::Writable for ICTAT_TRIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICTAT_TRIM0 to value 0"]
impl crate::Resettable for ICTAT_TRIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
