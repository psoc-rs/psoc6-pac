#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAMPLE` reader - Logical and of corresponding request and mask bits."]
pub type SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `INIT` reader - Logical and of corresponding request and mask bits."]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `ADC_RES` reader - Logical and of corresponding request and mask bits."]
pub type ADC_RES_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn adc_res(&self) -> ADC_RES_R {
        ADC_RES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "CSD Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
