#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCBLL_CTRL` reader - N/A"]
pub type RCBLL_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `RCBLL_CTRL` writer - N/A"]
pub type RCBLL_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RCBLL_CPU_REQ` reader - N/A"]
pub type RCBLL_CPU_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RCBLL_CPU_REQ` writer - N/A"]
pub type RCBLL_CPU_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CPU_SINGLE_WRITE` reader - N/A"]
pub type CPU_SINGLE_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `CPU_SINGLE_WRITE` writer - N/A"]
pub type CPU_SINGLE_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CPU_SINGLE_READ` reader - N/A"]
pub type CPU_SINGLE_READ_R = crate::BitReader<bool>;
#[doc = "Field `CPU_SINGLE_READ` writer - N/A"]
pub type CPU_SINGLE_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ALLOW_CPU_ACCESS_TX_RX` reader - N/A"]
pub type ALLOW_CPU_ACCESS_TX_RX_R = crate::BitReader<bool>;
#[doc = "Field `ALLOW_CPU_ACCESS_TX_RX` writer - N/A"]
pub type ALLOW_CPU_ACCESS_TX_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE_RADIO_BOD` reader - N/A"]
pub type ENABLE_RADIO_BOD_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_RADIO_BOD` writer - N/A"]
pub type ENABLE_RADIO_BOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rcbll_ctrl(&self) -> RCBLL_CTRL_R {
        RCBLL_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&self) -> RCBLL_CPU_REQ_R {
        RCBLL_CPU_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&self) -> CPU_SINGLE_WRITE_R {
        CPU_SINGLE_WRITE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&self) -> CPU_SINGLE_READ_R {
        CPU_SINGLE_READ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&self) -> ALLOW_CPU_ACCESS_TX_RX_R {
        ALLOW_CPU_ACCESS_TX_RX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enable_radio_bod(&self) -> ENABLE_RADIO_BOD_R {
        ENABLE_RADIO_BOD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn rcbll_ctrl(&mut self) -> RCBLL_CTRL_W<0> {
        RCBLL_CTRL_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&mut self) -> RCBLL_CPU_REQ_W<1> {
        RCBLL_CPU_REQ_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&mut self) -> CPU_SINGLE_WRITE_W<2> {
        CPU_SINGLE_WRITE_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&mut self) -> CPU_SINGLE_READ_W<3> {
        CPU_SINGLE_READ_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&mut self) -> ALLOW_CPU_ACCESS_TX_RX_W<4> {
        ALLOW_CPU_ACCESS_TX_RX_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn enable_radio_bod(&mut self) -> ENABLE_RADIO_BOD_W<5> {
        ENABLE_RADIO_BOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCB LL control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
