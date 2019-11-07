#[doc = "Reader of register BREG[%s]"]
pub type R = crate::R<u32, super::BREG>;
#[doc = "Writer for register BREG[%s]"]
pub type W = crate::W<u32, super::BREG>;
#[doc = "Register BREG[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::BREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BREG`"]
pub type BREG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BREG`"]
pub struct BREG_W<'a> {
    w: &'a mut W,
}
impl<'a> BREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub fn breg(&self) -> BREG_R {
        BREG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Backup memory that contains application-specific data. Memory is retained on vbackup supply."]
    #[inline(always)]
    pub fn breg(&mut self) -> BREG_W {
        BREG_W { w: self }
    }
}
