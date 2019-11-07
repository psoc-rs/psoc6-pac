#[doc = "Reader of register HW_LOAD_OFFSET"]
pub type R = crate::R<u32, super::HW_LOAD_OFFSET>;
#[doc = "Writer for register HW_LOAD_OFFSET"]
pub type W = crate::W<u32, super::HW_LOAD_OFFSET>;
#[doc = "Register HW_LOAD_OFFSET `reset()`'s with value 0x04"]
impl crate::ResetValue for super::HW_LOAD_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `LOAD_OFFSET`"]
pub type LOAD_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOAD_OFFSET`"]
pub struct LOAD_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
    #[inline(always)]
    pub fn load_offset(&self) -> LOAD_OFFSET_R {
        LOAD_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Load Offset in us before connection event at which the connection parameters are loaded from memory, granularity is in 1us"]
    #[inline(always)]
    pub fn load_offset(&mut self) -> LOAD_OFFSET_W {
        LOAD_OFFSET_W { w: self }
    }
}
