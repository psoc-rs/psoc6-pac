#[doc = "Register `INTR_USBHOST` reader"]
pub struct R(crate::R<INTR_USBHOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_USBHOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_USBHOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_USBHOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_USBHOST` writer"]
pub struct W(crate::W<INTR_USBHOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_USBHOST_SPEC>;
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
impl From<crate::W<INTR_USBHOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_USBHOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIRQ` reader - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type SOFIRQ_R = crate::BitReader<bool>;
#[doc = "Field `SOFIRQ` writer - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type SOFIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `DIRQ` reader - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type DIRQ_R = crate::BitReader<bool>;
#[doc = "Field `DIRQ` writer - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type DIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `CNNIRQ` reader - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CNNIRQ_R = crate::BitReader<bool>;
#[doc = "Field `CNNIRQ` writer - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CNNIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `CMPIRQ` reader - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type CMPIRQ_R = crate::BitReader<bool>;
#[doc = "Field `CMPIRQ` writer - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
pub type CMPIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `URIRQ` reader - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type URIRQ_R = crate::BitReader<bool>;
#[doc = "Field `URIRQ` writer - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type URIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `RWKIRQ` reader - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RWKIRQ_R = crate::BitReader<bool>;
#[doc = "Field `RWKIRQ` writer - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RWKIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type RSVD_6_R = crate::BitReader<bool>;
#[doc = "Field `RSVD_6` writer - N/A"]
pub type RSVD_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
#[doc = "Field `TCAN` reader - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TCAN_R = crate::BitReader<bool>;
#[doc = "Field `TCAN` writer - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_USBHOST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn sofirq(&self) -> SOFIRQ_R {
        SOFIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn cnnirq(&self) -> CNNIRQ_R {
        CNNIRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn urirq(&self) -> URIRQ_R {
        URIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rwkirq(&self) -> RWKIRQ_R {
        RWKIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> RSVD_6_R {
        RSVD_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tcan(&self) -> TCAN_R {
        TCAN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', it means that SOF token sending is started. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not issue an interrupt request by starting a SOF token. '1' : Issues an interrupt request by starting a SOF token. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn sofirq(&mut self) -> SOFIRQ_W<0> {
        SOFIRQ_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set to '1', it means that a device disconnection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device disconnection. '1' : Issues an interrupt request by detecting a device disconnection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn dirq(&mut self) -> DIRQ_W<1> {
        DIRQ_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a device connection is detected. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by detecting a device connection. '1' : Issues an interrupt request by detecting a device connection. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn cnnirq(&mut self) -> CNNIRQ_W<2> {
        CNNIRQ_W::new(self)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that a token is completed. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by token completion. '1' : Issues an interrupt request by token completion. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - This bit is not set to '1' even if the TCAN bit of the Interrupt USBHost Register (INTR_USBHOST) changes to '1'. - Take the following steps when this bit is set to '1' by finishing IN token or Isochronous IN token. 1. Read HS bit of Host Error Status Register (HOST_ERR), then set CMPIRQ bit to '0'. 2. Set EPn bit of Host DMA Enable Register (HOST_DMA_ENBL) (n=1 or 2) to '1' if HS bit of Host Error Status Register (HOST_ERR) is equal to '00' and wait until EPn bit of Host DMA Data Request Register (HOST_DMA_DREQ) changes to '1'. Finish the IN token processing if HS bit is not equal to '00'. 3. Read the received data if EPn bit of Host DMA Data Requet (HOST_DMA_DREQ) (n=1 or 2) changes to '1'."]
    #[inline(always)]
    pub fn cmpirq(&mut self) -> CMPIRQ_W<3> {
        CMPIRQ_W::new(self)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that USB bus resetting is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by USB bus resetting. '1' : Issues an interrupt request by USB bus resetting. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn urirq(&mut self) -> URIRQ_W<4> {
        URIRQ_W::new(self)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that remote Wake-up is ended. When this bit is '0', it has no meaning. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Issues no interrupt request by restart. '1' : Issues an interrupt request by restart. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rwkirq(&mut self) -> RWKIRQ_W<5> {
        RWKIRQ_W::new(self)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&mut self) -> RSVD_6_W<6> {
        RSVD_6_W::new(self)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that token sending is canceled based on the setting of the CANCEL bit of Host Control 2 Register (HOST_CTL2). When this bit is '0', it means that token sending is not canceled. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Does not cancel token sending. '1' : Cancels token sending. Note : - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tcan(&mut self) -> TCAN_W<7> {
        TCAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt USB Host Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_usbhost](index.html) module"]
pub struct INTR_USBHOST_SPEC;
impl crate::RegisterSpec for INTR_USBHOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_usbhost::R](R) reader structure"]
impl crate::Readable for INTR_USBHOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_usbhost::W](W) writer structure"]
impl crate::Writable for INTR_USBHOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_USBHOST to value 0"]
impl crate::Resettable for INTR_USBHOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
