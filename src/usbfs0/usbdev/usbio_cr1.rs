#[doc = "Reader of register USBIO_CR1"]
pub type R = crate::R<u32, super::USBIO_CR1>;
#[doc = "Writer for register USBIO_CR1"]
pub type W = crate::W<u32, super::USBIO_CR1>;
#[doc = "Register USBIO_CR1 `reset()`'s with value 0x20"]
impl crate::ResetValue for super::USBIO_CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `DMO`"]
pub type DMO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPO`"]
pub type DPO_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD_2`"]
pub type RSVD_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_2`"]
pub struct RSVD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_2_W<'a> {
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
#[doc = "Reader of field `IOMODE`"]
pub type IOMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOMODE`"]
pub struct IOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dmo(&self) -> DMO_R {
        DMO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dpo(&self) -> DPO_R {
        DPO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&self) -> IOMODE_R {
        IOMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> RSVD_2_W {
        RSVD_2_W { w: self }
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&mut self) -> IOMODE_W {
        IOMODE_W { w: self }
    }
}
