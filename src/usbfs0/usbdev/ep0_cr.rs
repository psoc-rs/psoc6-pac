#[doc = "Register `EP0_CR` reader"]
pub struct R(crate::R<EP0_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP0_CR` writer"]
pub struct W(crate::W<EP0_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0_CR_SPEC>;
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
impl From<crate::W<EP0_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Ignore all USB traffic to this endpoint"]
    DISABLE = 0,
    #[doc = "1: SETUP: Accept IN: NAK OUT: NAK"]
    NAK_INOUT = 1,
    #[doc = "2: SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    STATUS_OUT_ONLY = 2,
    #[doc = "3: SETUP: Accept IN: STALL OUT: STALL"]
    STALL_INOUT = 3,
    #[doc = "5: SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    ISO_OUT = 5,
    #[doc = "6: SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    STATUS_IN_ONLY = 6,
    #[doc = "7: SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    ISO_IN = 7,
    #[doc = "8: SETUP: Ignore IN: Ignore OUT: NAK"]
    NAK_OUT = 8,
    #[doc = "9: SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    ACK_OUT = 9,
    #[doc = "11: SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    ACK_OUT_STATUS_IN = 11,
    #[doc = "12: SETUP: Ignore IN: NAK OUT: Ignore"]
    NAK_IN = 12,
    #[doc = "13: SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    ACK_IN = 13,
    #[doc = "15: SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    ACK_IN_STATUS_OUT = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLE),
            1 => Some(MODE_A::NAK_INOUT),
            2 => Some(MODE_A::STATUS_OUT_ONLY),
            3 => Some(MODE_A::STALL_INOUT),
            5 => Some(MODE_A::ISO_OUT),
            6 => Some(MODE_A::STATUS_IN_ONLY),
            7 => Some(MODE_A::ISO_IN),
            8 => Some(MODE_A::NAK_OUT),
            9 => Some(MODE_A::ACK_OUT),
            11 => Some(MODE_A::ACK_OUT_STATUS_IN),
            12 => Some(MODE_A::NAK_IN),
            13 => Some(MODE_A::ACK_IN),
            15 => Some(MODE_A::ACK_IN_STATUS_OUT),
            _ => None,
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
#[doc = "Field `MODE` writer - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0_CR_SPEC, u8, MODE_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
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
#[doc = "Field `ACKED_TXN` reader - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type ACKED_TXN_R = crate::BitReader<ACKED_TXN_A>;
impl ACKED_TXN_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ACKED_TXN` writer - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type ACKED_TXN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CR_SPEC, ACKED_TXN_A, O>;
impl<'a, const O: u8> ACKED_TXN_W<'a, O> {
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
}
#[doc = "Field `OUT_RCVD` reader - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
pub type OUT_RCVD_R = crate::BitReader<bool>;
#[doc = "Field `OUT_RCVD` writer - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
pub type OUT_RCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CR_SPEC, bool, O>;
#[doc = "Field `IN_RCVD` reader - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
pub type IN_RCVD_R = crate::BitReader<bool>;
#[doc = "Field `IN_RCVD` writer - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
pub type IN_RCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CR_SPEC, bool, O>;
#[doc = "Field `SETUP_RCVD` reader - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
pub type SETUP_RCVD_R = crate::BitReader<bool>;
#[doc = "Field `SETUP_RCVD` writer - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
pub type SETUP_RCVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&self) -> ACKED_TXN_R {
        ACKED_TXN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn out_rcvd(&self) -> OUT_RCVD_R {
        OUT_RCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn in_rcvd(&self) -> IN_RCVD_R {
        IN_RCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn setup_rcvd(&self) -> SETUP_RCVD_R {
        SETUP_RCVD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&mut self) -> ACKED_TXN_W<4> {
        ACKED_TXN_W::new(self)
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn out_rcvd(&mut self) -> OUT_RCVD_W<5> {
        OUT_RCVD_W::new(self)
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn in_rcvd(&mut self) -> IN_RCVD_W<6> {
        IN_RCVD_W::new(self)
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn setup_rcvd(&mut self) -> SETUP_RCVD_W<7> {
        SETUP_RCVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint0 control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_cr](index.html) module"]
pub struct EP0_CR_SPEC;
impl crate::RegisterSpec for EP0_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep0_cr::R](R) reader structure"]
impl crate::Readable for EP0_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep0_cr::W](W) writer structure"]
impl crate::Writable for EP0_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP0_CR to value 0"]
impl crate::Resettable for EP0_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
