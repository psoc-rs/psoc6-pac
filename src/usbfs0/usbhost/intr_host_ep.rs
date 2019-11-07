#[doc = "Reader of register INTR_HOST_EP"]
pub type R = crate::R<u32, super::INTR_HOST_EP>;
#[doc = "Writer for register INTR_HOST_EP"]
pub type W = crate::W<u32, super::INTR_HOST_EP>;
#[doc = "Register INTR_HOST_EP `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_HOST_EP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1DRQ`"]
pub type EP1DRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1DRQ`"]
pub struct EP1DRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1DRQ_W<'a> {
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
#[doc = "Reader of field `EP1SPK`"]
pub type EP1SPK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1SPK`"]
pub struct EP1SPK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1SPK_W<'a> {
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
#[doc = "Reader of field `EP2DRQ`"]
pub type EP2DRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2DRQ`"]
pub struct EP2DRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2DRQ_W<'a> {
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
#[doc = "Reader of field `EP2SPK`"]
pub type EP2SPK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2SPK`"]
pub struct EP2SPK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2SPK_W<'a> {
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
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep1drq(&self) -> EP1DRQ_R {
        EP1DRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep1spk(&self) -> EP1SPK_R {
        EP1SPK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep2drq(&self) -> EP2DRQ_R {
        EP2DRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep2spk(&self) -> EP2SPK_R {
        EP2SPK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit indicates that the EP1 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep1drq(&mut self) -> EP1DRQ_W {
        EP1DRQ_W { w: self }
    }
    #[doc = "Bit 3 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 1 Control Register (HOST_EP1_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep1spk(&mut self) -> EP1SPK_W {
        EP1SPK_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates that the EP2 packet transfer has normally ended, and processing of the data is required. The DRQ bit is an interrupt cause, and writing '0' is ignored. Clear the DRQ bit by writing '1'. '0' : Clears the interrupt cause '1' : Packet transfer normally ended Note : - If automatic buffer transfer mode (DMAE = '1') is not used, '1' must be written to the DRQ bit after data has been written or read to/from the send/receive buffer. Switch the access buffers once the DRQ bit is cleared. That DRQ = '0' may not be read after the DRQ bit is cleared. If the transfer direction is set to OUT, and the DRQ bit is cleared without writing buffer data while the DRQ bit is '1', it implies that 0-byte data is set. If DIR of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is set to '1' at initial settings, the DRQ bit of corresponding Endpoint is set at the same time. Also while the DRQ bit is not set, '1' must not be written."]
    #[inline(always)]
    pub fn ep2drq(&mut self) -> EP2DRQ_W {
        EP2DRQ_W { w: self }
    }
    #[doc = "Bit 5 - This bit indicates that the data size transferred from the host does not satisfy the maximum packet size (including 0-byte) set by PKS in the Host Endpoint 2 Control Register (HOST_EP2_CTL) when the data has been received successfully. This bit is an interrupt cause, and writing '0' is ignored. Clear it by writing '1'. '0' : Received data size satisfies the maximum packet size '1' : Received data size does not satisfy the maximum packet size Note : - The SPK bit is not set during data transfer in the OUT direction."]
    #[inline(always)]
    pub fn ep2spk(&mut self) -> EP2SPK_W {
        EP2SPK_W { w: self }
    }
}
