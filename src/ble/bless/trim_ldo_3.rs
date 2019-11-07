#[doc = "Reader of register TRIM_LDO_3"]
pub type R = crate::R<u32, super::TRIM_LDO_3>;
#[doc = "Writer for register TRIM_LDO_3"]
pub type W = crate::W<u32, super::TRIM_LDO_3>;
#[doc = "Register TRIM_LDO_3 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::TRIM_LDO_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `LVDET`"]
pub type LVDET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVDET`"]
pub struct LVDET_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `SLOPE_SB_BMULT`"]
pub type SLOPE_SB_BMULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOPE_SB_BMULT`"]
pub struct SLOPE_SB_BMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOPE_SB_BMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - To trim the trip points of the LV-Detect block"]
    #[inline(always)]
    pub fn lvdet(&self) -> LVDET_R {
        LVDET_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier temp-co slope"]
    #[inline(always)]
    pub fn slope_sb_bmult(&self) -> SLOPE_SB_BMULT_R {
        SLOPE_SB_BMULT_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - To trim the trip points of the LV-Detect block"]
    #[inline(always)]
    pub fn lvdet(&mut self) -> LVDET_W {
        LVDET_W { w: self }
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier temp-co slope"]
    #[inline(always)]
    pub fn slope_sb_bmult(&mut self) -> SLOPE_SB_BMULT_W {
        SLOPE_SB_BMULT_W { w: self }
    }
}
