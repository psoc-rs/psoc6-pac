#[doc = "Register `WHITELIST_BASE_ADDR` reader"]
pub struct R(crate::R<WHITELIST_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WHITELIST_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WHITELIST_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WHITELIST_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WHITELIST_BASE_ADDR` writer"]
pub struct W(crate::W<WHITELIST_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WHITELIST_BASE_ADDR_SPEC>;
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
impl From<crate::W<WHITELIST_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WHITELIST_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WL_BASE_ADDR` reader - Device address values written to white list memory are written as 16-bit wide address."]
pub type WL_BASE_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WL_BASE_ADDR` writer - Device address values written to white list memory are written as 16-bit wide address."]
pub type WL_BASE_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WHITELIST_BASE_ADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Device address values written to white list memory are written as 16-bit wide address."]
    #[inline(always)]
    pub fn wl_base_addr(&self) -> WL_BASE_ADDR_R {
        WL_BASE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address values written to white list memory are written as 16-bit wide address."]
    #[inline(always)]
    pub fn wl_base_addr(&mut self) -> WL_BASE_ADDR_W<0> {
        WL_BASE_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whitelist base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [whitelist_base_addr](index.html) module"]
pub struct WHITELIST_BASE_ADDR_SPEC;
impl crate::RegisterSpec for WHITELIST_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [whitelist_base_addr::R](R) reader structure"]
impl crate::Readable for WHITELIST_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [whitelist_base_addr::W](W) writer structure"]
impl crate::Writable for WHITELIST_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WHITELIST_BASE_ADDR to value 0"]
impl crate::Resettable for WHITELIST_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
