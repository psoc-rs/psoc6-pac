#[doc = "Register `POWER_CTL` reader"]
pub struct R(crate::R<POWER_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_CTL` writer"]
pub struct W(crate::W<POWER_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<POWER_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPEND` reader - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DP_UP_EN` reader - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DP_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `DP_UP_EN` writer - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
pub type DP_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DP_BIG` reader - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DP_BIG_R = crate::BitReader<bool>;
#[doc = "Field `DP_BIG` writer - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
pub type DP_BIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DP_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DP_DOWN_EN_R = crate::BitReader<bool>;
#[doc = "Field `DP_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DP_DOWN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DM_UP_EN` reader - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DM_UP_EN_R = crate::BitReader<bool>;
#[doc = "Field `DM_UP_EN` writer - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
pub type DM_UP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DM_BIG` reader - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DM_BIG_R = crate::BitReader<bool>;
#[doc = "Field `DM_BIG` writer - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
pub type DM_BIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `DM_DOWN_EN` reader - Enables the ~15k pull down on the DP."]
pub type DM_DOWN_EN_R = crate::BitReader<bool>;
#[doc = "Field `DM_DOWN_EN` writer - Enables the ~15k pull down on the DP."]
pub type DM_DOWN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLE_DPO` reader - Enables the single ended receiver on D+."]
pub type ENABLE_DPO_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_DPO` writer - Enables the single ended receiver on D+."]
pub type ENABLE_DPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLE_DMO` reader - Enables the signle ended receiver on D-."]
pub type ENABLE_DMO_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_DMO` writer - Enables the signle ended receiver on D-."]
pub type ENABLE_DMO_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&self) -> DP_UP_EN_R {
        DP_UP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&self) -> DP_BIG_R {
        DP_BIG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&self) -> DP_DOWN_EN_R {
        DP_DOWN_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&self) -> DM_UP_EN_R {
        DM_UP_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&self) -> DM_BIG_R {
        DM_BIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&self) -> DM_DOWN_EN_R {
        DM_DOWN_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&self) -> ENABLE_DPO_R {
        ENABLE_DPO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&self) -> ENABLE_DMO_R {
        ENABLE_DMO_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Put PHY into Suspend mode. If the PHY is enabled, this bit MUST be set before entering a low power mode (DeepSleep). Note: - This bit is invalid if the HOST bit of the Host Control 0 Register (HOST_CTL0) is '1'."]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W<2> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bit 16 - Enables the pull up on the DP. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dp_up_en(&mut self) -> DP_UP_EN_W<16> {
        DP_UP_EN_W::new(self)
    }
    #[doc = "Bit 17 - Select the resister value if POWER_CTL.DP_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DP. '1' : The resister value is from 1425 to 3090Ohmpull up on the DP"]
    #[inline(always)]
    pub fn dp_big(&mut self) -> DP_BIG_W<17> {
        DP_BIG_W::new(self)
    }
    #[doc = "Bit 18 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dp_down_en(&mut self) -> DP_DOWN_EN_W<18> {
        DP_DOWN_EN_W::new(self)
    }
    #[doc = "Bit 19 - Enables the pull up on the DM. The bit is valid in GPIO. The pull up resistor is disabled in not GPIO. '0' : Disable. '1' : Enable."]
    #[inline(always)]
    pub fn dm_up_en(&mut self) -> DM_UP_EN_W<19> {
        DM_UP_EN_W::new(self)
    }
    #[doc = "Bit 20 - Select the resister value if POWER_CTL.DM_EN='1'. This bit is valid in GPIO. '0' : The resister value is from 900 to1575Ohmpull up on the DM. '1' : The resister value is from 1425 to 3090Ohmpull up on the DM"]
    #[inline(always)]
    pub fn dm_big(&mut self) -> DM_BIG_W<20> {
        DM_BIG_W::new(self)
    }
    #[doc = "Bit 21 - Enables the ~15k pull down on the DP."]
    #[inline(always)]
    pub fn dm_down_en(&mut self) -> DM_DOWN_EN_W<21> {
        DM_DOWN_EN_W::new(self)
    }
    #[doc = "Bit 28 - Enables the single ended receiver on D+."]
    #[inline(always)]
    pub fn enable_dpo(&mut self) -> ENABLE_DPO_W<28> {
        ENABLE_DPO_W::new(self)
    }
    #[doc = "Bit 29 - Enables the signle ended receiver on D-."]
    #[inline(always)]
    pub fn enable_dmo(&mut self) -> ENABLE_DMO_W<29> {
        ENABLE_DMO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_ctl](index.html) module"]
pub struct POWER_CTL_SPEC;
impl crate::RegisterSpec for POWER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_ctl::R](R) reader structure"]
impl crate::Readable for POWER_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_ctl::W](W) writer structure"]
impl crate::Writable for POWER_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_CTL to value 0"]
impl crate::Resettable for POWER_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
