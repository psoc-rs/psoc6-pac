#[doc = "Register `IV_MASTER0` reader"]
pub struct R(crate::R<IV_MASTER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV_MASTER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV_MASTER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV_MASTER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV_MASTER0` writer"]
pub struct W(crate::W<IV_MASTER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV_MASTER0_SPEC>;
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
impl From<crate::W<IV_MASTER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV_MASTER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV_MASTER` reader - This is the IVm field, which contains the master's portion of the initialization vector."]
pub type IV_MASTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV_MASTER` writer - This is the IVm field, which contains the master's portion of the initialization vector."]
pub type IV_MASTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IV_MASTER0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This is the IVm field, which contains the master's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_master(&self) -> IV_MASTER_R {
        IV_MASTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the IVm field, which contains the master's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_master(&mut self) -> IV_MASTER_W<0> {
        IV_MASTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Initialization Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv_master0](index.html) module"]
pub struct IV_MASTER0_SPEC;
impl crate::RegisterSpec for IV_MASTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv_master0::R](R) reader structure"]
impl crate::Readable for IV_MASTER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv_master0::W](W) writer structure"]
impl crate::Writable for IV_MASTER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV_MASTER0 to value 0"]
impl crate::Resettable for IV_MASTER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
