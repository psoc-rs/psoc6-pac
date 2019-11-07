#[doc = "Reader of register CLK_ECO_CONFIG"]
pub type R = crate::R<u32, super::CLK_ECO_CONFIG>;
#[doc = "Writer for register CLK_ECO_CONFIG"]
pub type W = crate::W<u32, super::CLK_ECO_CONFIG>;
#[doc = "Register CLK_ECO_CONFIG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CLK_ECO_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `AGC_EN`"]
pub type AGC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AGC_EN`"]
pub struct AGC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AGC_EN_W<'a> {
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
#[doc = "Reader of field `ECO_EN`"]
pub type ECO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECO_EN`"]
pub struct ECO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and will grow until it saturates to the supply rail (1.8V nom). WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn agc_en(&self) -> AGC_EN_R {
        AGC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn eco_en(&self) -> ECO_EN_R {
        ECO_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and will grow until it saturates to the supply rail (1.8V nom). WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn agc_en(&mut self) -> AGC_EN_W {
        AGC_EN_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn eco_en(&mut self) -> ECO_EN_W {
        ECO_EN_W { w: self }
    }
}
