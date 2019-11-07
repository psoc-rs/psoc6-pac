#[doc = "Reader of register HOST_STATUS"]
pub type R = crate::R<u32, super::HOST_STATUS>;
#[doc = "Writer for register HOST_STATUS"]
pub type W = crate::W<u32, super::HOST_STATUS>;
#[doc = "Register HOST_STATUS `reset()`'s with value 0xc2"]
impl crate::ResetValue for super::HOST_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc2
    }
}
#[doc = "Reader of field `CSTAT`"]
pub type CSTAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TMODE`"]
pub type TMODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
#[doc = "Reader of field `SOFBUSY`"]
pub type SOFBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFBUSY`"]
pub struct SOFBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFBUSY_W<'a> {
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
#[doc = "Reader of field `URST`"]
pub type URST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URST`"]
pub struct URST_W<'a> {
    w: &'a mut W,
}
impl<'a> URST_W<'a> {
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
#[doc = "Reader of field `RSTBUSY`"]
pub type RSTBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKSEL_ST`"]
pub type CLKSEL_ST_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOST_ST`"]
pub type HOST_ST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - When this bit is '1', it means that the device is connected. When this bit is '0', it means that the device is disconnected. '0' : Device is disconnected. '1' : Device is connected. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (Host_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is '1', it means that the device is connected in the full-speed mode. When this bit is '0', it means that the device is connected in the low-speed mode. This bit is valid when the CSTAT bit of the Host Status Register (HOST_STATUS) is '1'. '0' : Low-speed. '1' : Full-speed. Notes: - This bit is initialized if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    pub fn sofbusy(&self) -> SOFBUSY_R {
        SOFBUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
    #[inline(always)]
    pub fn urst(&self) -> URST_R {
        URST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> RSVD_5_R {
        RSVD_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit shows that USB Host is being reset internally. If the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. If the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0', this bit is set to '0'. '0' : USB Host isn't being reset. '1' : USB Host is being reset. Notes: - If this bit is '1', the token must't be executed. - This bit isn't set to '0' or '1' immediately evne if the RST bit of Host Control 1 Register (HOST_CTL1) is set to '0' or '1'."]
    #[inline(always)]
    pub fn rstbusy(&self) -> RSTBUSY_R {
        RSTBUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit shows whether it is full-speed or not. If the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is set to '1', this bit is set to '1'. '0' : Low speed '1' : Full speed Note: - If this bit is different from the CLKSEL bit, The execution of the token and bus reset must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn clksel_st(&self) -> CLKSEL_ST_R {
        CLKSEL_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit shows whether it is USB Host mode. If the HOST bit of the Host Control Register (HOST_CTL0) is set to '1', this bit is set to '1'. '0' : USB Device '1' : USB Host Notes: - If this bit is different from the CLKSEL bit, The execution of the token must be waited until the match. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1)."]
    #[inline(always)]
    pub fn host_st(&self) -> HOST_ST_R {
        HOST_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - If this bit is set to '1', the USB Host is placed into the suspend state. If this bit is set to '0' while it is '1' or the USB bus is placed into the k-state mode, the suspend state is released, and the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. Set to '1' : Suspend. Set '0' while this bit is '1' : Resume. Others : Holds the status. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - If this bit is set to '1', this bit must not be set to '1' until the RWIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. - Do not set this bit to '1' while the USB is active (during USB bus resetting, data transfer, or SOF timer running). - If the value of this bit is changed, it is not immediately reflected on the state of the USB bus. To check whether or not the state is updated, read this bit."]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 3 - When a SOF token is sent using the Host Token Endpoint Register (HOST_TOKEN), this bit is set to '1', which means that the SOF timer is active. When this bit is '0', it means that the SOF timer is under suspension. To stop the active SOF timer, write '0' to this bit. However, if this bit is written with '1', its value is ignored. '0' : The SOF timer is stopped. '1' : The SOF timer is active. Notes: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit takes time to be initialized by the RST bit of the Host Control 1 Resgiter (HOST_CTL1). - The SOF timer does not stop immediately after this bit has been set to '0' to stop the SOF timer. To check whether or not the SOF timer is stopped, read this bit."]
    #[inline(always)]
    pub fn sofbusy(&mut self) -> SOFBUSY_W {
        SOFBUSY_W { w: self }
    }
    #[doc = "Bit 4 - When this bit is set to '1', the USB bus is reset. This bit continues set to '1' during USB bus resetting, and changes to '0' when USB bus resetting is ended. If this bit is set to '0', no processing is performed."]
    #[inline(always)]
    pub fn urst(&mut self) -> URST_W {
        URST_W { w: self }
    }
}
