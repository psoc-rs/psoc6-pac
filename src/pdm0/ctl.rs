#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PGA_R` reader - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PGA_R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGA_R` writer - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PGA_R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PGA_L` reader - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PGA_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGA_L` writer - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PGA_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SOFT_MUTE` reader - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SOFT_MUTE_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_MUTE` writer - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SOFT_MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `STEP_SEL` reader - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type STEP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `STEP_SEL` writer - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type STEP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn pga_r(&self) -> PGA_R_R {
        PGA_R_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn pga_l(&self) -> PGA_L_R {
        PGA_L_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn soft_mute(&self) -> SOFT_MUTE_R {
        SOFT_MUTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&self) -> STEP_SEL_R {
        STEP_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn pga_r(&mut self) -> PGA_R_W<0> {
        PGA_R_W::new(self)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn pga_l(&mut self) -> PGA_L_W<8> {
        PGA_L_W::new(self)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn soft_mute(&mut self) -> SOFT_MUTE_W<16> {
        SOFT_MUTE_W::new(self)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&mut self) -> STEP_SEL_W<17> {
        STEP_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x0002_0808"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0808
    }
}
