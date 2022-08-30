#[doc = "Register `US_COUNTER` reader"]
pub struct R(crate::R<US_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `US_COUNTER` reader - Current value of the US Counter"]
pub type US_COUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Current value of the US Counter"]
    #[inline(always)]
    pub fn us_counter(&self) -> US_COUNTER_R {
        US_COUNTER_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Running US of the current BT Slot\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_counter](index.html) module"]
pub struct US_COUNTER_SPEC;
impl crate::RegisterSpec for US_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_counter::R](R) reader structure"]
impl crate::Readable for US_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_COUNTER to value 0"]
impl crate::Resettable for US_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
