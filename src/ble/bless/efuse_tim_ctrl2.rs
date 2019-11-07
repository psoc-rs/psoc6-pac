#[doc = "Reader of register EFUSE_TIM_CTRL2"]
pub type R = crate::R<u32, super::EFUSE_TIM_CTRL2>;
#[doc = "Writer for register EFUSE_TIM_CTRL2"]
pub type W = crate::W<u32, super::EFUSE_TIM_CTRL2>;
#[doc = "Register EFUSE_TIM_CTRL2 `reset()`'s with value 0x0102"]
impl crate::ResetValue for super::EFUSE_TIM_CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0102
    }
}
#[doc = "Reader of field `DATA_SAMPLE_TIME`"]
pub type DATA_SAMPLE_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_SAMPLE_TIME`"]
pub struct DATA_SAMPLE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SAMPLE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DOUT_CS_HOLD_TIME`"]
pub type DOUT_CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOUT_CS_HOLD_TIME`"]
pub struct DOUT_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
    #[inline(always)]
    pub fn data_sample_time(&self) -> DATA_SAMPLE_TIME_R {
        DATA_SAMPLE_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Wait time DOUT to CS hold time out of read mode (TDQH)"]
    #[inline(always)]
    pub fn dout_cs_hold_time(&self) -> DOUT_CS_HOLD_TIME_R {
        DOUT_CS_HOLD_TIME_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time for data sampling from SCLK HIGH (TCKDQ_H)"]
    #[inline(always)]
    pub fn data_sample_time(&mut self) -> DATA_SAMPLE_TIME_W {
        DATA_SAMPLE_TIME_W { w: self }
    }
    #[doc = "Bits 8:11 - Wait time DOUT to CS hold time out of read mode (TDQH)"]
    #[inline(always)]
    pub fn dout_cs_hold_time(&mut self) -> DOUT_CS_HOLD_TIME_W {
        DOUT_CS_HOLD_TIME_W { w: self }
    }
}
