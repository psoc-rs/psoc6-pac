#[doc = "Reader of register ADV_CH_TX_POWER_LVL_LS"]
pub type R = crate::R<u32, super::ADV_CH_TX_POWER_LVL_LS>;
#[doc = "Writer for register ADV_CH_TX_POWER_LVL_LS"]
pub type W = crate::W<u32, super::ADV_CH_TX_POWER_LVL_LS>;
#[doc = "Register ADV_CH_TX_POWER_LVL_LS `reset()`'s with value 0"]
impl crate::ResetValue for super::ADV_CH_TX_POWER_LVL_LS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_TRANSMIT_POWER_LVL_LS`"]
pub type ADV_TRANSMIT_POWER_LVL_LS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADV_TRANSMIT_POWER_LVL_LS`"]
pub struct ADV_TRANSMIT_POWER_LVL_LS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_TRANSMIT_POWER_LVL_LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Advertising channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Advertising channel transmit power code 4 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ls(&self) -> ADV_TRANSMIT_POWER_LVL_LS_R {
        ADV_TRANSMIT_POWER_LVL_LS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 1, this field represents the Advertising channel transmit power setting Least Significant 16 bits. When LL_CONFIG.TX_PA_PWR_LVL_TYPE is 0, the LS 4 bits represents the Advertising channel transmit power code 4 bits."]
    #[inline(always)]
    pub fn adv_transmit_power_lvl_ls(&mut self) -> ADV_TRANSMIT_POWER_LVL_LS_W {
        ADV_TRANSMIT_POWER_LVL_LS_W { w: self }
    }
}
