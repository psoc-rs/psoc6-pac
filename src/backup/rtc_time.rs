#[doc = "Reader of register RTC_TIME"]
pub type R = crate::R<u32, super::RTC_TIME>;
#[doc = "Writer for register RTC_TIME"]
pub type W = crate::W<u32, super::RTC_TIME>;
#[doc = "Register RTC_TIME `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::RTC_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `RTC_SEC`"]
pub type RTC_SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_SEC`"]
pub struct RTC_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RTC_MIN`"]
pub type RTC_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_MIN`"]
pub struct RTC_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTC_HOUR`"]
pub type RTC_HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_HOUR`"]
pub struct RTC_HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTRL_12HR`"]
pub type CTRL_12HR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_12HR`"]
pub struct CTRL_12HR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_12HR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `RTC_DAY`"]
pub type RTC_DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_DAY`"]
pub struct RTC_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&self) -> RTC_SEC_R {
        RTC_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&self) -> RTC_MIN_R {
        RTC_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&self) -> RTC_HOUR_R {
        RTC_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&self) -> CTRL_12HR_R {
        CTRL_12HR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&self) -> RTC_DAY_R {
        RTC_DAY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&mut self) -> RTC_SEC_W {
        RTC_SEC_W { w: self }
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&mut self) -> RTC_MIN_W {
        RTC_MIN_W { w: self }
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&mut self) -> RTC_HOUR_W {
        RTC_HOUR_W { w: self }
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&mut self) -> CTRL_12HR_W {
        CTRL_12HR_W { w: self }
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&mut self) -> RTC_DAY_W {
        RTC_DAY_W { w: self }
    }
}
