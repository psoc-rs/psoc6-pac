#[doc = "Reader of register CONN_CE_INSTANT"]
pub type R = crate::R<u32, super::CONN_CE_INSTANT>;
#[doc = "Writer for register CONN_CE_INSTANT"]
pub type W = crate::W<u32, super::CONN_CE_INSTANT>;
#[doc = "Register CONN_CE_INSTANT `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_CE_INSTANT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CE_INSTANT`"]
pub type CE_INSTANT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CE_INSTANT`"]
pub struct CE_INSTANT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_INSTANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn ce_instant(&self) -> CE_INSTANT_R {
        CE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn ce_instant(&mut self) -> CE_INSTANT_W {
        CE_INSTANT_W { w: self }
    }
}
