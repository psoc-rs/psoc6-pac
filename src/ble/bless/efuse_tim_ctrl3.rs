#[doc = "Reader of register EFUSE_TIM_CTRL3"]
pub type R = crate::R<u32, super::EFUSE_TIM_CTRL3>;
#[doc = "Writer for register EFUSE_TIM_CTRL3"]
pub type W = crate::W<u32, super::EFUSE_TIM_CTRL3>;
#[doc = "Register EFUSE_TIM_CTRL3 `reset()`'s with value 0x003a_3a11"]
impl crate::ResetValue for super::EFUSE_TIM_CTRL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x003a_3a11
    }
}
#[doc = "Reader of field `PGM_SCLK_SETUP_TIME`"]
pub type PGM_SCLK_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_SCLK_SETUP_TIME`"]
pub struct PGM_SCLK_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SCLK_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PGM_SCLK_HOLD_TIME`"]
pub type PGM_SCLK_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_SCLK_HOLD_TIME`"]
pub struct PGM_SCLK_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SCLK_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `AVDD_CS_SETUP_TIME`"]
pub type AVDD_CS_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AVDD_CS_SETUP_TIME`"]
pub struct AVDD_CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDD_CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AVDD_CS_HOLD_TIME`"]
pub type AVDD_CS_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AVDD_CS_HOLD_TIME`"]
pub struct AVDD_CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> AVDD_CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
    #[inline(always)]
    pub fn pgm_sclk_setup_time(&self) -> PGM_SCLK_SETUP_TIME_R {
        PGM_SCLK_SETUP_TIME_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PGM to SCLK hold time (TH_PGM)"]
    #[inline(always)]
    pub fn pgm_sclk_hold_time(&self) -> PGM_SCLK_HOLD_TIME_R {
        PGM_SCLK_HOLD_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_setup_time(&self) -> AVDD_CS_SETUP_TIME_R {
        AVDD_CS_SETUP_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_hold_time(&self) -> AVDD_CS_HOLD_TIME_R {
        AVDD_CS_HOLD_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PGM to SCLK setup time (TS_PGM) PGM_SCLK_SETUP_TIME <CS_SCLK_SETUP_TIME"]
    #[inline(always)]
    pub fn pgm_sclk_setup_time(&mut self) -> PGM_SCLK_SETUP_TIME_W {
        PGM_SCLK_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 4:7 - PGM to SCLK hold time (TH_PGM)"]
    #[inline(always)]
    pub fn pgm_sclk_hold_time(&mut self) -> PGM_SCLK_HOLD_TIME_W {
        PGM_SCLK_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 8:15 - AVDD to CS setup time into program mode (TSP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_setup_time(&mut self) -> AVDD_CS_SETUP_TIME_W {
        AVDD_CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bits 16:23 - AVDD to CS hold time out of program mode (THP_AVDD_CS)"]
    #[inline(always)]
    pub fn avdd_cs_hold_time(&mut self) -> AVDD_CS_HOLD_TIME_W {
        AVDD_CS_HOLD_TIME_W { w: self }
    }
}
