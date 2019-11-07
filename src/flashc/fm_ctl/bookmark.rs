#[doc = "Writer for register BOOKMARK"]
pub type W = crate::W<u32, super::BOOKMARK>;
#[doc = "Register BOOKMARK `reset()`'s with value 0"]
impl crate::ResetValue for super::BOOKMARK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BOOKMARK`"]
pub struct BOOKMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOKMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&mut self) -> BOOKMARK_W {
        BOOKMARK_W { w: self }
    }
}
