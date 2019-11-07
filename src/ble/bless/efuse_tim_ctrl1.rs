#[doc = "Reader of register EFUSE_TIM_CTRL1"]
pub type R = crate::R<u32, super::EFUSE_TIM_CTRL1>;
#[doc = "Writer for register EFUSE_TIM_CTRL1"]
pub type W = crate::W<u32, super::EFUSE_TIM_CTRL1>;
#[doc = "Register EFUSE_TIM_CTRL1 `reset()`'s with value 0x1112_01c0"]
impl crate::ResetValue for super::EFUSE_TIM_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1112_01c0
    }
}
#[doc = "Reader of field `SCLK_HIGH`"]
pub type SCLK_HIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_HIGH`"]
pub struct SCLK_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SCLK_LOW`"]
pub type SCLK_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_LOW`"]
pub struct SCLK_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CS_SCLK_SETUP_TIME`"]
pub type CS_SCLK_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS_SCLK_SETUP_TIME`"]
pub struct CS_SCLK_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SCLK_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CS_SCLK_HOLD_TIME`"]
pub type CS_SCLK_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS_SCLK_HOLD_TIME`"]
pub struct CS_SCLK_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SCLK_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RW_CS_SETUP_TIME`"]
pub type RW_CS_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RW_CS_SETUP_TIME`"]
pub struct RW_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RW_CS_HOLD_TIME`"]
pub type RW_CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RW_CS_HOLD_TIME`"]
pub struct RW_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
    #[inline(always)]
    pub fn sclk_high(&self) -> SCLK_HIGH_R {
        SCLK_HIGH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
    #[inline(always)]
    pub fn sclk_low(&self) -> SCLK_LOW_R {
        SCLK_LOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_setup_time(&self) -> CS_SCLK_SETUP_TIME_R {
        CS_SCLK_SETUP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - This register specifies the hold time between CS and SCLK (THR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_hold_time(&self) -> CS_SCLK_HOLD_TIME_R {
        CS_SCLK_HOLD_TIME_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
    #[inline(always)]
    pub fn rw_cs_setup_time(&self) -> RW_CS_SETUP_TIME_R {
        RW_CS_SETUP_TIME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
    #[inline(always)]
    pub fn rw_cs_hold_time(&self) -> RW_CS_HOLD_TIME_R {
        RW_CS_HOLD_TIME_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decides the duration of TPGM (in Program mode) or TCKHP (in Read mode) TPGM: Burning Time TCKHP : SCLK high Period"]
    #[inline(always)]
    pub fn sclk_high(&mut self) -> SCLK_HIGH_W {
        SCLK_HIGH_W { w: self }
    }
    #[doc = "Bits 8:15 - Duration of SCLK LOW (TCLKP_R) or TCKLP_P"]
    #[inline(always)]
    pub fn sclk_low(&mut self) -> SCLK_LOW_W {
        SCLK_LOW_W { w: self }
    }
    #[doc = "Bits 16:19 - This register specifies the setup time between CS and SCLK (TSR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_setup_time(&mut self) -> CS_SCLK_SETUP_TIME_W {
        CS_SCLK_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 20:23 - This register specifies the hold time between CS and SCLK (THR_CLK)"]
    #[inline(always)]
    pub fn cs_sclk_hold_time(&mut self) -> CS_SCLK_HOLD_TIME_W {
        CS_SCLK_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 24:27 - This field decides setup time between RW & CS (TSR_RW: in read mode) or RW & AVDD (TSP_RW: in Program mode). TSR_RW: RW to CS setup time into Read mode TSP_RW: RW to AVDD setup time into program mode"]
    #[inline(always)]
    pub fn rw_cs_setup_time(&mut self) -> RW_CS_SETUP_TIME_W {
        RW_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 28:31 - This field decides hold time between RW & CS (THR_RW: in read mode) or RW & AVDD (THP_RW: in Program mode). THR_RW: RW to CS hold time out of Read mode THP_RW: RW to AVDD hold time out of program mode"]
    #[inline(always)]
    pub fn rw_cs_hold_time(&mut self) -> RW_CS_HOLD_TIME_W {
        RW_CS_HOLD_TIME_W { w: self }
    }
}
