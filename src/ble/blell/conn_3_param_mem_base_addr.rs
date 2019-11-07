#[doc = "Reader of register CONN_3_PARAM_MEM_BASE_ADDR"]
pub type R = crate::R<u32, super::CONN_3_PARAM_MEM_BASE_ADDR>;
#[doc = "Writer for register CONN_3_PARAM_MEM_BASE_ADDR"]
pub type W = crate::W<u32, super::CONN_3_PARAM_MEM_BASE_ADDR>;
#[doc = "Register CONN_3_PARAM_MEM_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_3_PARAM_MEM_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_3_PARAM`"]
pub type CONN_3_PARAM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_3_PARAM`"]
pub struct CONN_3_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_3_PARAM_W<'a> {
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
    pub fn conn_3_param(&self) -> CONN_3_PARAM_R {
        CONN_3_PARAM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_3_param(&mut self) -> CONN_3_PARAM_W {
        CONN_3_PARAM_W { w: self }
    }
}
