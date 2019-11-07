#[doc = "Reader of register ADV_ACCADDR_H"]
pub type R = crate::R<u32, super::ADV_ACCADDR_H>;
#[doc = "Writer for register ADV_ACCADDR_H"]
pub type W = crate::W<u32, super::ADV_ACCADDR_H>;
#[doc = "Register ADV_ACCADDR_H `reset()`'s with value 0x8e89"]
impl crate::ResetValue for super::ADV_ACCADDR_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8e89
    }
}
#[doc = "Reader of field `ADV_ACCADDR_H`"]
pub type ADV_ACCADDR_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADV_ACCADDR_H`"]
pub struct ADV_ACCADDR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_ACCADDR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - higher 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_h(&self) -> ADV_ACCADDR_H_R {
        ADV_ACCADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - higher 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_h(&mut self) -> ADV_ACCADDR_H_W {
        ADV_ACCADDR_H_W { w: self }
    }
}
