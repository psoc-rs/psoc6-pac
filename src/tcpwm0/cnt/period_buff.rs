#[doc = "Register `PERIOD_BUFF` reader"]
pub struct R(crate::R<PERIOD_BUFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_BUFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_BUFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_BUFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIOD_BUFF` writer"]
pub struct W(crate::W<PERIOD_BUFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_BUFF_SPEC>;
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
impl From<crate::W<PERIOD_BUFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_BUFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Additional buffer for counter PERIOD register."]
pub type PERIOD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERIOD` writer - Additional buffer for counter PERIOD register."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERIOD_BUFF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter buffered period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_buff](index.html) module"]
pub struct PERIOD_BUFF_SPEC;
impl crate::RegisterSpec for PERIOD_BUFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period_buff::R](R) reader structure"]
impl crate::Readable for PERIOD_BUFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period_buff::W](W) writer structure"]
impl crate::Writable for PERIOD_BUFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIOD_BUFF to value 0xffff_ffff"]
impl crate::Resettable for PERIOD_BUFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
