#[doc = "Reader of register US_CAPT_PREV"]
pub type R = crate::R<u32, super::US_CAPT_PREV>;
#[doc = "Writer for register US_CAPT_PREV"]
pub type W = crate::W<u32, super::US_CAPT_PREV>;
#[doc = "Register US_CAPT_PREV `reset()`'s with value 0"]
impl crate::ResetValue for super::US_CAPT_PREV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `US_CAPT_LOAD`"]
pub type US_CAPT_LOAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `US_CAPT_LOAD`"]
pub struct US_CAPT_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> US_CAPT_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
    #[inline(always)]
    pub fn us_capt_load(&self) -> US_CAPT_LOAD_R {
        US_CAPT_LOAD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HW uses this register to load the us_offset from connection parameter memory. This can be used by firmware as a fail safe option if the HW load from memory is disabled. In alll other conditions firmware should not use this register."]
    #[inline(always)]
    pub fn us_capt_load(&mut self) -> US_CAPT_LOAD_W {
        US_CAPT_LOAD_W { w: self }
    }
}
