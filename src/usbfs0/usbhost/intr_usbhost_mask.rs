#[doc = "Register `INTR_USBHOST_MASK` reader"]
pub struct R(crate::R<INTR_USBHOST_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_USBHOST_MASK` writer"]
pub struct W(crate::W<INTR_USBHOST_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_USBHOST_MASK_SPEC>;
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
impl From<crate::W<INTR_USBHOST_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_USBHOST_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIRQM` reader - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SOFIRQM_R = crate::BitReader<bool>;
#[doc = "Field `SOFIRQM` writer - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
pub type SOFIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `DIRQM` reader - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DIRQM_R = crate::BitReader<bool>;
#[doc = "Field `DIRQM` writer - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
pub type DIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `CNNIRQM` reader - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CNNIRQM_R = crate::BitReader<bool>;
#[doc = "Field `CNNIRQM` writer - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
pub type CNNIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `CMPIRQM` reader - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CMPIRQM_R = crate::BitReader<bool>;
#[doc = "Field `CMPIRQM` writer - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
pub type CMPIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `URIRQM` reader - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type URIRQM_R = crate::BitReader<bool>;
#[doc = "Field `URIRQM` writer - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
pub type URIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `RWKIRQM` reader - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RWKIRQM_R = crate::BitReader<bool>;
#[doc = "Field `RWKIRQM` writer - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
pub type RWKIRQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type RSVD_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
#[doc = "Field `TCANM` reader - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TCANM_R = crate::BitReader<bool>;
#[doc = "Field `TCANM` writer - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
pub type TCANM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&self) -> SOFIRQM_R {
        SOFIRQM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&self) -> DIRQM_R {
        DIRQM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&self) -> CNNIRQM_R {
        CNNIRQM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&self) -> CMPIRQM_R {
        CMPIRQM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&self) -> URIRQM_R {
        URIRQM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&self) -> RWKIRQM_R {
        RWKIRQM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&self) -> TCANM_R {
        TCANM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit masks the interrupt by SOF flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn sofirqm(&mut self) -> SOFIRQM_W<0> {
        SOFIRQM_W::new(self)
    }
    #[doc = "Bit 1 - This bit masks the interrupt by DIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn dirqm(&mut self) -> DIRQM_W<1> {
        DIRQM_W::new(self)
    }
    #[doc = "Bit 2 - This bit masks the interrupt by CNNIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cnnirqm(&mut self) -> CNNIRQM_W<2> {
        CNNIRQM_W::new(self)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by CMPIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn cmpirqm(&mut self) -> CMPIRQM_W<3> {
        CMPIRQM_W::new(self)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by URIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn urirqm(&mut self) -> URIRQM_W<4> {
        URIRQM_W::new(self)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by RWKIRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn rwkirqm(&mut self) -> RWKIRQM_W<5> {
        RWKIRQM_W::new(self)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<6> {
        RSVD_6_W::new(self)
    }
    #[doc = "Bit 7 - This bit masks the interrupt by TCAN flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn tcanm(&mut self) -> TCANM_W<7> {
        TCANM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost_mask](index.html) module"]
pub struct INTR_USBHOST_MASK_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost_mask::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_usbhost_mask::W](W) writer structure"]
impl crate::Writable for INTR_USBHOST_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_USBHOST_MASK to value 0"]
impl crate::Resettable for INTR_USBHOST_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
