#[doc = "Register `WL_ENABLE` reader"]
pub struct R(crate::R<WL_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WL_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WL_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WL_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WL_ENABLE` writer"]
pub struct W(crate::W<WL_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WL_ENABLE_SPEC>;
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
impl From<crate::W<WL_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WL_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WL_ENABLE` reader - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
pub type WL_ENABLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WL_ENABLE` writer - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
pub type WL_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WL_ENABLE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&self) -> WL_ENABLE_R {
        WL_ENABLE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&mut self) -> WL_ENABLE_W<0> {
        WL_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "whitelist valid entry bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wl_enable](index.html) module"]
pub struct WL_ENABLE_SPEC;
impl crate::RegisterSpec for WL_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wl_enable::R](R) reader structure"]
impl crate::Readable for WL_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wl_enable::W](W) writer structure"]
impl crate::Writable for WL_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WL_ENABLE to value 0"]
impl crate::Resettable for WL_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
