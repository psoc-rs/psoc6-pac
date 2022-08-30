#[doc = "Register `CAL_CTL` reader"]
pub struct R(crate::R<CAL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL` writer"]
pub struct W(crate::W<CAL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL_SPEC>;
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
impl From<crate::W<CAL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALIB_VAL` reader - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CALIB_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALIB_VAL` writer - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
pub type CALIB_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CALIB_SIGN` reader - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_R = crate::BitReader<bool>;
#[doc = "Field `CALIB_SIGN` writer - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
pub type CALIB_SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL_SPEC, bool, O>;
#[doc = "Field `CAL_OUT` reader - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CAL_OUT_R = crate::BitReader<bool>;
#[doc = "Field `CAL_OUT` writer - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
pub type CAL_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&self) -> CALIB_VAL_R {
        CALIB_VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&self) -> CALIB_SIGN_R {
        CALIB_SIGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&self) -> CAL_OUT_R {
        CAL_OUT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&mut self) -> CALIB_VAL_W<0> {
        CALIB_VAL_W::new(self)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&mut self) -> CALIB_SIGN_W<6> {
        CALIB_SIGN_W::new(self)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&mut self) -> CAL_OUT_W<31> {
        CAL_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator calibration for absolute frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl](index.html) module"]
pub struct CAL_CTL_SPEC;
impl crate::RegisterSpec for CAL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl::R](R) reader structure"]
impl crate::Readable for CAL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl::W](W) writer structure"]
impl crate::Writable for CAL_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL_CTL to value 0"]
impl crate::Resettable for CAL_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
