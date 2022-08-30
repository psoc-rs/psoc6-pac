#[doc = "Register `SRSS_INTR_MASKED` reader"]
pub struct R(crate::R<SRSS_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDT_MATCH` reader - Logical and of corresponding request and mask bits."]
pub type WDT_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `HVLVD1` reader - Logical and of corresponding request and mask bits."]
pub type HVLVD1_R = crate::BitReader<bool>;
#[doc = "Field `CLK_CAL` reader - Logical and of corresponding request and mask bits."]
pub type CLK_CAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SRSS Interrupt Masked Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_masked](index.html) module"]
pub struct SRSS_INTR_MASKED_SPEC;
impl crate::RegisterSpec for SRSS_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_intr_masked::R](R) reader structure"]
impl crate::Readable for SRSS_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRSS_INTR_MASKED to value 0"]
impl crate::Resettable for SRSS_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
