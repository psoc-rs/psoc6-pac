#[doc = "Register `TR_CTL` reader"]
pub struct R(crate::R<TR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL` writer"]
pub struct W(crate::W<TR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL_SPEC>;
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
impl From<crate::W<TR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_REQ_EN` reader - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
pub type RX_REQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_REQ_EN` writer - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
pub type RX_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&self) -> RX_REQ_EN_R {
        RX_REQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&mut self) -> RX_REQ_EN_W<16> {
        RX_REQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](index.html) module"]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl::R](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTL to value 0"]
impl crate::Resettable for TR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
