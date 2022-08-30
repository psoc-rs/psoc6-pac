#[doc = "Register `DEVICE_RAND_ADDR_L` reader"]
pub struct R(crate::R<DEVICE_RAND_ADDR_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_RAND_ADDR_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_RAND_ADDR_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_RAND_ADDR_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICE_RAND_ADDR_L` writer"]
pub struct W(crate::W<DEVICE_RAND_ADDR_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICE_RAND_ADDR_L_SPEC>;
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
impl From<crate::W<DEVICE_RAND_ADDR_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICE_RAND_ADDR_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVICE_RAND_ADDR_L` reader - Lower 16 bit of 48-bit random address of the device."]
pub type DEVICE_RAND_ADDR_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVICE_RAND_ADDR_L` writer - Lower 16 bit of 48-bit random address of the device."]
pub type DEVICE_RAND_ADDR_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICE_RAND_ADDR_L_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit random address of the device."]
    #[inline(always)]
    pub fn device_rand_addr_l(&self) -> DEVICE_RAND_ADDR_L_R {
        DEVICE_RAND_ADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit random address of the device."]
    #[inline(always)]
    pub fn device_rand_addr_l(&mut self) -> DEVICE_RAND_ADDR_L_W<0> {
        DEVICE_RAND_ADDR_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower 16 bit random address of the device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_rand_addr_l](index.html) module"]
pub struct DEVICE_RAND_ADDR_L_SPEC;
impl crate::RegisterSpec for DEVICE_RAND_ADDR_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_rand_addr_l::R](R) reader structure"]
impl crate::Readable for DEVICE_RAND_ADDR_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [device_rand_addr_l::W](W) writer structure"]
impl crate::Writable for DEVICE_RAND_ADDR_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICE_RAND_ADDR_L to value 0"]
impl crate::Resettable for DEVICE_RAND_ADDR_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
