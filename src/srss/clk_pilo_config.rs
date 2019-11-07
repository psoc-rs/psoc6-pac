#[doc = "Reader of register CLK_PILO_CONFIG"]
pub type R = crate::R<u32, super::CLK_PILO_CONFIG>;
#[doc = "Writer for register CLK_PILO_CONFIG"]
pub type W = crate::W<u32, super::CLK_PILO_CONFIG>;
#[doc = "Register CLK_PILO_CONFIG `reset()`'s with value 0x80"]
impl crate::ResetValue for super::CLK_PILO_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `PILO_FFREQ`"]
pub type PILO_FFREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PILO_FFREQ`"]
pub struct PILO_FFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_FFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `PILO_CLK_EN`"]
pub type PILO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PILO_CLK_EN`"]
pub struct PILO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PILO_RESET_N`"]
pub type PILO_RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PILO_RESET_N`"]
pub struct PILO_RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PILO_EN`"]
pub type PILO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PILO_EN`"]
pub struct PILO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_EN_W<'a> {
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
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&self) -> PILO_FFREQ_R {
        PILO_FFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&self) -> PILO_CLK_EN_R {
        PILO_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&self) -> PILO_RESET_N_R {
        PILO_RESET_N_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&self) -> PILO_EN_R {
        PILO_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&mut self) -> PILO_FFREQ_W {
        PILO_FFREQ_W { w: self }
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&mut self) -> PILO_CLK_EN_W {
        PILO_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&mut self) -> PILO_RESET_N_W {
        PILO_RESET_N_W { w: self }
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&mut self) -> PILO_EN_W {
        PILO_EN_W { w: self }
    }
}
