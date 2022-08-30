#[doc = "Register `HOST_CTL2` reader"]
pub struct R(crate::R<HOST_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CTL2` writer"]
pub struct W(crate::W<HOST_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTL2_SPEC>;
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
impl From<crate::W<HOST_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETRY` reader - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RETRY_R = crate::BitReader<bool>;
#[doc = "Field `RETRY` writer - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RETRY_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `CANCEL` reader - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
pub type CANCEL_R = crate::BitReader<bool>;
#[doc = "Field `CANCEL` writer - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
pub type CANCEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `SOFSTEP` reader - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
pub type SOFSTEP_R = crate::BitReader<bool>;
#[doc = "Field `SOFSTEP` writer - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
pub type SOFSTEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `ALIVE` reader - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
pub type ALIVE_R = crate::BitReader<bool>;
#[doc = "Field `ALIVE` writer - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
pub type ALIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `RSVD_4` reader - N/A"]
pub type RSVD_4_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_4` writer - N/A"]
pub type RSVD_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `RSVD_5` reader - N/A"]
pub type RSVD_5_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_5` writer - N/A"]
pub type RSVD_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL2_SPEC, bool, O>;
#[doc = "Field `TTEST` reader - Timer Test. Set this bits to '00'."]
pub type TTEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTEST` writer - Timer Test. Set this bits to '00'."]
pub type TTEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_CTL2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn cancel(&self) -> CANCEL_R {
        CANCEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn sofstep(&self) -> SOFSTEP_R {
        SOFSTEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn alive(&self) -> ALIVE_R {
        ALIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> RSVD_4_R {
        RSVD_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> RSVD_5_R {
        RSVD_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Timer Test. Set this bits to '00'."]
    #[inline(always)]
    pub fn ttest(&self) -> TTEST_R {
        TTEST_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn retry(&mut self) -> RETRY_W<0> {
        RETRY_W::new(self)
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn cancel(&mut self) -> CANCEL_W<1> {
        CANCEL_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn sofstep(&mut self) -> SOFSTEP_W<2> {
        SOFSTEP_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn alive(&mut self) -> ALIVE_W<3> {
        ALIVE_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> RSVD_4_W<4> {
        RSVD_4_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> RSVD_5_W<5> {
        RSVD_5_W::new(self)
    }
    #[doc = "Bits 6:7 - Timer Test. Set this bits to '00'."]
    #[inline(always)]
    pub fn ttest(&mut self) -> TTEST_W<6> {
        TTEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 2 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl2](index.html) module"]
pub struct HOST_CTL2_SPEC;
impl crate::RegisterSpec for HOST_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ctl2::R](R) reader structure"]
impl crate::Readable for HOST_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctl2::W](W) writer structure"]
impl crate::Writable for HOST_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_CTL2 to value 0x01"]
impl crate::Resettable for HOST_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
