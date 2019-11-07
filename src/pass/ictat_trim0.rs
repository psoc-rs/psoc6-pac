#[doc = "Reader of register ICTAT_TRIM0"]
pub type R = crate::R<u32, super::ICTAT_TRIM0>;
#[doc = "Writer for register ICTAT_TRIM0"]
pub type W = crate::W<u32, super::ICTAT_TRIM0>;
#[doc = "Register ICTAT_TRIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICTAT_TRIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICTAT_TRIM`"]
pub type ICTAT_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICTAT_TRIM`"]
pub struct ICTAT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICTAT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&self) -> ICTAT_TRIM_R {
        ICTAT_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&mut self) -> ICTAT_TRIM_W {
        ICTAT_TRIM_W { w: self }
    }
}
