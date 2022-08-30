#[doc = "Register `HOST_CTL1` reader"]
pub struct R(crate::R<HOST_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_CTL1` writer"]
pub struct W(crate::W<HOST_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_CTL1_SPEC>;
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
impl From<crate::W<HOST_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLKSEL` writer - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL1_SPEC, bool, O>;
#[doc = "Field `USTP` reader - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type USTP_R = crate::BitReader<bool>;
#[doc = "Field `USTP` writer - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type USTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL1_SPEC, bool, O>;
#[doc = "Field `RST` reader - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&self) -> USTP_R {
        USTP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&mut self) -> USTP_W<1> {
        USTP_W::new(self)
    }
    #[doc = "Bit 7 - This bit resets this IP. '0' : Releases the reset for USB Host. '1' : Resets USB Host. Notes: - This bit is initialized if ENABLE bit of the Host Control 0 Register changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<7> {
        RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Control 1 Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctl1](index.html) module"]
pub struct HOST_CTL1_SPEC;
impl crate::RegisterSpec for HOST_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ctl1::R](R) reader structure"]
impl crate::Readable for HOST_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ctl1::W](W) writer structure"]
impl crate::Writable for HOST_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_CTL1 to value 0x83"]
impl crate::Resettable for HOST_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
