#[doc = "Register `HOST_CTL0` reader"]
pub struct R(crate::R<HOST_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CTL0` writer"]
pub struct W(crate::W<HOST_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTL0_SPEC>;
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
impl From<crate::W<HOST_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST` reader - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HOST_R = crate::BitReader<bool>;
#[doc = "Field `HOST` writer - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL0_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The operation mode does not transition to the required one immediately after it was changed using this bit. Read this bit to check that the operation mode has changed. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'.. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub fn host(&mut self) -> HOST_W<0> {
        HOST_W::new(self)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn' affect the USB Device."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 0 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl0](index.html) module"]
pub struct HOST_CTL0_SPEC;
impl crate::RegisterSpec for HOST_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ctl0::R](R) reader structure"]
impl crate::Readable for HOST_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctl0::W](W) writer structure"]
impl crate::Writable for HOST_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_CTL0 to value 0"]
impl crate::Resettable for HOST_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
