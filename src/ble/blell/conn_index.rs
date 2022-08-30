#[doc = "Register `CONN_INDEX` reader"]
pub struct R(crate::R<CONN_INDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_INDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_INDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_INDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_INDEX` writer"]
pub struct W(crate::W<CONN_INDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_INDEX_SPEC>;
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
impl From<crate::W<CONN_INDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_INDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_INDEX` reader - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
pub type CONN_INDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONN_INDEX` writer - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
pub type CONN_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_INDEX_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
    #[inline(always)]
    pub fn conn_index(&self) -> CONN_INDEX_R {
        CONN_INDEX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
    #[inline(always)]
    pub fn conn_index(&mut self) -> CONN_INDEX_W<0> {
        CONN_INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_index](index.html) module"]
pub struct CONN_INDEX_SPEC;
impl crate::RegisterSpec for CONN_INDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_index::R](R) reader structure"]
impl crate::Readable for CONN_INDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_index::W](W) writer structure"]
impl crate::Writable for CONN_INDEX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_INDEX to value 0"]
impl crate::Resettable for CONN_INDEX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
