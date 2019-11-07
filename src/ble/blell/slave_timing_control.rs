#[doc = "Reader of register SLAVE_TIMING_CONTROL"]
pub type R = crate::R<u32, super::SLAVE_TIMING_CONTROL>;
#[doc = "Writer for register SLAVE_TIMING_CONTROL"]
pub type W = crate::W<u32, super::SLAVE_TIMING_CONTROL>;
#[doc = "Register SLAVE_TIMING_CONTROL `reset()`'s with value 0xbe96"]
impl crate::ResetValue for super::SLAVE_TIMING_CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xbe96
    }
}
#[doc = "Reader of field `SLAVE_TIME_SET_VAL`"]
pub type SLAVE_TIME_SET_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLAVE_TIME_SET_VAL`"]
pub struct SLAVE_TIME_SET_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TIME_SET_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_TIME_ADJ_VAL`"]
pub type SLAVE_TIME_ADJ_VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLAVE_TIME_ADJ_VAL`"]
pub struct SLAVE_TIME_ADJ_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_TIME_ADJ_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Programmable adjust value to the clock counter when slave is connected"]
    #[inline(always)]
    pub fn slave_time_set_val(&self) -> SLAVE_TIME_SET_VAL_R {
        SLAVE_TIME_SET_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
    #[inline(always)]
    pub fn slave_time_adj_val(&self) -> SLAVE_TIME_ADJ_VAL_R {
        SLAVE_TIME_ADJ_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable adjust value to the clock counter when slave is connected"]
    #[inline(always)]
    pub fn slave_time_set_val(&mut self) -> SLAVE_TIME_SET_VAL_W {
        SLAVE_TIME_SET_VAL_W { w: self }
    }
    #[doc = "Bits 8:15 - Timing adjust value. The internal micro second counter is adjusted to this value whenever slave receives a good access address match at connection anchor point. This will ensure the slave gets synchronized to master timing."]
    #[inline(always)]
    pub fn slave_time_adj_val(&mut self) -> SLAVE_TIME_ADJ_VAL_W {
        SLAVE_TIME_ADJ_VAL_W { w: self }
    }
}
