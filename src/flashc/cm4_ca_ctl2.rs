#[doc = "Reader of register CM4_CA_CTL2"]
pub type R = crate::R<u32, super::CM4_CA_CTL2>;
#[doc = "Writer for register CM4_CA_CTL2"]
pub type W = crate::W<u32, super::CM4_CA_CTL2>;
#[doc = "Register CM4_CA_CTL2 `reset()`'s with value 0x012c"]
impl crate::ResetValue for super::CM4_CA_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x012c
    }
}
#[doc = "Reader of field `PWRUP_DELAY`"]
pub type PWRUP_DELAY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PWRUP_DELAY`"]
pub struct PWRUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_DELAY_W<'a> {
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
    pub fn pwrup_delay(&self) -> PWRUP_DELAY_R {
        PWRUP_DELAY_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn pwrup_delay(&mut self) -> PWRUP_DELAY_W {
        PWRUP_DELAY_W { w: self }
    }
}
