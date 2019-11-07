#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x0002_0808"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0808
    }
}
#[doc = "Reader of field `PGA_R`"]
pub type PGA_R_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGA_R`"]
pub struct PGA_R_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PGA_L`"]
pub type PGA_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGA_L`"]
pub struct PGA_L_W<'a> {
    w: &'a mut W,
}
impl<'a> PGA_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SOFT_MUTE`"]
pub type SOFT_MUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFT_MUTE`"]
pub struct SOFT_MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_MUTE_W<'a> {
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
#[doc = "Reader of field `STEP_SEL`"]
pub type STEP_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STEP_SEL`"]
pub struct STEP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_SEL_W<'a> {
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
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        SOFT_MUTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&self) -> STEP_SEL_R {
        STEP_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn pga_r(&mut self) -> PGA_R_W {
        PGA_R_W { w: self }
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn pga_l(&mut self) -> PGA_L_W {
        PGA_L_W { w: self }
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn soft_mute(&mut self) -> SOFT_MUTE_W {
        SOFT_MUTE_W { w: self }
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&mut self) -> STEP_SEL_W {
        STEP_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
