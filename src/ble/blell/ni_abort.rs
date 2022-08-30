#[doc = "Register `NI_ABORT` reader"]
pub struct R(crate::R<NI_ABORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NI_ABORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NI_ABORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NI_ABORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NI_ABORT` writer"]
pub struct W(crate::W<NI_ABORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NI_ABORT_SPEC>;
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
impl From<crate::W<NI_ABORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NI_ABORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NI_ABORT` reader - Setting this bit clears the schedule NI"]
pub type NI_ABORT_R = crate::BitReader<bool>;
#[doc = "Field `NI_ABORT` writer - Setting this bit clears the schedule NI"]
pub type NI_ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NI_ABORT_SPEC, bool, O>;
#[doc = "Field `ABORT_ACK` reader - This bit will set if the scheduled NI is aborted"]
pub type ABORT_ACK_R = crate::BitReader<bool>;
#[doc = "Field `ABORT_ACK` writer - This bit will set if the scheduled NI is aborted"]
pub type ABORT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NI_ABORT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit clears the schedule NI"]
    #[inline(always)]
    pub fn ni_abort(&self) -> NI_ABORT_R {
        NI_ABORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit will set if the scheduled NI is aborted"]
    #[inline(always)]
    pub fn abort_ack(&self) -> ABORT_ACK_R {
        ABORT_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit clears the schedule NI"]
    #[inline(always)]
    pub fn ni_abort(&mut self) -> NI_ABORT_W<0> {
        NI_ABORT_W::new(self)
    }
    #[doc = "Bit 1 - This bit will set if the scheduled NI is aborted"]
    #[inline(always)]
    pub fn abort_ack(&mut self) -> ABORT_ACK_W<1> {
        ABORT_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Abort next scheduled connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ni_abort](index.html) module"]
pub struct NI_ABORT_SPEC;
impl crate::RegisterSpec for NI_ABORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ni_abort::R](R) reader structure"]
impl crate::Readable for NI_ABORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ni_abort::W](W) writer structure"]
impl crate::Writable for NI_ABORT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NI_ABORT to value 0"]
impl crate::Resettable for NI_ABORT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
