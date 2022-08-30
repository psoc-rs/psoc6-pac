#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSM_EXIT` reader - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
pub type DSM_EXIT_R = crate::BitReader<bool>;
#[doc = "Field `DSM_EXIT` writer - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
pub type DSM_EXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `DSM_ENTERED_INTR_MASK` reader - Masks the DSM Entered Interrupt, when disabled."]
pub type DSM_ENTERED_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `DSM_ENTERED_INTR_MASK` writer - Masks the DSM Entered Interrupt, when disabled."]
pub type DSM_ENTERED_INTR_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `DSM_EXITED_INTR_MASK` reader - Masks the DSM Exited Interrupt, when disabled."]
pub type DSM_EXITED_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `DSM_EXITED_INTR_MASK` writer - Masks the DSM Exited Interrupt, when disabled."]
pub type DSM_EXITED_INTR_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `XTAL_ON_INTR_MASK` reader - Masks the Crystal Stable Interrupt, when disabled."]
pub type XTAL_ON_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_ON_INTR_MASK` writer - Masks the Crystal Stable Interrupt, when disabled."]
pub type XTAL_ON_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RCBLL_INTR_MASK` reader - Mask for RCBLL interrupt"]
pub type RCBLL_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RCBLL_INTR_MASK` writer - Mask for RCBLL interrupt"]
pub type RCBLL_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `BLERD_ACTIVE_INTR_MASK` reader - Mask for CYBLERD55 Active Interrupt"]
pub type BLERD_ACTIVE_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `BLERD_ACTIVE_INTR_MASK` writer - Mask for CYBLERD55 Active Interrupt"]
pub type BLERD_ACTIVE_INTR_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RCB_INTR_MASK` reader - Mask for RCB interrupt"]
pub type RCB_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `RCB_INTR_MASK` writer - Mask for RCB interrupt"]
pub type RCB_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `LL_INTR_MASK` reader - Mask for LL interrupt"]
pub type LL_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `LL_INTR_MASK` writer - Mask for LL interrupt"]
pub type LL_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `GPIO_INTR_MASK` reader - Mask for GPIO interrupt"]
pub type GPIO_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_INTR_MASK` writer - Mask for GPIO interrupt"]
pub type GPIO_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `EFUSE_INTR_MASK` reader - This bit enables the efuse interrupt to firmware"]
pub type EFUSE_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_INTR_MASK` writer - This bit enables the efuse interrupt to firmware"]
pub type EFUSE_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `ENC_INTR_MASK` reader - Mask for Encryption interrupt"]
pub type ENC_INTR_MASK_R = crate::BitReader<bool>;
#[doc = "Field `ENC_INTR_MASK` writer - Mask for Encryption interrupt"]
pub type ENC_INTR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `HVLDO_LV_DETECT_POS_MASK` reader - Mask for HVLDO LV Detector Rise edge interrupt"]
pub type HVLDO_LV_DETECT_POS_MASK_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_LV_DETECT_POS_MASK` writer - Mask for HVLDO LV Detector Rise edge interrupt"]
pub type HVLDO_LV_DETECT_POS_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `HVLDO_LV_DETECT_NEG_MASK` reader - Mask for HVLDO LV Detector Fall edge interrupt"]
pub type HVLDO_LV_DETECT_NEG_MASK_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_LV_DETECT_NEG_MASK` writer - Mask for HVLDO LV Detector Fall edge interrupt"]
pub type HVLDO_LV_DETECT_NEG_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
    #[inline(always)]
    pub fn dsm_exit(&self) -> DSM_EXIT_R {
        DSM_EXIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks the DSM Entered Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_entered_intr_mask(&self) -> DSM_ENTERED_INTR_MASK_R {
        DSM_ENTERED_INTR_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks the DSM Exited Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_exited_intr_mask(&self) -> DSM_EXITED_INTR_MASK_R {
        DSM_EXITED_INTR_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks the Crystal Stable Interrupt, when disabled."]
    #[inline(always)]
    pub fn xtal_on_intr_mask(&self) -> XTAL_ON_INTR_MASK_R {
        XTAL_ON_INTR_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask for RCBLL interrupt"]
    #[inline(always)]
    pub fn rcbll_intr_mask(&self) -> RCBLL_INTR_MASK_R {
        RCBLL_INTR_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for CYBLERD55 Active Interrupt"]
    #[inline(always)]
    pub fn blerd_active_intr_mask(&self) -> BLERD_ACTIVE_INTR_MASK_R {
        BLERD_ACTIVE_INTR_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask for RCB interrupt"]
    #[inline(always)]
    pub fn rcb_intr_mask(&self) -> RCB_INTR_MASK_R {
        RCB_INTR_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask for LL interrupt"]
    #[inline(always)]
    pub fn ll_intr_mask(&self) -> LL_INTR_MASK_R {
        LL_INTR_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask for GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr_mask(&self) -> GPIO_INTR_MASK_R {
        GPIO_INTR_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit enables the efuse interrupt to firmware"]
    #[inline(always)]
    pub fn efuse_intr_mask(&self) -> EFUSE_INTR_MASK_R {
        EFUSE_INTR_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask for Encryption interrupt"]
    #[inline(always)]
    pub fn enc_intr_mask(&self) -> ENC_INTR_MASK_R {
        ENC_INTR_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask for HVLDO LV Detector Rise edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos_mask(&self) -> HVLDO_LV_DETECT_POS_MASK_R {
        HVLDO_LV_DETECT_POS_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask for HVLDO LV Detector Fall edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg_mask(&self) -> HVLDO_LV_DETECT_NEG_MASK_R {
        HVLDO_LV_DETECT_NEG_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
    #[inline(always)]
    pub fn dsm_exit(&mut self) -> DSM_EXIT_W<0> {
        DSM_EXIT_W::new(self)
    }
    #[doc = "Bit 1 - Masks the DSM Entered Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_entered_intr_mask(&mut self) -> DSM_ENTERED_INTR_MASK_W<1> {
        DSM_ENTERED_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 2 - Masks the DSM Exited Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_exited_intr_mask(&mut self) -> DSM_EXITED_INTR_MASK_W<2> {
        DSM_EXITED_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 3 - Masks the Crystal Stable Interrupt, when disabled."]
    #[inline(always)]
    pub fn xtal_on_intr_mask(&mut self) -> XTAL_ON_INTR_MASK_W<3> {
        XTAL_ON_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 4 - Mask for RCBLL interrupt"]
    #[inline(always)]
    pub fn rcbll_intr_mask(&mut self) -> RCBLL_INTR_MASK_W<4> {
        RCBLL_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 5 - Mask for CYBLERD55 Active Interrupt"]
    #[inline(always)]
    pub fn blerd_active_intr_mask(&mut self) -> BLERD_ACTIVE_INTR_MASK_W<5> {
        BLERD_ACTIVE_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 6 - Mask for RCB interrupt"]
    #[inline(always)]
    pub fn rcb_intr_mask(&mut self) -> RCB_INTR_MASK_W<6> {
        RCB_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 7 - Mask for LL interrupt"]
    #[inline(always)]
    pub fn ll_intr_mask(&mut self) -> LL_INTR_MASK_W<7> {
        LL_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 8 - Mask for GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr_mask(&mut self) -> GPIO_INTR_MASK_W<8> {
        GPIO_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 9 - This bit enables the efuse interrupt to firmware"]
    #[inline(always)]
    pub fn efuse_intr_mask(&mut self) -> EFUSE_INTR_MASK_W<9> {
        EFUSE_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 10 - Mask for Encryption interrupt"]
    #[inline(always)]
    pub fn enc_intr_mask(&mut self) -> ENC_INTR_MASK_W<10> {
        ENC_INTR_MASK_W::new(self)
    }
    #[doc = "Bit 11 - Mask for HVLDO LV Detector Rise edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos_mask(&mut self) -> HVLDO_LV_DETECT_POS_MASK_W<11> {
        HVLDO_LV_DETECT_POS_MASK_W::new(self)
    }
    #[doc = "Bit 12 - Mask for HVLDO LV Detector Fall edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg_mask(&mut self) -> HVLDO_LV_DETECT_NEG_MASK_W<12> {
        HVLDO_LV_DETECT_NEG_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Layer interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
