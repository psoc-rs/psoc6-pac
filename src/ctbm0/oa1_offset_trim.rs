#[doc = "Reader of register OA1_OFFSET_TRIM"]
pub type R = crate::R<u32, super::OA1_OFFSET_TRIM>;
#[doc = "Writer for register OA1_OFFSET_TRIM"]
pub type W = crate::W<u32, super::OA1_OFFSET_TRIM>;
#[doc = "Register OA1_OFFSET_TRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::OA1_OFFSET_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1_OFFSET_TRIM`"]
pub type OA1_OFFSET_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA1_OFFSET_TRIM`"]
pub struct OA1_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Opamp1 offset trim"]
    #[inline(always)]
    pub fn oa1_offset_trim(&self) -> OA1_OFFSET_TRIM_R {
        OA1_OFFSET_TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Opamp1 offset trim"]
    #[inline(always)]
    pub fn oa1_offset_trim(&mut self) -> OA1_OFFSET_TRIM_W {
        OA1_OFFSET_TRIM_W { w: self }
    }
}
