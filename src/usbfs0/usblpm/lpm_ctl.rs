#[doc = "Reader of register LPM_CTL"]
pub type R = crate::R<u32, super::LPM_CTL>;
#[doc = "Writer for register LPM_CTL"]
pub type W = crate::W<u32, super::LPM_CTL>;
#[doc = "Register LPM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LPM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPM_EN`"]
pub type LPM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_EN`"]
pub struct LPM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_EN_W<'a> {
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
#[doc = "Reader of field `LPM_ACK_RESP`"]
pub type LPM_ACK_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_ACK_RESP`"]
pub struct LPM_ACK_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_ACK_RESP_W<'a> {
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
#[doc = "Reader of field `NYET_EN`"]
pub type NYET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYET_EN`"]
pub struct NYET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_EN_W<'a> {
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
#[doc = "Reader of field `SUB_RESP`"]
pub type SUB_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUB_RESP`"]
pub struct SUB_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB_RESP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&self) -> LPM_ACK_RESP_R {
        LPM_ACK_RESP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&self) -> NYET_EN_R {
        NYET_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&self) -> SUB_RESP_R {
        SUB_RESP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LPM_EN_W {
        LPM_EN_W { w: self }
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&mut self) -> LPM_ACK_RESP_W {
        LPM_ACK_RESP_W { w: self }
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&mut self) -> NYET_EN_W {
        NYET_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&mut self) -> SUB_RESP_W {
        SUB_RESP_W { w: self }
    }
}
