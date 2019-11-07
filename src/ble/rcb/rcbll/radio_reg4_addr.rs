#[doc = "Reader of register RADIO_REG4_ADDR"]
pub type R = crate::R<u32, super::RADIO_REG4_ADDR>;
#[doc = "Writer for register RADIO_REG4_ADDR"]
pub type W = crate::W<u32, super::RADIO_REG4_ADDR>;
#[doc = "Register RADIO_REG4_ADDR `reset()`'s with value 0x0823"]
impl crate::ResetValue for super::RADIO_REG4_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0823
    }
}
#[doc = "Reader of field `REG_ADDR`"]
pub type REG_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REG_ADDR`"]
pub struct REG_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn reg_addr(&self) -> REG_ADDR_R {
        REG_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn reg_addr(&mut self) -> REG_ADDR_W {
        REG_ADDR_W { w: self }
    }
}
