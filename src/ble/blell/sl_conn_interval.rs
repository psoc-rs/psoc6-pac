#[doc = "Reader of register SL_CONN_INTERVAL"]
pub type R = crate::R<u32, super::SL_CONN_INTERVAL>;
#[doc = "Writer for register SL_CONN_INTERVAL"]
pub type W = crate::W<u32, super::SL_CONN_INTERVAL>;
#[doc = "Register SL_CONN_INTERVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::SL_CONN_INTERVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SL_CONN_INTERVAL_VAL`"]
pub type SL_CONN_INTERVAL_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SL_CONN_INTERVAL_VAL`"]
pub struct SL_CONN_INTERVAL_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_CONN_INTERVAL_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the (SL*CI) product for the ongoing connection. This value is used in calculation of next connection instant during slave latency."]
    #[inline(always)]
    pub fn sl_conn_interval_val(&self) -> SL_CONN_INTERVAL_VAL_R {
        SL_CONN_INTERVAL_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the (SL*CI) product for the ongoing connection. This value is used in calculation of next connection instant during slave latency."]
    #[inline(always)]
    pub fn sl_conn_interval_val(&mut self) -> SL_CONN_INTERVAL_VAL_W {
        SL_CONN_INTERVAL_VAL_W { w: self }
    }
}
