#[doc = "Reader of register CONN_4_PARAM_MEM_BASE_ADDR"]
pub type R = crate::R<u32, super::CONN_4_PARAM_MEM_BASE_ADDR>;
#[doc = "Writer for register CONN_4_PARAM_MEM_BASE_ADDR"]
pub type W = crate::W<u32, super::CONN_4_PARAM_MEM_BASE_ADDR>;
#[doc = "Register CONN_4_PARAM_MEM_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_4_PARAM_MEM_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_4_PARAM`"]
pub type CONN_4_PARAM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_4_PARAM`"]
pub struct CONN_4_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_4_PARAM_W<'a> {
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
    pub fn conn_4_param(&self) -> CONN_4_PARAM_R {
        CONN_4_PARAM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn conn_4_param(&mut self) -> CONN_4_PARAM_W {
        CONN_4_PARAM_W { w: self }
    }
}
