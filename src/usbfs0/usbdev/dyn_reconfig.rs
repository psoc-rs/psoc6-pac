#[doc = "Reader of register DYN_RECONFIG"]
pub type R = crate::R<u32, super::DYN_RECONFIG>;
#[doc = "Writer for register DYN_RECONFIG"]
pub type W = crate::W<u32, super::DYN_RECONFIG>;
#[doc = "Register DYN_RECONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::DYN_RECONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DYN_CONFIG_EN`"]
pub type DYN_CONFIG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DYN_CONFIG_EN`"]
pub struct DYN_CONFIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CONFIG_EN_W<'a> {
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
#[doc = "Reader of field `DYN_RECONFIG_EPNO`"]
pub type DYN_RECONFIG_EPNO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DYN_RECONFIG_EPNO`"]
pub struct DYN_RECONFIG_EPNO_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_RECONFIG_EPNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `DYN_RECONFIG_RDY_STS`"]
pub type DYN_RECONFIG_RDY_STS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&self) -> DYN_CONFIG_EN_R {
        DYN_CONFIG_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&self) -> DYN_RECONFIG_EPNO_R {
        DYN_RECONFIG_EPNO_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub fn dyn_reconfig_rdy_sts(&self) -> DYN_RECONFIG_RDY_STS_R {
        DYN_RECONFIG_RDY_STS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&mut self) -> DYN_CONFIG_EN_W {
        DYN_CONFIG_EN_W { w: self }
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&mut self) -> DYN_RECONFIG_EPNO_W {
        DYN_RECONFIG_EPNO_W { w: self }
    }
}
