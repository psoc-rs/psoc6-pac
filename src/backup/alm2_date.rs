#[doc = "Register `ALM2_DATE` reader"]
pub struct R(crate::R<ALM2_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALM2_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALM2_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALM2_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALM2_DATE` writer"]
pub struct W(crate::W<ALM2_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALM2_DATE_SPEC>;
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
impl From<crate::W<ALM2_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALM2_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALM_DATE` reader - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type ALM_DATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_DATE` writer - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type ALM_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM2_DATE_SPEC, u8, u8, 6, O>;
#[doc = "Field `ALM_DATE_EN` reader - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type ALM_DATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_DATE_EN` writer - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type ALM_DATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM2_DATE_SPEC, bool, O>;
#[doc = "Field `ALM_MON` reader - Alarm Month in BCD, 1-12"]
pub type ALM_MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_MON` writer - Alarm Month in BCD, 1-12"]
pub type ALM_MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM2_DATE_SPEC, u8, u8, 5, O>;
#[doc = "Field `ALM_MON_EN` reader - Alarm Month enable: 0=ignore, 1=match"]
pub type ALM_MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_MON_EN` writer - Alarm Month enable: 0=ignore, 1=match"]
pub type ALM_MON_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM2_DATE_SPEC, bool, O>;
#[doc = "Field `ALM_EN` reader - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type ALM_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_EN` writer - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type ALM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM2_DATE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&self) -> ALM_DATE_R {
        ALM_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&self) -> ALM_DATE_EN_R {
        ALM_DATE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&self) -> ALM_MON_R {
        ALM_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&self) -> ALM_MON_EN_R {
        ALM_MON_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&self) -> ALM_EN_R {
        ALM_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&mut self) -> ALM_DATE_W<0> {
        ALM_DATE_W::new(self)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&mut self) -> ALM_DATE_EN_W<7> {
        ALM_DATE_EN_W::new(self)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&mut self) -> ALM_MON_W<8> {
        ALM_MON_W::new(self)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&mut self) -> ALM_MON_EN_W<15> {
        ALM_MON_EN_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&mut self) -> ALM_EN_W<31> {
        ALM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 2 Day of Month, Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm2_date](index.html) module"]
pub struct ALM2_DATE_SPEC;
impl crate::RegisterSpec for ALM2_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alm2_date::R](R) reader structure"]
impl crate::Readable for ALM2_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alm2_date::W](W) writer structure"]
impl crate::Writable for ALM2_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALM2_DATE to value 0x0101"]
impl crate::Resettable for ALM2_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
