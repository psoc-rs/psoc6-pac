#[doc = "Reader of register TRIM_LDO_2"]
pub type R = crate::R<u32, super::TRIM_LDO_2>;
#[doc = "Writer for register TRIM_LDO_2"]
pub type W = crate::W<u32, super::TRIM_LDO_2>;
#[doc = "Register TRIM_LDO_2 `reset()`'s with value 0x60"]
impl crate::ResetValue for super::TRIM_LDO_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x60
    }
}
#[doc = "Reader of field `SB_BMULT_RES`"]
pub type SB_BMULT_RES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SB_BMULT_RES`"]
pub struct SB_BMULT_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BMULT_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `SB_BMULT_NBIAS`"]
pub type SB_BMULT_NBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SB_BMULT_NBIAS`"]
pub struct SB_BMULT_NBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BMULT_NBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_res(&self) -> SB_BMULT_RES_R {
        SB_BMULT_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_nbias(&self) -> SB_BMULT_NBIAS_R {
        SB_BMULT_NBIAS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_res(&mut self) -> SB_BMULT_RES_W {
        SB_BMULT_RES_W { w: self }
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_nbias(&mut self) -> SB_BMULT_NBIAS_W {
        SB_BMULT_NBIAS_W { w: self }
    }
}
