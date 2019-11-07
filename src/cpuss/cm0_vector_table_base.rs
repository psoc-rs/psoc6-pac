#[doc = "Reader of register CM0_VECTOR_TABLE_BASE"]
pub type R = crate::R<u32, super::CM0_VECTOR_TABLE_BASE>;
#[doc = "Writer for register CM0_VECTOR_TABLE_BASE"]
pub type W = crate::W<u32, super::CM0_VECTOR_TABLE_BASE>;
#[doc = "Register CM0_VECTOR_TABLE_BASE `reset()`'s with value 0"]
impl crate::ResetValue for super::CM0_VECTOR_TABLE_BASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR24`"]
pub type ADDR24_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR24`"]
pub struct ADDR24_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&mut self) -> ADDR24_W {
        ADDR24_W { w: self }
    }
}
