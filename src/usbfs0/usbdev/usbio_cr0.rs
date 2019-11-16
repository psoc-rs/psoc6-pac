#[doc = "Reader of register USBIO_CR0"]
pub type R = crate::R<u32, super::USBIO_CR0>;
#[doc = "Writer for register USBIO_CR0"]
pub type W = crate::W<u32, super::USBIO_CR0>;
#[doc = "Register USBIO_CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBIO_CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_A {
    #[doc = "0: D+ < D- (K state)"]
    DIFF_LOW = 0,
    #[doc = "1: D+ > D- (J state)"]
    DIFF_HIGH = 1,
}
impl From<RD_A> for bool {
    #[inline(always)]
    fn from(variant: RD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<bool, RD_A>;
impl RD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_A {
        match self.bits {
            false => RD_A::DIFF_LOW,
            true => RD_A::DIFF_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `DIFF_LOW`"]
    #[inline(always)]
    pub fn is_diff_low(&self) -> bool {
        *self == RD_A::DIFF_LOW
    }
    #[doc = "Checks if the value of the field is `DIFF_HIGH`"]
    #[inline(always)]
    pub fn is_diff_high(&self) -> bool {
        *self == RD_A::DIFF_HIGH
    }
}
#[doc = "Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TD_A {
    #[doc = "0: Force USB K state (D+ is low D- is high)."]
    DIFF_K = 0,
    #[doc = "1: Force USB J state (D+ is high D- is low)."]
    DIFF_J = 1,
}
impl From<TD_A> for bool {
    #[inline(always)]
    fn from(variant: TD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TD`"]
pub type TD_R = crate::R<bool, TD_A>;
impl TD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TD_A {
        match self.bits {
            false => TD_A::DIFF_K,
            true => TD_A::DIFF_J,
        }
    }
    #[doc = "Checks if the value of the field is `DIFF_K`"]
    #[inline(always)]
    pub fn is_diff_k(&self) -> bool {
        *self == TD_A::DIFF_K
    }
    #[doc = "Checks if the value of the field is `DIFF_J`"]
    #[inline(always)]
    pub fn is_diff_j(&self) -> bool {
        *self == TD_A::DIFF_J
    }
}
#[doc = "Write proxy for field `TD`"]
pub struct TD_W<'a> {
    w: &'a mut W,
}
impl<'a> TD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Force USB K state (D+ is low D- is high)."]
    #[inline(always)]
    pub fn diff_k(self) -> &'a mut W {
        self.variant(TD_A::DIFF_K)
    }
    #[doc = "Force USB J state (D+ is high D- is low)."]
    #[inline(always)]
    pub fn diff_j(self) -> &'a mut W {
        self.variant(TD_A::DIFF_J)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TSE0`"]
pub type TSE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSE0`"]
pub struct TSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE0_W<'a> {
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
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
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
    #[doc = "Bit 0 - Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub fn td(&self) -> TD_R {
        TD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub fn tse0(&self) -> TSE0_R {
        TSE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub fn td(&mut self) -> TD_W {
        TD_W { w: self }
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub fn tse0(&mut self) -> TSE0_W {
        TSE0_W { w: self }
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
}
