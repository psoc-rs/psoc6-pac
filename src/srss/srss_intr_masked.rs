#[doc = "Reader of register SRSS_INTR_MASKED"]
pub type R = crate::R<u32, super::SRSS_INTR_MASKED>;
#[doc = "Reader of field `WDT_MATCH`"]
pub type WDT_MATCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `HVLVD1`"]
pub type HVLVD1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_CAL`"]
pub type CLK_CAL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn hvlvd1(&self) -> HVLVD1_R {
        HVLVD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn clk_cal(&self) -> CLK_CAL_R {
        CLK_CAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
