#[doc = "Register `CLK_ILO_CONFIG` reader"]
pub struct R(crate::R<CLK_ILO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ILO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ILO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ILO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ILO_CONFIG` writer"]
pub struct W(crate::W<CLK_ILO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ILO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_ILO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ILO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILO_BACKUP` reader - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type ILO_BACKUP_R = crate::BitReader<bool>;
#[doc = "Field `ILO_BACKUP` writer - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
pub type ILO_BACKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_ILO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_ILO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&self) -> ILO_BACKUP_R {
        ILO_BACKUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If backup domain is present on this product, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD on VDDD/VCCD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&mut self) -> ILO_BACKUP_W<0> {
        ILO_BACKUP_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. After enabling, it takes at most two cycles to reach the accuracy spec."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ILO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ilo_config](index.html) module"]
pub struct CLK_ILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ilo_config::R](R) reader structure"]
impl crate::Readable for CLK_ILO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ilo_config::W](W) writer structure"]
impl crate::Writable for CLK_ILO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_ILO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_ILO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
