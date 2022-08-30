#[doc = "Register `EXT_PA_LNA_CTRL` reader"]
pub struct R(crate::R<EXT_PA_LNA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_PA_LNA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_PA_LNA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_PA_LNA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_PA_LNA_CTRL` writer"]
pub struct W(crate::W<EXT_PA_LNA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_PA_LNA_CTRL_SPEC>;
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
impl From<crate::W<EXT_PA_LNA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_PA_LNA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_EXT_PA_LNA` reader - When set to 1, enables the external PA & LNA"]
pub type ENABLE_EXT_PA_LNA_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_EXT_PA_LNA` writer - When set to 1, enables the external PA & LNA"]
pub type ENABLE_EXT_PA_LNA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_PA_LNA_CTRL_SPEC, bool, O>;
#[doc = "Field `CHIP_EN_POL` reader - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type CHIP_EN_POL_R = crate::BitReader<bool>;
#[doc = "Field `CHIP_EN_POL` writer - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type CHIP_EN_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_PA_LNA_CTRL_SPEC, bool, O>;
#[doc = "Field `PA_CTRL_POL` reader - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type PA_CTRL_POL_R = crate::BitReader<bool>;
#[doc = "Field `PA_CTRL_POL` writer - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type PA_CTRL_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_PA_LNA_CTRL_SPEC, bool, O>;
#[doc = "Field `LNA_CTRL_POL` reader - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type LNA_CTRL_POL_R = crate::BitReader<bool>;
#[doc = "Field `LNA_CTRL_POL` writer - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
pub type LNA_CTRL_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXT_PA_LNA_CTRL_SPEC, bool, O>;
#[doc = "Field `OUT_EN_DRIVE_VAL` reader - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
pub type OUT_EN_DRIVE_VAL_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EN_DRIVE_VAL` writer - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
pub type OUT_EN_DRIVE_VAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXT_PA_LNA_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - When set to 1, enables the external PA & LNA"]
    #[inline(always)]
    pub fn enable_ext_pa_lna(&self) -> ENABLE_EXT_PA_LNA_R {
        ENABLE_EXT_PA_LNA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn chip_en_pol(&self) -> CHIP_EN_POL_R {
        CHIP_EN_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn pa_ctrl_pol(&self) -> PA_CTRL_POL_R {
        PA_CTRL_POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn lna_ctrl_pol(&self) -> LNA_CTRL_POL_R {
        LNA_CTRL_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
    #[inline(always)]
    pub fn out_en_drive_val(&self) -> OUT_EN_DRIVE_VAL_R {
        OUT_EN_DRIVE_VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When set to 1, enables the external PA & LNA"]
    #[inline(always)]
    pub fn enable_ext_pa_lna(&mut self) -> ENABLE_EXT_PA_LNA_W<1> {
        ENABLE_EXT_PA_LNA_W::new(self)
    }
    #[doc = "Bit 2 - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn chip_en_pol(&mut self) -> CHIP_EN_POL_W<2> {
        CHIP_EN_POL_W::new(self)
    }
    #[doc = "Bit 3 - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn pa_ctrl_pol(&mut self) -> PA_CTRL_POL_W<3> {
        PA_CTRL_POL_W::new(self)
    }
    #[doc = "Bit 4 - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn lna_ctrl_pol(&mut self) -> LNA_CTRL_POL_W<4> {
        LNA_CTRL_POL_W::new(self)
    }
    #[doc = "Bit 5 - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
    #[inline(always)]
    pub fn out_en_drive_val(&mut self) -> OUT_EN_DRIVE_VAL_W<5> {
        OUT_EN_DRIVE_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External TX PA and RX LNA control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_pa_lna_ctrl](index.html) module"]
pub struct EXT_PA_LNA_CTRL_SPEC;
impl crate::RegisterSpec for EXT_PA_LNA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_pa_lna_ctrl::R](R) reader structure"]
impl crate::Readable for EXT_PA_LNA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_pa_lna_ctrl::W](W) writer structure"]
impl crate::Writable for EXT_PA_LNA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_PA_LNA_CTRL to value 0"]
impl crate::Resettable for EXT_PA_LNA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
