#[doc = "Reader of register EP0_CR"]
pub type R = crate::R<u32, super::EP0_CR>;
#[doc = "Writer for register EP0_CR"]
pub type W = crate::W<u32, super::EP0_CR>;
#[doc = "Register EP0_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::EP0_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "1: SETUP: Accept\nIN: NAK\nOUT:  NAK"]
    NAK_INOUT = 1,
    #[doc = "2: SETUP: Accept\nIN: STALL\nOUT:  ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 2,
    #[doc = "3: SETUP: Accept\nIN: STALL\nOUT:  STALL"]
    STALL_INOUT = 3,
    #[doc = "5: SETUP: Ignore\nIN: Ignore\nOUT:  Accept Isochronous OUT token"]
    ISO_OUT = 5,
    #[doc = "6: SETUP: Accept\nIN: Respond with 0B data\nOUT:  Stall"]
    STATUS_IN_ONLY = 6,
    #[doc = "7: SETUP: Ignore\nIN: Accept Isochronous IN token\nOUT:  Ignore"]
    ISO_IN = 7,
    #[doc = "8: SETUP: Ignore\nIN: Ignore\nOUT:  NAK"]
    NAK_OUT = 8,
    #[doc = "9: SETUP: Ignore\nIN: Ignore\nOUT:  Accept data and ACK if STALL=0, STALL otherwise.\nChange to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 9,
    #[doc = "11: SETUP: Accept\nIN: Respond with 0B data\nOUT:  Accept data"]
    ACK_OUT_STATUS_IN = 11,
    #[doc = "12: SETUP: Ignore\nIN: NAK\nOUT:  Ignore"]
    NAK_IN = 12,
    #[doc = "13: SETUP: Ignore\nIN: Respond to IN with data if STALL=0, STALL otherwise\nOUT:  Ignore"]
    ACK_IN = 13,
    #[doc = "15: SETUP: Accept\nIN: Respond to IN with data\nOUT:  ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::DISABLE),
            1 => Val(MODE_A::NAK_INOUT),
            2 => Val(MODE_A::STATUS_OUT_ONLY),
            3 => Val(MODE_A::STALL_INOUT),
            5 => Val(MODE_A::ISO_OUT),
            6 => Val(MODE_A::STATUS_IN_ONLY),
            7 => Val(MODE_A::ISO_IN),
            8 => Val(MODE_A::NAK_OUT),
            9 => Val(MODE_A::ACK_OUT),
            11 => Val(MODE_A::ACK_OUT_STATUS_IN),
            12 => Val(MODE_A::NAK_IN),
            13 => Val(MODE_A::ACK_IN),
            15 => Val(MODE_A::ACK_IN_STATUS_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `NAK_INOUT`"]
    #[inline(always)]
    pub fn is_nak_inout(&self) -> bool {
        *self == MODE_A::NAK_INOUT
    }
    #[doc = "Checks if the value of the field is `STATUS_OUT_ONLY`"]
    #[inline(always)]
    pub fn is_status_out_only(&self) -> bool {
        *self == MODE_A::STATUS_OUT_ONLY
    }
    #[doc = "Checks if the value of the field is `STALL_INOUT`"]
    #[inline(always)]
    pub fn is_stall_inout(&self) -> bool {
        *self == MODE_A::STALL_INOUT
    }
    #[doc = "Checks if the value of the field is `ISO_OUT`"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == MODE_A::ISO_OUT
    }
    #[doc = "Checks if the value of the field is `STATUS_IN_ONLY`"]
    #[inline(always)]
    pub fn is_status_in_only(&self) -> bool {
        *self == MODE_A::STATUS_IN_ONLY
    }
    #[doc = "Checks if the value of the field is `ISO_IN`"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == MODE_A::ISO_IN
    }
    #[doc = "Checks if the value of the field is `NAK_OUT`"]
    #[inline(always)]
    pub fn is_nak_out(&self) -> bool {
        *self == MODE_A::NAK_OUT
    }
    #[doc = "Checks if the value of the field is `ACK_OUT`"]
    #[inline(always)]
    pub fn is_ack_out(&self) -> bool {
        *self == MODE_A::ACK_OUT
    }
    #[doc = "Checks if the value of the field is `ACK_OUT_STATUS_IN`"]
    #[inline(always)]
    pub fn is_ack_out_status_in(&self) -> bool {
        *self == MODE_A::ACK_OUT_STATUS_IN
    }
    #[doc = "Checks if the value of the field is `NAK_IN`"]
    #[inline(always)]
    pub fn is_nak_in(&self) -> bool {
        *self == MODE_A::NAK_IN
    }
    #[doc = "Checks if the value of the field is `ACK_IN`"]
    #[inline(always)]
    pub fn is_ack_in(&self) -> bool {
        *self == MODE_A::ACK_IN
    }
    #[doc = "Checks if the value of the field is `ACK_IN_STATUS_OUT`"]
    #[inline(always)]
    pub fn is_ack_in_status_out(&self) -> bool {
        *self == MODE_A::ACK_IN_STATUS_OUT
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ignore all USB traffic to this endpoint"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MODE_A::DISABLE)
    }
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    #[inline(always)]
    pub fn nak_inout(self) -> &'a mut W {
        self.variant(MODE_A::NAK_INOUT)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn status_out_only(self) -> &'a mut W {
        self.variant(MODE_A::STATUS_OUT_ONLY)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    #[inline(always)]
    pub fn stall_inout(self) -> &'a mut W {
        self.variant(MODE_A::STALL_INOUT)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut W {
        self.variant(MODE_A::ISO_OUT)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    #[inline(always)]
    pub fn status_in_only(self) -> &'a mut W {
        self.variant(MODE_A::STATUS_IN_ONLY)
    }
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut W {
        self.variant(MODE_A::ISO_IN)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    #[inline(always)]
    pub fn nak_out(self) -> &'a mut W {
        self.variant(MODE_A::NAK_OUT)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    #[inline(always)]
    pub fn ack_out(self) -> &'a mut W {
        self.variant(MODE_A::ACK_OUT)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    #[inline(always)]
    pub fn ack_out_status_in(self) -> &'a mut W {
        self.variant(MODE_A::ACK_OUT_STATUS_IN)
    }
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    #[inline(always)]
    pub fn nak_in(self) -> &'a mut W {
        self.variant(MODE_A::NAK_IN)
    }
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    #[inline(always)]
    pub fn ack_in(self) -> &'a mut W {
        self.variant(MODE_A::ACK_IN)
    }
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn ack_in_status_out(self) -> &'a mut W {
        self.variant(MODE_A::ACK_IN_STATUS_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKED_TXN_A {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    ACKED_NO = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    ACKED_YES = 1,
}
impl From<ACKED_TXN_A> for bool {
    #[inline(always)]
    fn from(variant: ACKED_TXN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACKED_TXN`"]
pub type ACKED_TXN_R = crate::R<bool, ACKED_TXN_A>;
impl ACKED_TXN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKED_TXN_A {
        match self.bits {
            false => ACKED_TXN_A::ACKED_NO,
            true => ACKED_TXN_A::ACKED_YES,
        }
    }
    #[doc = "Checks if the value of the field is `ACKED_NO`"]
    #[inline(always)]
    pub fn is_acked_no(&self) -> bool {
        *self == ACKED_TXN_A::ACKED_NO
    }
    #[doc = "Checks if the value of the field is `ACKED_YES`"]
    #[inline(always)]
    pub fn is_acked_yes(&self) -> bool {
        *self == ACKED_TXN_A::ACKED_YES
    }
}
#[doc = "Write proxy for field `ACKED_TXN`"]
pub struct ACKED_TXN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKED_TXN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKED_TXN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn acked_no(self) -> &'a mut W {
        self.variant(ACKED_TXN_A::ACKED_NO)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn acked_yes(self) -> &'a mut W {
        self.variant(ACKED_TXN_A::ACKED_YES)
    }
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
#[doc = "Reader of field `OUT_RCVD`"]
pub type OUT_RCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_RCVD`"]
pub struct OUT_RCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RCVD_W<'a> {
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
#[doc = "Reader of field `IN_RCVD`"]
pub type IN_RCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_RCVD`"]
pub struct IN_RCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RCVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SETUP_RCVD`"]
pub type SETUP_RCVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP_RCVD`"]
pub struct SETUP_RCVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_RCVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&self) -> ACKED_TXN_R {
        ACKED_TXN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn out_rcvd(&self) -> OUT_RCVD_R {
        OUT_RCVD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn in_rcvd(&self) -> IN_RCVD_R {
        IN_RCVD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn setup_rcvd(&self) -> SETUP_RCVD_R {
        SETUP_RCVD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&mut self) -> ACKED_TXN_W {
        ACKED_TXN_W { w: self }
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn out_rcvd(&mut self) -> OUT_RCVD_W {
        OUT_RCVD_W { w: self }
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn in_rcvd(&mut self) -> IN_RCVD_W {
        IN_RCVD_W { w: self }
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn setup_rcvd(&mut self) -> SETUP_RCVD_W {
        SETUP_RCVD_W { w: self }
    }
}
