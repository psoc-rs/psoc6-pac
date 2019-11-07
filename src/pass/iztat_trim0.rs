#[doc = "Reader of register IZTAT_TRIM0"]
pub type R = crate::R<u32, super::IZTAT_TRIM0>;
#[doc = "Writer for register IZTAT_TRIM0"]
pub type W = crate::W<u32, super::IZTAT_TRIM0>;
#[doc = "Register IZTAT_TRIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IZTAT_TRIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IZTAT_ABS_TRIM`"]
pub type IZTAT_ABS_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IZTAT_ABS_TRIM`"]
pub struct IZTAT_ABS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IZTAT_ABS_TRIM_W<'a> {
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
    pub fn iztat_abs_trim(&self) -> IZTAT_ABS_TRIM_R {
        IZTAT_ABS_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn iztat_abs_trim(&mut self) -> IZTAT_ABS_TRIM_W {
        IZTAT_ABS_TRIM_W { w: self }
    }
}
