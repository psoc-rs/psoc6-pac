#[doc = "Reader of register CONN_UPDATE_NEW_LATENCY"]
pub type R = crate::R<u32, super::CONN_UPDATE_NEW_LATENCY>;
#[doc = "Writer for register CONN_UPDATE_NEW_LATENCY"]
pub type W = crate::W<u32, super::CONN_UPDATE_NEW_LATENCY>;
#[doc = "Register CONN_UPDATE_NEW_LATENCY `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_UPDATE_NEW_LATENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_UPDT_SLV_LATENCY`"]
pub type CONN_UPDT_SLV_LATENCY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_UPDT_SLV_LATENCY`"]
pub struct CONN_UPDT_SLV_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_UPDT_SLV_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_slv_latency(&self) -> CONN_UPDT_SLV_LATENCY_R {
        CONN_UPDT_SLV_LATENCY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register will have the new slave latency parameter that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SLAVE_LATENCY will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_slv_latency(&mut self) -> CONN_UPDT_SLV_LATENCY_W {
        CONN_UPDT_SLV_LATENCY_W { w: self }
    }
}
