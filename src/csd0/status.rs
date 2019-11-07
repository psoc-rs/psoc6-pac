#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `CSD_SENSE`"]
pub type CSD_SENSE_R = crate::R<bool, bool>;
#[doc = "Only for Debug/test purpose the output status of CSD comparator can be read by CPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSCMP_OUT_A {
    #[doc = "0: N/A"]
    C_LT_VREF,
    #[doc = "1: N/A"]
    C_GT_VREF,
}
impl From<HSCMP_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: HSCMP_OUT_A) -> Self {
        match variant {
            HSCMP_OUT_A::C_LT_VREF => false,
            HSCMP_OUT_A::C_GT_VREF => true,
        }
    }
}
#[doc = "Reader of field `HSCMP_OUT`"]
pub type HSCMP_OUT_R = crate::R<bool, HSCMP_OUT_A>;
impl HSCMP_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSCMP_OUT_A {
        match self.bits {
            false => HSCMP_OUT_A::C_LT_VREF,
            true => HSCMP_OUT_A::C_GT_VREF,
        }
    }
    #[doc = "Checks if the value of the field is `C_LT_VREF`"]
    #[inline(always)]
    pub fn is_c_lt_vref(&self) -> bool {
        *self == HSCMP_OUT_A::C_LT_VREF
    }
    #[doc = "Checks if the value of the field is `C_GT_VREF`"]
    #[inline(always)]
    pub fn is_c_gt_vref(&self) -> bool {
        *self == HSCMP_OUT_A::C_GT_VREF
    }
}
#[doc = "Reader of field `CSDCMP_OUT`"]
pub type CSDCMP_OUT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Only for Debug/test purpose this internal signal (sensor clock) status can be read by CPU"]
    #[inline(always)]
    pub fn csd_sense(&self) -> CSD_SENSE_R {
        CSD_SENSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Only for Debug/test purpose the output status of CSD comparator can be read by CPU"]
    #[inline(always)]
    pub fn hscmp_out(&self) -> HSCMP_OUT_R {
        HSCMP_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Only for Debug/test purpose the output status of CSD modulator can be read by CPU"]
    #[inline(always)]
    pub fn csdcmp_out(&self) -> CSDCMP_OUT_R {
        CSDCMP_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
