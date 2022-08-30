#[doc = "Register `SIE_EP_INT_SR` reader"]
pub struct R(crate::R<SIE_EP_INT_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_EP_INT_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_EP_INT_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_EP_INT_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_EP_INT_SR` writer"]
pub struct W(crate::W<SIE_EP_INT_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_EP_INT_SR_SPEC>;
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
impl From<crate::W<SIE_EP_INT_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_EP_INT_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_INTR` reader - Interrupt status for EP1"]
pub type EP1_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP1_INTR` writer - Interrupt status for EP1"]
pub type EP1_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP2_INTR` reader - Interrupt status for EP2"]
pub type EP2_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP2_INTR` writer - Interrupt status for EP2"]
pub type EP2_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP3_INTR` reader - Interrupt status for EP3"]
pub type EP3_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP3_INTR` writer - Interrupt status for EP3"]
pub type EP3_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP4_INTR` reader - Interrupt status for EP4"]
pub type EP4_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP4_INTR` writer - Interrupt status for EP4"]
pub type EP4_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP5_INTR` reader - Interrupt status for EP5"]
pub type EP5_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP5_INTR` writer - Interrupt status for EP5"]
pub type EP5_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP6_INTR` reader - Interrupt status for EP6"]
pub type EP6_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP6_INTR` writer - Interrupt status for EP6"]
pub type EP6_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP7_INTR` reader - Interrupt status for EP7"]
pub type EP7_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP7_INTR` writer - Interrupt status for EP7"]
pub type EP7_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
#[doc = "Field `EP8_INTR` reader - Interrupt status for EP8"]
pub type EP8_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EP8_INTR` writer - Interrupt status for EP8"]
pub type EP8_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> EP1_INTR_R {
        EP1_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> EP2_INTR_R {
        EP2_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> EP3_INTR_R {
        EP3_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> EP4_INTR_R {
        EP4_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> EP5_INTR_R {
        EP5_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> EP6_INTR_R {
        EP6_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> EP7_INTR_R {
        EP7_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> EP8_INTR_R {
        EP8_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&mut self) -> EP1_INTR_W<0> {
        EP1_INTR_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&mut self) -> EP2_INTR_W<1> {
        EP2_INTR_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&mut self) -> EP3_INTR_W<2> {
        EP3_INTR_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&mut self) -> EP4_INTR_W<3> {
        EP4_INTR_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&mut self) -> EP5_INTR_W<4> {
        EP5_INTR_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&mut self) -> EP6_INTR_W<5> {
        EP6_INTR_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&mut self) -> EP7_INTR_W<6> {
        EP7_INTR_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&mut self) -> EP8_INTR_W<7> {
        EP8_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SIE Data Endpoint Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep_int_sr](index.html) module"]
pub struct SIE_EP_INT_SR_SPEC;
impl crate::RegisterSpec for SIE_EP_INT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ep_int_sr::R](R) reader structure"]
impl crate::Readable for SIE_EP_INT_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ep_int_sr::W](W) writer structure"]
impl crate::Writable for SIE_EP_INT_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_EP_INT_SR to value 0"]
impl crate::Resettable for SIE_EP_INT_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
