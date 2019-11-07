#[doc = "Reader of register OA1_COMP_TRIM"]
pub type R = crate::R<u32, super::OA1_COMP_TRIM>;
#[doc = "Writer for register OA1_COMP_TRIM"]
pub type W = crate::W<u32, super::OA1_COMP_TRIM>;
#[doc = "Register OA1_COMP_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::OA1_COMP_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1_COMP_TRIM`"]
pub type OA1_COMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA1_COMP_TRIM`"]
pub struct OA1_COMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_COMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Opamp1 Compensation Capacitor Trim. Value depends on the drive strength setting - 1x mode: set to 01; 10x mode: set to 11"]
    #[inline(always)]
    pub fn oa1_comp_trim(&self) -> OA1_COMP_TRIM_R {
        OA1_COMP_TRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Opamp1 Compensation Capacitor Trim. Value depends on the drive strength setting - 1x mode: set to 01; 10x mode: set to 11"]
    #[inline(always)]
    pub fn oa1_comp_trim(&mut self) -> OA1_COMP_TRIM_W {
        OA1_COMP_TRIM_W { w: self }
    }
}
