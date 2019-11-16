#[doc = "Reader of register ADC_CTL"]
pub type R = crate::R<u32, super::ADC_CTL>;
#[doc = "Writer for register ADC_CTL"]
pub type W = crate::W<u32, super::ADC_CTL>;
#[doc = "Register ADC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADC_TIME`"]
pub type ADC_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_TIME`"]
pub struct ADC_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_MODE_A {
    #[doc = "0: No ADC measurement"]
    OFF = 0,
    #[doc = "1: Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VREF_CNT = 1,
    #[doc = "2: Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VREF_BY2_CNT = 2,
    #[doc = "3: Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VIN_CNT = 3,
}
impl From<ADC_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_MODE`"]
pub type ADC_MODE_R = crate::R<u8, ADC_MODE_A>;
impl ADC_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_MODE_A {
        match self.bits {
            0 => ADC_MODE_A::OFF,
            1 => ADC_MODE_A::VREF_CNT,
            2 => ADC_MODE_A::VREF_BY2_CNT,
            3 => ADC_MODE_A::VIN_CNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ADC_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `VREF_CNT`"]
    #[inline(always)]
    pub fn is_vref_cnt(&self) -> bool {
        *self == ADC_MODE_A::VREF_CNT
    }
    #[doc = "Checks if the value of the field is `VREF_BY2_CNT`"]
    #[inline(always)]
    pub fn is_vref_by2_cnt(&self) -> bool {
        *self == ADC_MODE_A::VREF_BY2_CNT
    }
    #[doc = "Checks if the value of the field is `VIN_CNT`"]
    #[inline(always)]
    pub fn is_vin_cnt(&self) -> bool {
        *self == ADC_MODE_A::VIN_CNT
    }
}
#[doc = "Write proxy for field `ADC_MODE`"]
pub struct ADC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ADC_MODE_A::OFF)
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn vref_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VREF_CNT)
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn vref_by2_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VREF_BY2_CNT)
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(self) -> &'a mut W {
        self.variant(ADC_MODE_A::VIN_CNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn adc_time(&self) -> ADC_TIME_R {
        ADC_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&self) -> ADC_MODE_R {
        ADC_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&2, or as the aperture to capture the input voltage on Cref1&2"]
    #[inline(always)]
    pub fn adc_time(&mut self) -> ADC_TIME_W {
        ADC_TIME_W { w: self }
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&mut self) -> ADC_MODE_W {
        ADC_MODE_W { w: self }
    }
}
