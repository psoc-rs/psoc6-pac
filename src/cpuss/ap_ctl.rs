#[doc = "Register `AP_CTL` reader"]
pub struct R(crate::R<AP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AP_CTL` writer"]
pub struct W(crate::W<AP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AP_CTL_SPEC>;
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
impl From<crate::W<AP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM0_ENABLE` reader - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type CM0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CM0_ENABLE` writer - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type CM0_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
#[doc = "Field `CM4_ENABLE` reader - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type CM4_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CM4_ENABLE` writer - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type CM4_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
#[doc = "Field `SYS_ENABLE` reader - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SYS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SYS_ENABLE` writer - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SYS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
#[doc = "Field `CM0_DISABLE` reader - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type CM0_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `CM0_DISABLE` writer - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type CM0_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
#[doc = "Field `CM4_DISABLE` reader - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type CM4_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `CM4_DISABLE` writer - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type CM4_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
#[doc = "Field `SYS_DISABLE` reader - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SYS_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SYS_DISABLE` writer - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SYS_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AP_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&self) -> CM0_ENABLE_R {
        CM0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&self) -> CM4_ENABLE_R {
        CM4_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&self) -> SYS_ENABLE_R {
        SYS_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&self) -> CM0_DISABLE_R {
        CM0_DISABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&self) -> CM4_DISABLE_R {
        CM4_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&self) -> SYS_DISABLE_R {
        SYS_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&mut self) -> CM0_ENABLE_W<0> {
        CM0_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&mut self) -> CM4_ENABLE_W<1> {
        CM4_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&mut self) -> SYS_ENABLE_W<2> {
        SYS_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&mut self) -> CM0_DISABLE_W<16> {
        CM0_DISABLE_W::new(self)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&mut self) -> CM4_DISABLE_W<17> {
        CM4_DISABLE_W::new(self)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&mut self) -> SYS_DISABLE_W<18> {
        SYS_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access port control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ap_ctl](index.html) module"]
pub struct AP_CTL_SPEC;
impl crate::RegisterSpec for AP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ap_ctl::R](R) reader structure"]
impl crate::Readable for AP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ap_ctl::W](W) writer structure"]
impl crate::Writable for AP_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AP_CTL to value 0"]
impl crate::Resettable for AP_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
