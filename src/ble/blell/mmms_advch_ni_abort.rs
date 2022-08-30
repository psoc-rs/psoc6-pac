#[doc = "Register `MMMS_ADVCH_NI_ABORT` reader"]
pub struct R(crate::R<MMMS_ADVCH_NI_ABORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_ADVCH_NI_ABORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_ADVCH_NI_ABORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_ADVCH_NI_ABORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_ADVCH_NI_ABORT` writer"]
pub struct W(crate::W<MMMS_ADVCH_NI_ABORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_ADVCH_NI_ABORT_SPEC>;
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
impl From<crate::W<MMMS_ADVCH_NI_ABORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_ADVCH_NI_ABORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADVCH_NI_ABORT` writer - FW can use this bit to clear an unserviced NI_VALID for Advertisement or scanner or initiator. HW will clear NI_VALID for ADV/SCAN/INIT if the event has not yet started"]
pub type ADVCH_NI_ABORT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_ADVCH_NI_ABORT_SPEC, bool, O>;
#[doc = "Field `ADVCH_ABORT_STATUS` reader - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
pub type ADVCH_ABORT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `ADVCH_ABORT_STATUS` writer - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
pub type ADVCH_ABORT_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_ADVCH_NI_ABORT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
    #[inline(always)]
    pub fn advch_abort_status(&self) -> ADVCH_ABORT_STATUS_R {
        ADVCH_ABORT_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FW can use this bit to clear an unserviced NI_VALID for Advertisement or scanner or initiator. HW will clear NI_VALID for ADV/SCAN/INIT if the event has not yet started"]
    #[inline(always)]
    pub fn advch_ni_abort(&mut self) -> ADVCH_NI_ABORT_W<0> {
        ADVCH_NI_ABORT_W::new(self)
    }
    #[doc = "Bit 1 - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
    #[inline(always)]
    pub fn advch_abort_status(&mut self) -> ADVCH_ABORT_STATUS_W<1> {
        ADVCH_ABORT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Abort the next instant of ADV, SCAN, INIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_abort](index.html) module"]
pub struct MMMS_ADVCH_NI_ABORT_SPEC;
impl crate::RegisterSpec for MMMS_ADVCH_NI_ABORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_advch_ni_abort::R](R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_ABORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_abort::W](W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_ABORT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_ADVCH_NI_ABORT to value 0"]
impl crate::Resettable for MMMS_ADVCH_NI_ABORT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
