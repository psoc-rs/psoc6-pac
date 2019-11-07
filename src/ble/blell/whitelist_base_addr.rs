#[doc = "Reader of register WHITELIST_BASE_ADDR"]
pub type R = crate::R<u32, super::WHITELIST_BASE_ADDR>;
#[doc = "Writer for register WHITELIST_BASE_ADDR"]
pub type W = crate::W<u32, super::WHITELIST_BASE_ADDR>;
#[doc = "Register WHITELIST_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::WHITELIST_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WL_BASE_ADDR`"]
pub type WL_BASE_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WL_BASE_ADDR`"]
pub struct WL_BASE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_BASE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device address values written to white list memory are written as 16-bit wide address."]
    #[inline(always)]
    pub fn wl_base_addr(&self) -> WL_BASE_ADDR_R {
        WL_BASE_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address values written to white list memory are written as 16-bit wide address."]
    #[inline(always)]
    pub fn wl_base_addr(&mut self) -> WL_BASE_ADDR_W {
        WL_BASE_ADDR_W { w: self }
    }
}
