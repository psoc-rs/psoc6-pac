#[doc = "Register `ADV_ACCADDR_H` reader"]
pub struct R(crate::R<ADV_ACCADDR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_ACCADDR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_ACCADDR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_ACCADDR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_ACCADDR_H` writer"]
pub struct W(crate::W<ADV_ACCADDR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_ACCADDR_H_SPEC>;
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
impl From<crate::W<ADV_ACCADDR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_ACCADDR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_ACCADDR_H` reader - higher 16 bit of ADV packet access code"]
pub type ADV_ACCADDR_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADV_ACCADDR_H` writer - higher 16 bit of ADV packet access code"]
pub type ADV_ACCADDR_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADV_ACCADDR_H_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - higher 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_h(&self) -> ADV_ACCADDR_H_R {
        ADV_ACCADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - higher 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_h(&mut self) -> ADV_ACCADDR_H_W<0> {
        ADV_ACCADDR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADV packet access code high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_accaddr_h](index.html) module"]
pub struct ADV_ACCADDR_H_SPEC;
impl crate::RegisterSpec for ADV_ACCADDR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_accaddr_h::R](R) reader structure"]
impl crate::Readable for ADV_ACCADDR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_accaddr_h::W](W) writer structure"]
impl crate::Writable for ADV_ACCADDR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_ACCADDR_H to value 0x8e89"]
impl crate::Resettable for ADV_ACCADDR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8e89
    }
}
