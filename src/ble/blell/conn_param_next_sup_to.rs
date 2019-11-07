#[doc = "Reader of register CONN_PARAM_NEXT_SUP_TO"]
pub type R = crate::R<u32, super::CONN_PARAM_NEXT_SUP_TO>;
#[doc = "Writer for register CONN_PARAM_NEXT_SUP_TO"]
pub type W = crate::W<u32, super::CONN_PARAM_NEXT_SUP_TO>;
#[doc = "Register CONN_PARAM_NEXT_SUP_TO `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_PARAM_NEXT_SUP_TO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NEXT_SUP_TO_LOAD`"]
pub type NEXT_SUP_TO_LOAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NEXT_SUP_TO_LOAD`"]
pub struct NEXT_SUP_TO_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_SUP_TO_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn next_sup_to_load(&self) -> NEXT_SUP_TO_LOAD_R {
        NEXT_SUP_TO_LOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HW uses this register to load the Supervision timeout Next instant from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn next_sup_to_load(&mut self) -> NEXT_SUP_TO_LOAD_W {
        NEXT_SUP_TO_LOAD_W { w: self }
    }
}
