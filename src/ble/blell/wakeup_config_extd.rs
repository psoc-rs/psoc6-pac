#[doc = "Reader of register WAKEUP_CONFIG_EXTD"]
pub type R = crate::R<u32, super::WAKEUP_CONFIG_EXTD>;
#[doc = "Writer for register WAKEUP_CONFIG_EXTD"]
pub type W = crate::W<u32, super::WAKEUP_CONFIG_EXTD>;
#[doc = "Register WAKEUP_CONFIG_EXTD `reset()`'s with value 0"]
impl crate::ResetValue for super::WAKEUP_CONFIG_EXTD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSM_LF_OFFSET`"]
pub type DSM_LF_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSM_LF_OFFSET`"]
pub struct DSM_LF_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_LF_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
    #[inline(always)]
    pub fn dsm_lf_offset(&self) -> DSM_LF_OFFSET_R {
        DSM_LF_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 'LF slots' before the wake up instant before which the hardware needs to exit from deep sleep mode. The LF slot is of 62.5us period. This is a onetime configuration field, which is used every time hardware does an auto-wakeup before the next wakeup instant. This is in addition to the LF slots calculated by HW window widening logic."]
    #[inline(always)]
    pub fn dsm_lf_offset(&mut self) -> DSM_LF_OFFSET_W {
        DSM_LF_OFFSET_W { w: self }
    }
}
