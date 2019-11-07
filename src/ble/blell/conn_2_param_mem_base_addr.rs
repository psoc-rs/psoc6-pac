#[doc = "Reader of register CONN_2_PARAM_MEM_BASE_ADDR"]
pub type R = crate::R<u32, super::CONN_2_PARAM_MEM_BASE_ADDR>;
#[doc = "Writer for register CONN_2_PARAM_MEM_BASE_ADDR"]
pub type W = crate::W<u32, super::CONN_2_PARAM_MEM_BASE_ADDR>;
#[doc = "Register CONN_2_PARAM_MEM_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_2_PARAM_MEM_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_2_PARAM`"]
pub type CONN_2_PARAM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_2_PARAM`"]
pub struct CONN_2_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_2_PARAM_W<'a> {
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
    pub fn conn_2_param(&self) -> CONN_2_PARAM_R {
        CONN_2_PARAM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_2_param(&mut self) -> CONN_2_PARAM_W {
        CONN_2_PARAM_W { w: self }
    }
}
