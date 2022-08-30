#[doc = "Register `ALM1_TIME` reader"]
pub struct R(crate::R<ALM1_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALM1_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALM1_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALM1_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALM1_TIME` writer"]
pub struct W(crate::W<ALM1_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALM1_TIME_SPEC>;
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
impl From<crate::W<ALM1_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALM1_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALM_SEC` reader - Alarm seconds in BCD, 0-59"]
pub type ALM_SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_SEC` writer - Alarm seconds in BCD, 0-59"]
pub type ALM_SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM1_TIME_SPEC, u8, u8, 7, O>;
#[doc = "Field `ALM_SEC_EN` reader - Alarm second enable: 0=ignore, 1=match"]
pub type ALM_SEC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_SEC_EN` writer - Alarm second enable: 0=ignore, 1=match"]
pub type ALM_SEC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM1_TIME_SPEC, bool, O>;
#[doc = "Field `ALM_MIN` reader - Alarm minutes in BCD, 0-59"]
pub type ALM_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_MIN` writer - Alarm minutes in BCD, 0-59"]
pub type ALM_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM1_TIME_SPEC, u8, u8, 7, O>;
#[doc = "Field `ALM_MIN_EN` reader - Alarm minutes enable: 0=ignore, 1=match"]
pub type ALM_MIN_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_MIN_EN` writer - Alarm minutes enable: 0=ignore, 1=match"]
pub type ALM_MIN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM1_TIME_SPEC, bool, O>;
#[doc = "Field `ALM_HOUR` reader - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type ALM_HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_HOUR` writer - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type ALM_HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM1_TIME_SPEC, u8, u8, 6, O>;
#[doc = "Field `ALM_HOUR_EN` reader - Alarm hour enable: 0=ignore, 1=match"]
pub type ALM_HOUR_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_HOUR_EN` writer - Alarm hour enable: 0=ignore, 1=match"]
pub type ALM_HOUR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM1_TIME_SPEC, bool, O>;
#[doc = "Field `ALM_DAY` reader - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type ALM_DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALM_DAY` writer - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type ALM_DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALM1_TIME_SPEC, u8, u8, 3, O>;
#[doc = "Field `ALM_DAY_EN` reader - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type ALM_DAY_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_DAY_EN` writer - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type ALM_DAY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALM1_TIME_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&self) -> ALM_SEC_R {
        ALM_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&self) -> ALM_SEC_EN_R {
        ALM_SEC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&self) -> ALM_MIN_R {
        ALM_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&self) -> ALM_MIN_EN_R {
        ALM_MIN_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&self) -> ALM_HOUR_R {
        ALM_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&self) -> ALM_HOUR_EN_R {
        ALM_HOUR_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&self) -> ALM_DAY_R {
        ALM_DAY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&self) -> ALM_DAY_EN_R {
        ALM_DAY_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&mut self) -> ALM_SEC_W<0> {
        ALM_SEC_W::new(self)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&mut self) -> ALM_SEC_EN_W<7> {
        ALM_SEC_EN_W::new(self)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&mut self) -> ALM_MIN_W<8> {
        ALM_MIN_W::new(self)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&mut self) -> ALM_MIN_EN_W<15> {
        ALM_MIN_EN_W::new(self)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&mut self) -> ALM_HOUR_W<16> {
        ALM_HOUR_W::new(self)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&mut self) -> ALM_HOUR_EN_W<23> {
        ALM_HOUR_EN_W::new(self)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&mut self) -> ALM_DAY_W<24> {
        ALM_DAY_W::new(self)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&mut self) -> ALM_DAY_EN_W<31> {
        ALM_DAY_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alm1_time](index.html) module"]
pub struct ALM1_TIME_SPEC;
impl crate::RegisterSpec for ALM1_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alm1_time::R](R) reader structure"]
impl crate::Readable for ALM1_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alm1_time::W](W) writer structure"]
impl crate::Writable for ALM1_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALM1_TIME to value 0x0100_0000"]
impl crate::Resettable for ALM1_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
