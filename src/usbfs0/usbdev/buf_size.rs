#[doc = "Register `BUF_SIZE` reader"]
pub struct R(crate::R<BUF_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_SIZE` writer"]
pub struct W(crate::W<BUF_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_SIZE_SPEC>;
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
impl From<crate::W<BUF_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_BUF` reader - Buffer size for IN Endpoints."]
pub type IN_BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_BUF` writer - Buffer size for IN Endpoints."]
pub type IN_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_SIZE_SPEC, u8, u8, 4, O>;
#[doc = "Field `OUT_BUF` reader - Buffer size for OUT Endpoints."]
pub type OUT_BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_BUF` writer - Buffer size for OUT Endpoints."]
pub type OUT_BUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUF_SIZE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&self) -> IN_BUF_R {
        IN_BUF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&self) -> OUT_BUF_R {
        OUT_BUF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&mut self) -> IN_BUF_W<0> {
        IN_BUF_W::new(self)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&mut self) -> OUT_BUF_W<4> {
        OUT_BUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated Endpoint Buffer Size Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_size](index.html) module"]
pub struct BUF_SIZE_SPEC;
impl crate::RegisterSpec for BUF_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_size::R](R) reader structure"]
impl crate::Readable for BUF_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_size::W](W) writer structure"]
impl crate::Writable for BUF_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_SIZE to value 0"]
impl crate::Resettable for BUF_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
