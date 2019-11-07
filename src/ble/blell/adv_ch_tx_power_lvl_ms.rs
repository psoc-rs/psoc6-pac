#[doc = "Reader of register ADV_CH_TX_POWER_LVL_MS"]
pub type R = crate::R<u32, super::ADV_CH_TX_POWER_LVL_MS>;
#[doc = "Writer for register ADV_CH_TX_POWER_LVL_MS"]
pub type W = crate::W<u32, super::ADV_CH_TX_POWER_LVL_MS>;
#[doc = "Register ADV_CH_TX_POWER_LVL_MS `reset()`'s with value 0"]
impl crate::ResetValue for super::ADV_CH_TX_POWER_LVL_MS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_TRANSMIT_POWER_LVL_MS`"]
pub type ADV_TRANSMIT_POWER_LVL_MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADV_TRANSMIT_POWER_LVL_MS`"]
pub struct ADV_TRANSMIT_POWER_LVL_MS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TRANSMIT_POWER_LVL_MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Advertising channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ms(&self) -> ADV_TRANSMIT_POWER_LVL_MS_R {
        ADV_TRANSMIT_POWER_LVL_MS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Advertising channel transmit power setting Most Significant 2 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ms(&mut self) -> ADV_TRANSMIT_POWER_LVL_MS_W {
        ADV_TRANSMIT_POWER_LVL_MS_W { w: self }
    }
}
