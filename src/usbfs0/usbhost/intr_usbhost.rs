#[doc = "Reader of register INTR_USBHOST"]
pub type R = crate::R<u32, super::INTR_USBHOST>;
#[doc = "Writer for register INTR_USBHOST"]
pub type W = crate::W<u32, super::INTR_USBHOST>;
#[doc = "Register INTR_USBHOST `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_USBHOST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFIRQ`"]
pub type SOFIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIRQ`"]
pub struct SOFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQ_W<'a> {
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
#[doc = "Reader of field `DIRQ`"]
pub type DIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRQ`"]
pub struct DIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQ_W<'a> {
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
#[doc = "Reader of field `CNNIRQ`"]
pub type CNNIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNNIRQ`"]
pub struct CNNIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQ_W<'a> {
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
#[doc = "Reader of field `CMPIRQ`"]
pub type CMPIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPIRQ`"]
pub struct CMPIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQ_W<'a> {
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
#[doc = "Reader of field `URIRQ`"]
pub type URIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URIRQ`"]
pub struct URIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQ_W<'a> {
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
#[doc = "Reader of field `RWKIRQ`"]
pub type RWKIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWKIRQ`"]
pub struct RWKIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQ_W<'a> {
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
#[doc = "Reader of field `RSVD_6`"]
pub type RSVD_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSVD_6`"]
pub struct RSVD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_6_W<'a> {
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
#[doc = "Reader of field `TCAN`"]
pub type TCAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCAN`"]
pub struct TCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCAN_W<'a> {
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
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn sofirq(&self) -> SOFIRQ_R {
        SOFIRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn cnnirq(&self) -> CNNIRQ_R {
        CNNIRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn urirq(&self) -> URIRQ_R {
        URIRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rwkirq(&self) -> RWKIRQ_R {
        RWKIRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tcan(&self) -> TCAN_R {
        TCAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn sofirq(&mut self) -> SOFIRQ_W {
        SOFIRQ_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn dirq(&mut self) -> DIRQ_W {
        DIRQ_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn cnnirq(&mut self) -> CNNIRQ_W {
        CNNIRQ_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn cmpirq(&mut self) -> CMPIRQ_W {
        CMPIRQ_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn urirq(&mut self) -> URIRQ_W {
        URIRQ_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rwkirq(&mut self) -> RWKIRQ_W {
        RWKIRQ_W { w: self }
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W {
        RSVD_6_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tcan(&mut self) -> TCAN_W {
        TCAN_W { w: self }
    }
}
