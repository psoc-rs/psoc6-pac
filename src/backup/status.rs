#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RTC_BUSY`"]
pub type RTC_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `WCO_OK`"]
pub type WCO_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - pending RTC write"]
    #[inline(always)]
    pub fn rtc_busy(&self) -> RTC_BUSY_R {
        RTC_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that output has transitioned."]
    #[inline(always)]
    pub fn wco_ok(&self) -> WCO_OK_R {
        WCO_OK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
