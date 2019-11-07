#[doc = "Reader of register PWR_TRIM_WAKE_CTL"]
pub type R = crate::R<u32, super::PWR_TRIM_WAKE_CTL>;
#[doc = "Writer for register PWR_TRIM_WAKE_CTL"]
pub type W = crate::W<u32, super::PWR_TRIM_WAKE_CTL>;
#[doc = "Register PWR_TRIM_WAKE_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_TRIM_WAKE_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WAKE_DELAY`"]
pub type WAKE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAKE_DELAY`"]
pub struct WAKE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieve with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&self) -> WAKE_DELAY_R {
        WAKE_DELAY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieve with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&mut self) -> WAKE_DELAY_W {
        WAKE_DELAY_W { w: self }
    }
}
