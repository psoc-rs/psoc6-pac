#[doc = "Reader of register SAMPLE_TIME01"]
pub type R = crate::R<u32, super::SAMPLE_TIME01>;
#[doc = "Writer for register SAMPLE_TIME01"]
pub type W = crate::W<u32, super::SAMPLE_TIME01>;
#[doc = "Register SAMPLE_TIME01 `reset()`'s with value 0x0003_0003"]
impl crate::ResetValue for super::SAMPLE_TIME01 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0003
    }
}
#[doc = "Reader of field `SAMPLE_TIME0`"]
pub type SAMPLE_TIME0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMPLE_TIME0`"]
pub struct SAMPLE_TIME0_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SAMPLE_TIME1`"]
pub type SAMPLE_TIME1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMPLE_TIME1`"]
pub struct SAMPLE_TIME1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&self) -> SAMPLE_TIME0_R {
        SAMPLE_TIME0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&self) -> SAMPLE_TIME1_R {
        SAMPLE_TIME1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&mut self) -> SAMPLE_TIME0_W {
        SAMPLE_TIME0_W { w: self }
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&mut self) -> SAMPLE_TIME1_W {
        SAMPLE_TIME1_W { w: self }
    }
}
