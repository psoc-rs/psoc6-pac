#[doc = "Register `WL_ADDR_TYPE` reader"]
pub struct R(crate::R<WL_ADDR_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WL_ADDR_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WL_ADDR_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WL_ADDR_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WL_ADDR_TYPE` writer"]
pub struct W(crate::W<WL_ADDR_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WL_ADDR_TYPE_SPEC>;
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
impl From<crate::W<WL_ADDR_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WL_ADDR_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WL_ADDR_TYPE` reader - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
pub type WL_ADDR_TYPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WL_ADDR_TYPE` writer - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
pub type WL_ADDR_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WL_ADDR_TYPE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn wl_addr_type(&self) -> WL_ADDR_TYPE_R {
        WL_ADDR_TYPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn wl_addr_type(&mut self) -> WL_ADDR_TYPE_W<0> {
        WL_ADDR_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "whitelist address type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_addr_type](index.html) module"]
pub struct WL_ADDR_TYPE_SPEC;
impl crate::RegisterSpec for WL_ADDR_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wl_addr_type::R](R) reader structure"]
impl crate::Readable for WL_ADDR_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wl_addr_type::W](W) writer structure"]
impl crate::Writable for WL_ADDR_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WL_ADDR_TYPE to value 0"]
impl crate::Resettable for WL_ADDR_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
