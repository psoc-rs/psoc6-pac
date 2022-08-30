#[doc = "Register `HOST_FRAME` reader"]
pub struct R(crate::R<HOST_FRAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_FRAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_FRAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_FRAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_FRAME` writer"]
pub struct W(crate::W<HOST_FRAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_FRAME_SPEC>;
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
impl From<crate::W<HOST_FRAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_FRAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME` reader - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FRAME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAME` writer - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FRAME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_FRAME_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<0> {
        FRAME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Frame Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_frame](index.html) module"]
pub struct HOST_FRAME_SPEC;
impl crate::RegisterSpec for HOST_FRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_frame::R](R) reader structure"]
impl crate::Readable for HOST_FRAME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_frame::W](W) writer structure"]
impl crate::Writable for HOST_FRAME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_FRAME to value 0"]
impl crate::Resettable for HOST_FRAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
