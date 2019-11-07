#[doc = "Reader of register RAM_PWR_DELAY_CTL"]
pub type R = crate::R<u32, super::RAM_PWR_DELAY_CTL>;
#[doc = "Writer for register RAM_PWR_DELAY_CTL"]
pub type W = crate::W<u32, super::RAM_PWR_DELAY_CTL>;
#[doc = "Register RAM_PWR_DELAY_CTL `reset()`'s with value 0x96"]
impl crate::ResetValue for super::RAM_PWR_DELAY_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x96
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UP`"]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
}
