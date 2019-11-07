#[doc = "Reader of register POWER_CTL"]
pub type R = crate::R<u32, super::POWER_CTL>;
#[doc = "Writer for register POWER_CTL"]
pub type W = crate::W<u32, super::POWER_CTL>;
#[doc = "Register POWER_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Reader of field `DP_UP_EN`"]
pub type DP_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_UP_EN`"]
pub struct DP_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DP_BIG`"]
pub type DP_BIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_BIG`"]
pub struct DP_BIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_BIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DP_DOWN_EN`"]
pub type DP_DOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP_DOWN_EN`"]
pub struct DP_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_DOWN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DM_UP_EN`"]
pub type DM_UP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_UP_EN`"]
pub struct DM_UP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_UP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DM_BIG`"]
pub type DM_BIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_BIG`"]
pub struct DM_BIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_BIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DM_DOWN_EN`"]
pub type DM_DOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DM_DOWN_EN`"]
pub struct DM_DOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_DOWN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_DPO`"]
pub type ENABLE_DPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_DPO`"]
pub struct ENABLE_DPO_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DPO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_DMO`"]
pub type ENABLE_DMO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_DMO`"]
pub struct ENABLE_DMO_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DMO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&self) -> DP_UP_EN_R {
        DP_UP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Opull up on the DP. '1' : The resister value is from 1425 to 3090Opull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&self) -> DP_BIG_R {
        DP_BIG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&self) -> DP_DOWN_EN_R {
        DP_DOWN_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&self) -> DM_UP_EN_R {
        DM_UP_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Opull up on the DM. '1' : The resister value is from 1425 to 3090Opull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&self) -> DM_BIG_R {
        DM_BIG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&self) -> DM_DOWN_EN_R {
        DM_DOWN_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&self) -> ENABLE_DPO_R {
        ENABLE_DPO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&self) -> ENABLE_DMO_R {
        ENABLE_DMO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&mut self) -> DP_UP_EN_W {
        DP_UP_EN_W { w: self }
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Opull up on the DP. '1' : The resister value is from 1425 to 3090Opull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&mut self) -> DP_BIG_W {
        DP_BIG_W { w: self }
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&mut self) -> DP_DOWN_EN_W {
        DP_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&mut self) -> DM_UP_EN_W {
        DM_UP_EN_W { w: self }
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Opull up on the DM. '1' : The resister value is from 1425 to 3090Opull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&mut self) -> DM_BIG_W {
        DM_BIG_W { w: self }
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&mut self) -> DM_DOWN_EN_W {
        DM_DOWN_EN_W { w: self }
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&mut self) -> ENABLE_DPO_W {
        ENABLE_DPO_W { w: self }
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&mut self) -> ENABLE_DMO_W {
        ENABLE_DMO_W { w: self }
    }
}
