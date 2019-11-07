#[doc = "Reader of register INIT_NI_VAL"]
pub type R = crate::R<u32, super::INIT_NI_VAL>;
#[doc = "Writer for register INIT_NI_VAL"]
pub type W = crate::W<u32, super::INIT_NI_VAL>;
#[doc = "Register INIT_NI_VAL `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_NI_VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_NI_VAL`"]
pub type INIT_NI_VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INIT_NI_VAL`"]
pub struct INIT_NI_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_NI_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
    #[inline(always)]
    pub fn init_ni_val(&self) -> INIT_NI_VAL_R {
        INIT_NI_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initiator window Next Instant value used for spacing Master connections in time, to minimize connection contention. This value is in 625us slots. The read value corresponds to the hardware updated Interval value"]
    #[inline(always)]
    pub fn init_ni_val(&mut self) -> INIT_NI_VAL_W {
        INIT_NI_VAL_W { w: self }
    }
}
