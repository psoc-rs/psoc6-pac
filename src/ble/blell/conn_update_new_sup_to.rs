#[doc = "Reader of register CONN_UPDATE_NEW_SUP_TO"]
pub type R = crate::R<u32, super::CONN_UPDATE_NEW_SUP_TO>;
#[doc = "Writer for register CONN_UPDATE_NEW_SUP_TO"]
pub type W = crate::W<u32, super::CONN_UPDATE_NEW_SUP_TO>;
#[doc = "Register CONN_UPDATE_NEW_SUP_TO `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_UPDATE_NEW_SUP_TO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_UPDT_SUP_TO`"]
pub type CONN_UPDT_SUP_TO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_UPDT_SUP_TO`"]
pub struct CONN_UPDT_SUP_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_UPDT_SUP_TO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_sup_to(&self) -> CONN_UPDT_SUP_TO_R {
        CONN_UPDT_SUP_TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_sup_to(&mut self) -> CONN_UPDT_SUP_TO_W {
        CONN_UPDT_SUP_TO_W { w: self }
    }
}
