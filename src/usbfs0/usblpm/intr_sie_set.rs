#[doc = "Register `INTR_SIE_SET` reader"]
pub struct R(crate::R<INTR_SIE_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SIE_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SIE_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SIE_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SIE_SET` writer"]
pub struct W(crate::W<INTR_SIE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SIE_SET_SPEC>;
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
impl From<crate::W<INTR_SIE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SIE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SOF_INTR_SET_R = crate::BitReader<bool>;
#[doc = "Field `SOF_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SOF_INTR_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SET_SPEC, bool, O>;
#[doc = "Field `BUS_RESET_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type BUS_RESET_INTR_SET_R = crate::BitReader<bool>;
#[doc = "Field `BUS_RESET_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type BUS_RESET_INTR_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_SIE_SET_SPEC, bool, O>;
#[doc = "Field `EP0_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EP0_INTR_SET_R = crate::BitReader<bool>;
#[doc = "Field `EP0_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EP0_INTR_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SET_SPEC, bool, O>;
#[doc = "Field `LPM_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type LPM_INTR_SET_R = crate::BitReader<bool>;
#[doc = "Field `LPM_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type LPM_INTR_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SET_SPEC, bool, O>;
#[doc = "Field `RESUME_INTR_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RESUME_INTR_SET_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_INTR_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RESUME_INTR_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SIE_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&self) -> SOF_INTR_SET_R {
        SOF_INTR_SET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&self) -> BUS_RESET_INTR_SET_R {
        BUS_RESET_INTR_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&self) -> EP0_INTR_SET_R {
        EP0_INTR_SET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&self) -> LPM_INTR_SET_R {
        LPM_INTR_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&self) -> RESUME_INTR_SET_R {
        RESUME_INTR_SET_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&mut self) -> SOF_INTR_SET_W<0> {
        SOF_INTR_SET_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&mut self) -> BUS_RESET_INTR_SET_W<1> {
        BUS_RESET_INTR_SET_W::new(self)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&mut self) -> EP0_INTR_SET_W<2> {
        EP0_INTR_SET_W::new(self)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&mut self) -> LPM_INTR_SET_W<3> {
        LPM_INTR_SET_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&mut self) -> RESUME_INTR_SET_W<4> {
        RESUME_INTR_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_sie_set](index.html) module"]
pub struct INTR_SIE_SET_SPEC;
impl crate::RegisterSpec for INTR_SIE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_sie_set::R](R) reader structure"]
impl crate::Readable for INTR_SIE_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_sie_set::W](W) writer structure"]
impl crate::Writable for INTR_SIE_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SIE_SET to value 0"]
impl crate::Resettable for INTR_SIE_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
