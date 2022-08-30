#[doc = "Register `TRIM_LDO_5` reader"]
pub struct R(crate::R<TRIM_LDO_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_5` writer"]
pub struct W(crate::W<TRIM_LDO_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_5_SPEC>;
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
impl From<crate::W<TRIM_LDO_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD` reader - N/A"]
pub type RSVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD` writer - N/A"]
pub type RSVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_LDO_5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RSVD_W<0> {
        RSVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_5](index.html) module"]
pub struct TRIM_LDO_5_SPEC;
impl crate::RegisterSpec for TRIM_LDO_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_5::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_5::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_5 to value 0"]
impl crate::Resettable for TRIM_LDO_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
