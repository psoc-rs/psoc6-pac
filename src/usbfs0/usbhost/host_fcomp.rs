#[doc = "Register `HOST_FCOMP` reader"]
pub struct R(crate::R<HOST_FCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_FCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_FCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_FCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_FCOMP` writer"]
pub struct W(crate::W<HOST_FCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_FCOMP_SPEC>;
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
impl From<crate::W<HOST_FCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_FCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMECOMP` reader - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FRAMECOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAMECOMP` writer - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FRAMECOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_FCOMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&self) -> FRAMECOMP_R {
        FRAMECOMP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&mut self) -> FRAMECOMP_W<0> {
        FRAMECOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host SOF Interrupt Frame Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_fcomp](index.html) module"]
pub struct HOST_FCOMP_SPEC;
impl crate::RegisterSpec for HOST_FCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_fcomp::R](R) reader structure"]
impl crate::Readable for HOST_FCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_fcomp::W](W) writer structure"]
impl crate::Writable for HOST_FCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_FCOMP to value 0"]
impl crate::Resettable for HOST_FCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
