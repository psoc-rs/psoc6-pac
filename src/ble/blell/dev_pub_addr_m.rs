#[doc = "Register `DEV_PUB_ADDR_M` reader"]
pub struct R(crate::R<DEV_PUB_ADDR_M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_PUB_ADDR_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_PUB_ADDR_M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_PUB_ADDR_M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_PUB_ADDR_M` writer"]
pub struct W(crate::W<DEV_PUB_ADDR_M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_PUB_ADDR_M_SPEC>;
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
impl From<crate::W<DEV_PUB_ADDR_M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_PUB_ADDR_M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_PUB_ADDR_M` reader - Middle 16 bit of 48-bit public address of the device."]
pub type DEV_PUB_ADDR_M_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEV_PUB_ADDR_M` writer - Middle 16 bit of 48-bit public address of the device."]
pub type DEV_PUB_ADDR_M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEV_PUB_ADDR_M_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit public address of the device."]
    #[inline(always)]
    pub fn dev_pub_addr_m(&self) -> DEV_PUB_ADDR_M_R {
        DEV_PUB_ADDR_M_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Middle 16 bit of 48-bit public address of the device."]
    #[inline(always)]
    pub fn dev_pub_addr_m(&mut self) -> DEV_PUB_ADDR_M_W<0> {
        DEV_PUB_ADDR_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device public address middle register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_pub_addr_m](index.html) module"]
pub struct DEV_PUB_ADDR_M_SPEC;
impl crate::RegisterSpec for DEV_PUB_ADDR_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_pub_addr_m::R](R) reader structure"]
impl crate::Readable for DEV_PUB_ADDR_M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_pub_addr_m::W](W) writer structure"]
impl crate::Writable for DEV_PUB_ADDR_M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_PUB_ADDR_M to value 0x56"]
impl crate::Resettable for DEV_PUB_ADDR_M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x56
    }
}
