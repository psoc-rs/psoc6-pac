#[doc = "Register `INTR_SIE` reader"]
pub struct R(crate::R<INTR_SIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SIE` writer"]
pub struct W(crate::W<INTR_SIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SIE_SPEC>;
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
impl From<crate::W<INTR_SIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF_INTR` reader - Interrupt status for USB SOF"]
pub type SOF_INTR_R = crate::BitReader<bool>;
#[doc = "Field `SOF_INTR` writer - Interrupt status for USB SOF"]
pub type SOF_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SPEC, bool, O>;
#[doc = "Field `BUS_RESET_INTR` reader - Interrupt status for BUS RESET"]
pub type BUS_RESET_INTR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_RESET_INTR` writer - Interrupt status for BUS RESET"]
pub type BUS_RESET_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SPEC, bool, O>;
#[doc = "Field `EP0_INTR` reader - Interrupt status for EP0"]
pub type EP0_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INTR` writer - Interrupt status for EP0"]
pub type EP0_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SPEC, bool, O>;
#[doc = "Field `LPM_INTR` reader - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LPM_INTR_R = crate::BitReader<bool>;
#[doc = "Field `LPM_INTR` writer - Interrupt status for LPM (Link Power Management, L1 entry)"]
pub type LPM_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SPEC, bool, O>;
#[doc = "Field `RESUME_INTR` reader - Interrupt status for Resume"]
pub type RESUME_INTR_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_INTR` writer - Interrupt status for Resume"]
pub type RESUME_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SOF_INTR_R {
        SOF_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BUS_RESET_INTR_R {
        BUS_RESET_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> EP0_INTR_R {
        EP0_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LPM_INTR_R {
        LPM_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&self) -> RESUME_INTR_R {
        RESUME_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&mut self) -> SOF_INTR_W<0> {
        SOF_INTR_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&mut self) -> BUS_RESET_INTR_W<1> {
        BUS_RESET_INTR_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&mut self) -> EP0_INTR_W<2> {
        EP0_INTR_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&mut self) -> LPM_INTR_W<3> {
        LPM_INTR_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&mut self) -> RESUME_INTR_W<4> {
        RESUME_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie](index.html) module"]
pub struct INTR_SIE_SPEC;
impl crate::RegisterSpec for INTR_SIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_sie::R](R) reader structure"]
impl crate::Readable for INTR_SIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_sie::W](W) writer structure"]
impl crate::Writable for INTR_SIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SIE to value 0"]
impl crate::Resettable for INTR_SIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
