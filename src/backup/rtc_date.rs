#[doc = "Reader of register RTC_DATE"]
pub type R = crate::R<u32, super::RTC_DATE>;
#[doc = "Writer for register RTC_DATE"]
pub type W = crate::W<u32, super::RTC_DATE>;
#[doc = "Register RTC_DATE `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::RTC_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `RTC_DATE`"]
pub type RTC_DATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_DATE`"]
pub struct RTC_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RTC_MON`"]
pub type RTC_MON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_MON`"]
pub struct RTC_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_YEAR`"]
pub type RTC_YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_YEAR`"]
pub struct RTC_YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub fn rtc_date(&self) -> RTC_DATE_R {
        RTC_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn rtc_mon(&self) -> RTC_MON_R {
        RTC_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn rtc_year(&self) -> RTC_YEAR_R {
        RTC_YEAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub fn rtc_date(&mut self) -> RTC_DATE_W {
        RTC_DATE_W { w: self }
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn rtc_mon(&mut self) -> RTC_MON_W {
        RTC_MON_W { w: self }
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn rtc_year(&mut self) -> RTC_YEAR_W {
        RTC_YEAR_W { w: self }
    }
}
