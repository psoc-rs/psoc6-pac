#[doc = "Reader of register VREF_TRIM2"]
pub type R = crate::R<u32, super::VREF_TRIM2>;
#[doc = "Writer for register VREF_TRIM2"]
pub type W = crate::W<u32, super::VREF_TRIM2>;
#[doc = "Register VREF_TRIM2 `reset()`'s with value 0"]
impl crate::ResetValue for super::VREF_TRIM2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREF_CURV_TRIM`"]
pub type VREF_CURV_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_CURV_TRIM`"]
pub struct VREF_CURV_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_CURV_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&self) -> VREF_CURV_TRIM_R {
        VREF_CURV_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&mut self) -> VREF_CURV_TRIM_W {
        VREF_CURV_TRIM_W { w: self }
    }
}
