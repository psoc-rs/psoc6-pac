#[doc = "Register `HW_LOAD_OFFSET` reader"]
pub struct R(crate::R<HW_LOAD_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_LOAD_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_LOAD_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_LOAD_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_LOAD_OFFSET` writer"]
pub struct W(crate::W<HW_LOAD_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_LOAD_OFFSET_SPEC>;
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
impl From<crate::W<HW_LOAD_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_LOAD_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_OFFSET` reader - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
pub type LOAD_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOAD_OFFSET` writer - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
pub type LOAD_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_LOAD_OFFSET_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
    #[inline(always)]
    pub fn load_offset(&self) -> LOAD_OFFSET_R {
        LOAD_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
    #[inline(always)]
    pub fn load_offset(&mut self) -> LOAD_OFFSET_W<0> {
        LOAD_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register to configure offset from connection anchor point at which connection parameter memory should be read\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_load_offset](index.html) module"]
pub struct HW_LOAD_OFFSET_SPEC;
impl crate::RegisterSpec for HW_LOAD_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_load_offset::R](R) reader structure"]
impl crate::Readable for HW_LOAD_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_load_offset::W](W) writer structure"]
impl crate::Writable for HW_LOAD_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HW_LOAD_OFFSET to value 0x04"]
impl crate::Resettable for HW_LOAD_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
