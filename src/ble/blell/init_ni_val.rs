#[doc = "Register `INIT_NI_VAL` reader"]
pub struct R(crate::R<INIT_NI_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_NI_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_NI_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_NI_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_NI_VAL` writer"]
pub struct W(crate::W<INIT_NI_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_NI_VAL_SPEC>;
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
impl From<crate::W<INIT_NI_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_NI_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_NI_VAL` reader - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
pub type INIT_NI_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT_NI_VAL` writer - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
pub type INIT_NI_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INIT_NI_VAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
    #[inline(always)]
    pub fn init_ni_val(&self) -> INIT_NI_VAL_R {
        INIT_NI_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
    #[inline(always)]
    pub fn init_ni_val(&mut self) -> INIT_NI_VAL_W<0> {
        INIT_NI_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initiator Window NI instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_ni_val](index.html) module"]
pub struct INIT_NI_VAL_SPEC;
impl crate::RegisterSpec for INIT_NI_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_ni_val::R](R) reader structure"]
impl crate::Readable for INIT_NI_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_ni_val::W](W) writer structure"]
impl crate::Writable for INIT_NI_VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_NI_VAL to value 0"]
impl crate::Resettable for INIT_NI_VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
