#[doc = "Reader of register TRIM_LDO_5"]
pub type R = crate::R<u32, super::TRIM_LDO_5>;
#[doc = "Writer for register TRIM_LDO_5"]
pub type W = crate::W<u32, super::TRIM_LDO_5>;
#[doc = "Register TRIM_LDO_5 `reset()`'s with value 0"]
impl crate::ResetValue for super::TRIM_LDO_5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD`"]
pub type RSVD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD`"]
pub struct RSVD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_W<'a> {
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
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RSVD_W {
        RSVD_W { w: self }
    }
}
