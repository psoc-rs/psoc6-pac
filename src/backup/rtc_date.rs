#[doc = "Register `RTC_DATE` reader"]
pub struct R(crate::R<RTC_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DATE` writer"]
pub struct W(crate::W<RTC_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DATE` reader - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RTC_DATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DATE` writer - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RTC_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_DATE_SPEC, u8, u8, 6, O>;
#[doc = "Field `RTC_MON` reader - Calendar Month in BCD, 1-12"]
pub type RTC_MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_MON` writer - Calendar Month in BCD, 1-12"]
pub type RTC_MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_DATE_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_YEAR` reader - Calendar year in BCD, 0-99"]
pub type RTC_YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_YEAR` writer - Calendar year in BCD, 0-99"]
pub type RTC_YEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_DATE_SPEC, u8, u8, 8, O>;
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
    pub fn rtc_date(&mut self) -> RTC_DATE_W<0> {
        RTC_DATE_W::new(self)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn rtc_mon(&mut self) -> RTC_MON_W<8> {
        RTC_MON_W::new(self)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn rtc_year(&mut self) -> RTC_YEAR_W<16> {
        RTC_YEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Day of Month, Month, Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_date](index.html) module"]
pub struct RTC_DATE_SPEC;
impl crate::RegisterSpec for RTC_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_date::R](R) reader structure"]
impl crate::Readable for RTC_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_date::W](W) writer structure"]
impl crate::Writable for RTC_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_DATE to value 0"]
impl crate::Resettable for RTC_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
