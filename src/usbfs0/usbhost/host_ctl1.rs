#[doc = "Reader of register HOST_CTL1"]
pub type R = crate::R<u32, super::HOST_CTL1>;
#[doc = "Writer for register HOST_CTL1"]
pub type W = crate::W<u32, super::HOST_CTL1>;
#[doc = "Register HOST_CTL1 `reset()`'s with value 0x83"]
impl crate::ResetValue for super::HOST_CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x83
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
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
#[doc = "Reader of field `USTP`"]
pub type USTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USTP`"]
pub struct USTP_W<'a> {
    w: &'a mut W,
}
impl<'a> USTP_W<'a> {
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
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&self) -> USTP_R {
        USTP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&mut self) -> USTP_W {
        USTP_W { w: self }
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
}
