#[doc = "Reader of register SUP_TIMEOUT"]
pub type R = crate::R<u32, super::SUP_TIMEOUT>;
#[doc = "Writer for register SUP_TIMEOUT"]
pub type W = crate::W<u32, super::SUP_TIMEOUT>;
#[doc = "Register SUP_TIMEOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::SUP_TIMEOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUPERVISION_TIMEOUT`"]
pub type SUPERVISION_TIMEOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUPERVISION_TIMEOUT`"]
pub struct SUPERVISION_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPERVISION_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout(&self) -> SUPERVISION_TIMEOUT_R {
        SUPERVISION_TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the maximum time between two received Data packet PDUs before the connection is considered lost. This shall be a multiple of 10 ms in the range of 100 ms to 32.0 s and it shall be larger than (1+connSlaveLatency)*connInterval."]
    #[inline(always)]
    pub fn supervision_timeout(&mut self) -> SUPERVISION_TIMEOUT_W {
        SUPERVISION_TIMEOUT_W { w: self }
    }
}
