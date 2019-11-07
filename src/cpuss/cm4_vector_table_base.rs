#[doc = "Reader of register CM4_VECTOR_TABLE_BASE"]
pub type R = crate::R<u32, super::CM4_VECTOR_TABLE_BASE>;
#[doc = "Writer for register CM4_VECTOR_TABLE_BASE"]
pub type W = crate::W<u32, super::CM4_VECTOR_TABLE_BASE>;
#[doc = "Register CM4_VECTOR_TABLE_BASE `reset()`'s with value 0"]
impl crate::ResetValue for super::CM4_VECTOR_TABLE_BASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR22`"]
pub type ADDR22_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR22`"]
pub struct ADDR22_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Address of CM4 vector table. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub fn addr22(&self) -> ADDR22_R {
        ADDR22_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - Address of CM4 vector table. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub fn addr22(&mut self) -> ADDR22_W {
        ADDR22_W { w: self }
    }
}
