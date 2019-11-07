#[doc = "Reader of register CAL_CTL"]
pub type R = crate::R<u32, super::CAL_CTL>;
#[doc = "Writer for register CAL_CTL"]
pub type W = crate::W<u32, super::CAL_CTL>;
#[doc = "Register CAL_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALIB_VAL`"]
pub type CALIB_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALIB_VAL`"]
pub struct CALIB_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CALIB_SIGN`"]
pub type CALIB_SIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALIB_SIGN`"]
pub struct CALIB_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_SIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CAL_OUT`"]
pub type CAL_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL_OUT`"]
pub struct CAL_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&self) -> CALIB_VAL_R {
        CALIB_VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&self) -> CALIB_SIGN_R {
        CALIB_SIGN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&self) -> CAL_OUT_R {
        CAL_OUT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration value for absolute frequency (at a fixed temperature). Each step causes 128 ticks to be added or removed each hour. Effectively that means that each step is 1.085ppm (= 128/(60*60*32,768)). Positive values 0x01-0x3c (1..60) add pulses, negative values remove pulses, thus giving a range of +/-65.1 ppm (limited by 60 minutes per hour, not the range of this field) Calibration is performed hourly, starting at 59 minutes and 59 seconds, and applied as 64 ticks every 30 seconds until there have been 2*CALIB_VAL adjustments."]
    #[inline(always)]
    pub fn calib_val(&mut self) -> CALIB_VAL_W {
        CALIB_VAL_W { w: self }
    }
    #[doc = "Bit 6 - Calibration sign: 0= Negative sign: remove pulses (it takes more clock ticks to count one second) 1= Positive sign: add pulses (it takes less clock ticks to count one second)"]
    #[inline(always)]
    pub fn calib_sign(&mut self) -> CALIB_SIGN_W {
        CALIB_SIGN_W { w: self }
    }
    #[doc = "Bit 31 - Output enable for 512Hz signal for calibration and allow CALIB_VAL to be written. Note that calibration does not affect the 512Hz output signal."]
    #[inline(always)]
    pub fn cal_out(&mut self) -> CAL_OUT_W {
        CAL_OUT_W { w: self }
    }
}
