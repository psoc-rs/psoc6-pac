#[doc = "Reader of register LE_PING_TIMER_ADDR"]
pub type R = crate::R<u32, super::LE_PING_TIMER_ADDR>;
#[doc = "Writer for register LE_PING_TIMER_ADDR"]
pub type W = crate::W<u32, super::LE_PING_TIMER_ADDR>;
#[doc = "Register LE_PING_TIMER_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::LE_PING_TIMER_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_PING_TIMER_ADDR`"]
pub type CONN_PING_TIMER_ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_PING_TIMER_ADDR`"]
pub struct CONN_PING_TIMER_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_PING_TIMER_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
    #[inline(always)]
    pub fn conn_ping_timer_addr(&self) -> CONN_PING_TIMER_ADDR_R {
        CONN_PING_TIMER_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The register used to configure the LE Au-thenticated payload Timeout (LE APTO) which is the Maximum amount of time specified between packets authenticated by a MIC. This value of ping timer is in the order of 10ms, valid range 0x1 ~ 0xFFFF"]
    #[inline(always)]
    pub fn conn_ping_timer_addr(&mut self) -> CONN_PING_TIMER_ADDR_W {
        CONN_PING_TIMER_ADDR_W { w: self }
    }
}
