#[doc = "Reader of register CTB_SW_STATUS"]
pub type R = crate::R<u32, super::CTB_SW_STATUS>;
#[doc = "Reader of field `OA0O_D51_STAT`"]
pub type OA0O_D51_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OA1O_D52_STAT`"]
pub type OA1O_D52_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OA1O_D62_STAT`"]
pub type OA1O_D62_STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTD_COS_STAT`"]
pub type CTD_COS_STAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 28 - see OA0O_D51 bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51_stat(&self) -> OA0O_D51_STAT_R {
        OA0O_D51_STAT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - see OA1O_D52 bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52_stat(&self) -> OA1O_D52_STAT_R {
        OA1O_D52_STAT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - see OA1O_D62 bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62_stat(&self) -> OA1O_D62_STAT_R {
        OA1O_D62_STAT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - see COS bit in CTD_SW"]
    #[inline(always)]
    pub fn ctd_cos_stat(&self) -> CTD_COS_STAT_R {
        CTD_COS_STAT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
