#[doc = "Reader of register CHAN_EN"]
pub type R = crate::R<u32, super::CHAN_EN>;
#[doc = "Writer for register CHAN_EN"]
pub type W = crate::W<u32, super::CHAN_EN>;
#[doc = "Register CHAN_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::CHAN_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHAN_EN`"]
pub type CHAN_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHAN_EN`"]
pub struct CHAN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&self) -> CHAN_EN_R {
        CHAN_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&mut self) -> CHAN_EN_W {
        CHAN_EN_W { w: self }
    }
}
