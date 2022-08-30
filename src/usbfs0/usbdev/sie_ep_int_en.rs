#[doc = "Register `SIE_EP_INT_EN` reader"]
pub struct R(crate::R<SIE_EP_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIE_EP_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIE_EP_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIE_EP_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIE_EP_INT_EN` writer"]
pub struct W(crate::W<SIE_EP_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIE_EP_INT_EN_SPEC>;
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
impl From<crate::W<SIE_EP_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIE_EP_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_INTR_EN` reader - Enables interrupt for EP1"]
pub type EP1_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP1_INTR_EN` writer - Enables interrupt for EP1"]
pub type EP1_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP2_INTR_EN` reader - Enables interrupt for EP2"]
pub type EP2_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP2_INTR_EN` writer - Enables interrupt for EP2"]
pub type EP2_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP3_INTR_EN` reader - Enables interrupt for EP3"]
pub type EP3_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP3_INTR_EN` writer - Enables interrupt for EP3"]
pub type EP3_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP4_INTR_EN` reader - Enables interrupt for EP4"]
pub type EP4_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP4_INTR_EN` writer - Enables interrupt for EP4"]
pub type EP4_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP5_INTR_EN` reader - Enables interrupt for EP5"]
pub type EP5_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP5_INTR_EN` writer - Enables interrupt for EP5"]
pub type EP5_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP6_INTR_EN` reader - Enables interrupt for EP6"]
pub type EP6_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP6_INTR_EN` writer - Enables interrupt for EP6"]
pub type EP6_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP7_INTR_EN` reader - Enables interrupt for EP7"]
pub type EP7_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP7_INTR_EN` writer - Enables interrupt for EP7"]
pub type EP7_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
#[doc = "Field `EP8_INTR_EN` reader - Enables interrupt for EP8"]
pub type EP8_INTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `EP8_INTR_EN` writer - Enables interrupt for EP8"]
pub type EP8_INTR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIE_EP_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&self) -> EP1_INTR_EN_R {
        EP1_INTR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&self) -> EP2_INTR_EN_R {
        EP2_INTR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&self) -> EP3_INTR_EN_R {
        EP3_INTR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&self) -> EP4_INTR_EN_R {
        EP4_INTR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&self) -> EP5_INTR_EN_R {
        EP5_INTR_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&self) -> EP6_INTR_EN_R {
        EP6_INTR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&self) -> EP7_INTR_EN_R {
        EP7_INTR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&self) -> EP8_INTR_EN_R {
        EP8_INTR_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&mut self) -> EP1_INTR_EN_W<0> {
        EP1_INTR_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&mut self) -> EP2_INTR_EN_W<1> {
        EP2_INTR_EN_W::new(self)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&mut self) -> EP3_INTR_EN_W<2> {
        EP3_INTR_EN_W::new(self)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&mut self) -> EP4_INTR_EN_W<3> {
        EP4_INTR_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&mut self) -> EP5_INTR_EN_W<4> {
        EP5_INTR_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&mut self) -> EP6_INTR_EN_W<5> {
        EP6_INTR_EN_W::new(self)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&mut self) -> EP7_INTR_EN_W<6> {
        EP7_INTR_EN_W::new(self)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&mut self) -> EP8_INTR_EN_W<7> {
        EP8_INTR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SIE Data Endpoints Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sie_ep_int_en](index.html) module"]
pub struct SIE_EP_INT_EN_SPEC;
impl crate::RegisterSpec for SIE_EP_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sie_ep_int_en::R](R) reader structure"]
impl crate::Readable for SIE_EP_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sie_ep_int_en::W](W) writer structure"]
impl crate::Writable for SIE_EP_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIE_EP_INT_EN to value 0"]
impl crate::Resettable for SIE_EP_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
