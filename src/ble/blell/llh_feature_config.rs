#[doc = "Reader of register LLH_FEATURE_CONFIG"]
pub type R = crate::R<u32, super::LLH_FEATURE_CONFIG>;
#[doc = "Writer for register LLH_FEATURE_CONFIG"]
pub type W = crate::W<u32, super::LLH_FEATURE_CONFIG>;
#[doc = "Register LLH_FEATURE_CONFIG `reset()`'s with value 0x06"]
impl crate::ResetValue for super::LLH_FEATURE_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `QUICK_TRANSMIT`"]
pub type QUICK_TRANSMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QUICK_TRANSMIT`"]
pub struct QUICK_TRANSMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> QUICK_TRANSMIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SL_DSM_EN`"]
pub type SL_DSM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SL_DSM_EN`"]
pub struct SL_DSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_DSM_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `US_COUNTER_OFFSET_ADJ`"]
pub type US_COUNTER_OFFSET_ADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `US_COUNTER_OFFSET_ADJ`"]
pub struct US_COUNTER_OFFSET_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> US_COUNTER_OFFSET_ADJ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
    #[inline(always)]
    pub fn quick_transmit(&self) -> QUICK_TRANSMIT_R {
        QUICK_TRANSMIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable/Disable Slave Latency Period DSM."]
    #[inline(always)]
    pub fn sl_dsm_en(&self) -> SL_DSM_EN_R {
        SL_DSM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
    #[inline(always)]
    pub fn us_counter_offset_adj(&self) -> US_COUNTER_OFFSET_ADJ_R {
        US_COUNTER_OFFSET_ADJ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quick transmit feature in slave latency is enabled by setting this bit. When slave latency is enabled, this feature enables the slave to transmit in the immediate connection interval, in case required, instead of waiting till the end of slave latency"]
    #[inline(always)]
    pub fn quick_transmit(&mut self) -> QUICK_TRANSMIT_W {
        QUICK_TRANSMIT_W { w: self }
    }
    #[doc = "Bit 1 - Enable/Disable Slave Latency Period DSM."]
    #[inline(always)]
    pub fn sl_dsm_en(&mut self) -> SL_DSM_EN_W {
        SL_DSM_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable/Disable the connection US counter offset adjust. For non-MMMS mode, this bit must be tied to 1."]
    #[inline(always)]
    pub fn us_counter_offset_adj(&mut self) -> US_COUNTER_OFFSET_ADJ_W {
        US_COUNTER_OFFSET_ADJ_W { w: self }
    }
}
