#[doc = "Reader of register ARB_RW7_DR16"]
pub type R = crate::R<u32, super::ARB_RW7_DR16>;
#[doc = "Writer for register ARB_RW7_DR16"]
pub type W = crate::W<u32, super::ARB_RW7_DR16>;
#[doc = "Register ARB_RW7_DR16 `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_RW7_DR16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR16`"]
pub type DR16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DR16`"]
pub struct DR16_W<'a> {
    w: &'a mut W,
}
impl<'a> DR16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&mut self) -> DR16_W {
        DR16_W { w: self }
    }
}
