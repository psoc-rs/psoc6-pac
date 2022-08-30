#[doc = "Register `CLK_ECO_STATUS` reader"]
pub struct R(crate::R<CLK_ECO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ECO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ECO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ECO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECO_OK` reader - Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
pub type ECO_OK_R = crate::BitReader<bool>;
#[doc = "Field `ECO_READY` reader - Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
pub type ECO_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the ECO internal oscillator circuit has sufficient amplitude. It may not meet the PPM accuracy or duty cycle spec."]
    #[inline(always)]
    pub fn eco_ok(&self) -> ECO_OK_R {
        ECO_OK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates the ECO internal oscillator circuit has had enough time to fully stabilize. This is the output of a counter since ECO was enabled, and it does not check the ECO output. It is recommended to also confirm ECO_OK==1."]
    #[inline(always)]
    pub fn eco_ready(&self) -> ECO_READY_R {
        ECO_READY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "ECO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_eco_status](index.html) module"]
pub struct CLK_ECO_STATUS_SPEC;
impl crate::RegisterSpec for CLK_ECO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_eco_status::R](R) reader structure"]
impl crate::Readable for CLK_ECO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_ECO_STATUS to value 0"]
impl crate::Resettable for CLK_ECO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
