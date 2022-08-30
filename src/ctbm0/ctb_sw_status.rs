#[doc = "Register `CTB_SW_STATUS` reader"]
pub struct R(crate::R<CTB_SW_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTB_SW_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTB_SW_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTB_SW_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OA0O_D51_STAT` reader - see OA0O_D51 bit in OA0_SW"]
pub type OA0O_D51_STAT_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D52_STAT` reader - see OA1O_D52 bit in OA1_SW"]
pub type OA1O_D52_STAT_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D62_STAT` reader - see OA1O_D62 bit in OA1_SW"]
pub type OA1O_D62_STAT_R = crate::BitReader<bool>;
#[doc = "Field `CTD_COS_STAT` reader - see COS bit in CTD_SW"]
pub type CTD_COS_STAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 28 - see OA0O_D51 bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51_stat(&self) -> OA0O_D51_STAT_R {
        OA0O_D51_STAT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - see OA1O_D52 bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52_stat(&self) -> OA1O_D52_STAT_R {
        OA1O_D52_STAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - see OA1O_D62 bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62_stat(&self) -> OA1O_D62_STAT_R {
        OA1O_D62_STAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - see COS bit in CTD_SW"]
    #[inline(always)]
    pub fn ctd_cos_stat(&self) -> CTD_COS_STAT_R {
        CTD_COS_STAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CTB bus switch control status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_status](index.html) module"]
pub struct CTB_SW_STATUS_SPEC;
impl crate::RegisterSpec for CTB_SW_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctb_sw_status::R](R) reader structure"]
impl crate::Readable for CTB_SW_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTB_SW_STATUS to value 0"]
impl crate::Resettable for CTB_SW_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
