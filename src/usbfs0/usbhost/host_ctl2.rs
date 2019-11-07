#[doc = "Reader of register HOST_CTL2"]
pub type R = crate::R<u32, super::HOST_CTL2>;
#[doc = "Writer for register HOST_CTL2"]
pub type W = crate::W<u32, super::HOST_CTL2>;
#[doc = "Register HOST_CTL2 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::HOST_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RETRY`"]
pub type RETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRY`"]
pub struct RETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CANCEL`"]
pub type CANCEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CANCEL`"]
pub struct CANCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CANCEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SOFSTEP`"]
pub type SOFSTEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFSTEP`"]
pub struct SOFSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFSTEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ALIVE`"]
pub type ALIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALIVE`"]
pub struct ALIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIVE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RSVD_4`"]
pub type RSVD_4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_4`"]
pub struct RSVD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSVD_5`"]
pub type RSVD_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_5`"]
pub struct RSVD_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TTEST`"]
pub type TTEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TTEST`"]
pub struct TTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TTEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn cancel(&self) -> CANCEL_R {
        CANCEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn sofstep(&self) -> SOFSTEP_R {
        SOFSTEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn alive(&self) -> ALIVE_R {
        ALIVE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> RSVD_4_R {
        RSVD_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> RSVD_5_R {
        RSVD_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Timer Test. Set this bits to '00'."]
    #[inline(always)]
    pub fn ttest(&self) -> TTEST_R {
        TTEST_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed during the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn retry(&mut self) -> RETRY_W {
        RETRY_W { w: self }
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn cancel(&mut self) -> CANCEL_W {
        CANCEL_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn sofstep(&mut self) -> SOFSTEP_W {
        SOFSTEP_W { w: self }
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is effective when the CLKSEL bit of the Host Conrtol 1 Register (HOST_CTL1) is '0'. If the CLKSEL bit is '1', SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn alive(&mut self) -> ALIVE_W {
        ALIVE_W { w: self }
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> RSVD_4_W {
        RSVD_4_W { w: self }
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&mut self) -> RSVD_5_W {
        RSVD_5_W { w: self }
    }
    #[doc = "Bits 6:7 - Timer Test. Set this bits to '00'."]
    #[inline(always)]
    pub fn ttest(&mut self) -> TTEST_W {
        TTEST_W { w: self }
    }
}
