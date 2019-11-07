#[doc = "Reader of register INTR_HOST_EP_SET"]
pub type R = crate::R<u32, super::INTR_HOST_EP_SET>;
#[doc = "Writer for register INTR_HOST_EP_SET"]
pub type W = crate::W<u32, super::INTR_HOST_EP_SET>;
#[doc = "Register INTR_HOST_EP_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_HOST_EP_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1DRQS`"]
pub type EP1DRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1DRQS`"]
pub struct EP1DRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1DRQS_W<'a> {
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
#[doc = "Reader of field `EP1SPKS`"]
pub type EP1SPKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1SPKS`"]
pub struct EP1SPKS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1SPKS_W<'a> {
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
#[doc = "Reader of field `EP2DRQS`"]
pub type EP2DRQS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2DRQS`"]
pub struct EP2DRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2DRQS_W<'a> {
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
#[doc = "Reader of field `EP2SPKS`"]
pub type EP2SPKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2SPKS`"]
pub struct EP2SPKS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2SPKS_W<'a> {
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
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&self) -> EP1DRQS_R {
        EP1DRQS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&self) -> EP1SPKS_R {
        EP1SPKS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&self) -> EP2DRQS_R {
        EP2DRQS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&self) -> EP2SPKS_R {
        EP2SPKS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&mut self) -> EP1DRQS_W {
        EP1DRQS_W { w: self }
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&mut self) -> EP1SPKS_W {
        EP1SPKS_W { w: self }
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&mut self) -> EP2DRQS_W {
        EP2DRQS_W { w: self }
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&mut self) -> EP2SPKS_W {
        EP2SPKS_W { w: self }
    }
}
