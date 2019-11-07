#[doc = "Reader of register ADV_ACCADDR_L"]
pub type R = crate::R<u32, super::ADV_ACCADDR_L>;
#[doc = "Writer for register ADV_ACCADDR_L"]
pub type W = crate::W<u32, super::ADV_ACCADDR_L>;
#[doc = "Register ADV_ACCADDR_L `reset()`'s with value 0xbed6"]
impl crate::ResetValue for super::ADV_ACCADDR_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xbed6
    }
}
#[doc = "Reader of field `ADV_ACCADDR_L`"]
pub type ADV_ACCADDR_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADV_ACCADDR_L`"]
pub struct ADV_ACCADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_ACCADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_l(&self) -> ADV_ACCADDR_L_R {
        ADV_ACCADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of ADV packet access code"]
    #[inline(always)]
    pub fn adv_accaddr_l(&mut self) -> ADV_ACCADDR_L_W {
        ADV_ACCADDR_L_W { w: self }
    }
}
