#[doc = "Reader of register B1_DATA_REG[%s]"]
pub type R = crate::R<u32, super::B1_DATA_REG>;
#[doc = "Writer for register B1_DATA_REG[%s]"]
pub type W = crate::W<u32, super::B1_DATA_REG>;
#[doc = "Register B1_DATA_REG[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::B1_DATA_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B1_DATA`"]
pub type B1_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B1_DATA`"]
pub struct B1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Programmable B1 Data register"]
    #[inline(always)]
    pub fn b1_data(&self) -> B1_DATA_R {
        B1_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable B1 Data register"]
    #[inline(always)]
    pub fn b1_data(&mut self) -> B1_DATA_W {
        B1_DATA_W { w: self }
    }
}
