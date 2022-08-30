#[doc = "Register `INTR_HOST_EP_SET` reader"]
pub struct R(crate::R<INTR_HOST_EP_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_HOST_EP_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_HOST_EP_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_HOST_EP_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_HOST_EP_SET` writer"]
pub struct W(crate::W<INTR_HOST_EP_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_HOST_EP_SET_SPEC>;
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
impl From<crate::W<INTR_HOST_EP_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_HOST_EP_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1DRQS` reader - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type EP1DRQS_R = crate::BitReader<bool>;
#[doc = "Field `EP1DRQS` writer - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type EP1DRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_SET_SPEC, bool, O>;
#[doc = "Field `EP1SPKS` reader - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type EP1SPKS_R = crate::BitReader<bool>;
#[doc = "Field `EP1SPKS` writer - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type EP1SPKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_SET_SPEC, bool, O>;
#[doc = "Field `EP2DRQS` reader - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type EP2DRQS_R = crate::BitReader<bool>;
#[doc = "Field `EP2DRQS` writer - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type EP2DRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_SET_SPEC, bool, O>;
#[doc = "Field `EP2SPKS` reader - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type EP2SPKS_R = crate::BitReader<bool>;
#[doc = "Field `EP2SPKS` writer - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type EP2SPKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_HOST_EP_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&self) -> EP1DRQS_R {
        EP1DRQS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&self) -> EP1SPKS_R {
        EP1SPKS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&self) -> EP2DRQS_R {
        EP2DRQS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&self) -> EP2SPKS_R {
        EP2SPKS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&mut self) -> EP1DRQS_W<2> {
        EP1DRQS_W::new(self)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&mut self) -> EP1SPKS_W<3> {
        EP1SPKS_W::new(self)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&mut self) -> EP2DRQS_W<4> {
        EP2DRQS_W::new(self)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&mut self) -> EP2SPKS_W<5> {
        EP2SPKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Endpoint Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_host_ep_set](index.html) module"]
pub struct INTR_HOST_EP_SET_SPEC;
impl crate::RegisterSpec for INTR_HOST_EP_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_host_ep_set::R](R) reader structure"]
impl crate::Readable for INTR_HOST_EP_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_host_ep_set::W](W) writer structure"]
impl crate::Writable for INTR_HOST_EP_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_HOST_EP_SET to value 0"]
impl crate::Resettable for INTR_HOST_EP_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
