#[doc = "Register `COUNTER` reader"]
pub struct R(crate::R<COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER` writer"]
pub struct W(crate::W<COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_SPEC>;
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
impl From<crate::W<COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNTER` writer - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNTER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](index.html) module"]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter::R](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter::W](W) writer structure"]
impl crate::Writable for COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
