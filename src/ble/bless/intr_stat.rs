#[doc = "Register `INTR_STAT` reader"]
pub struct R(crate::R<INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_STAT` writer"]
pub struct W(crate::W<INTR_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_STAT_SPEC>;
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
impl From<crate::W<INTR_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSM_ENTERED_INTR` reader - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
pub type DSM_ENTERED_INTR_R = crate::BitReader<bool>;
#[doc = "Field `DSM_ENTERED_INTR` writer - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
pub type DSM_ENTERED_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `DSM_EXITED_INTR` reader - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
pub type DSM_EXITED_INTR_R = crate::BitReader<bool>;
#[doc = "Field `DSM_EXITED_INTR` writer - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
pub type DSM_EXITED_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `RCBLL_DONE_INTR` reader - RCB transaction Complete"]
pub type RCBLL_DONE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `BLERD_ACTIVE_INTR` reader - CYBLERD55 is in active mode. RF is active"]
pub type BLERD_ACTIVE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `BLERD_ACTIVE_INTR` writer - CYBLERD55 is in active mode. RF is active"]
pub type BLERD_ACTIVE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `RCB_INTR` reader - RCB controller Interrupt - Refer to RCB_INTR_STAT register"]
pub type RCB_INTR_R = crate::BitReader<bool>;
#[doc = "Field `LL_INTR` reader - LL controller interrupt - Refer to EVENT_INTR register"]
pub type LL_INTR_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_INTR` reader - GPIO interrupt"]
pub type GPIO_INTR_R = crate::BitReader<bool>;
#[doc = "Field `GPIO_INTR` writer - GPIO interrupt"]
pub type GPIO_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `EFUSE_INTR` reader - This bit when set by efuse controller logic when the efuse read/write is completed"]
pub type EFUSE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_INTR` writer - This bit when set by efuse controller logic when the efuse read/write is completed"]
pub type EFUSE_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `XTAL_ON_INTR` reader - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
pub type XTAL_ON_INTR_R = crate::BitReader<bool>;
#[doc = "Field `XTAL_ON_INTR` writer - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
pub type XTAL_ON_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `ENC_INTR` reader - Encryption Interrupt Triggered"]
pub type ENC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_LV_DETECT_POS` reader - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub type HVLDO_LV_DETECT_POS_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_LV_DETECT_POS` writer - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub type HVLDO_LV_DETECT_POS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
#[doc = "Field `HVLDO_LV_DETECT_NEG` reader - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub type HVLDO_LV_DETECT_NEG_R = crate::BitReader<bool>;
#[doc = "Field `HVLDO_LV_DETECT_NEG` writer - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
pub type HVLDO_LV_DETECT_NEG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTR_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&self) -> DSM_ENTERED_INTR_R {
        DSM_ENTERED_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&self) -> DSM_EXITED_INTR_R {
        DSM_EXITED_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RCB transaction Complete"]
    #[inline(always)]
    pub fn rcbll_done_intr(&self) -> RCBLL_DONE_INTR_R {
        RCBLL_DONE_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&self) -> BLERD_ACTIVE_INTR_R {
        BLERD_ACTIVE_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RCB controller Interrupt - Refer to RCB_INTR_STAT register"]
    #[inline(always)]
    pub fn rcb_intr(&self) -> RCB_INTR_R {
        RCB_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LL controller interrupt - Refer to EVENT_INTR register"]
    #[inline(always)]
    pub fn ll_intr(&self) -> LL_INTR_R {
        LL_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&self) -> GPIO_INTR_R {
        GPIO_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&self) -> EFUSE_INTR_R {
        EFUSE_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&self) -> XTAL_ON_INTR_R {
        XTAL_ON_INTR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Encryption Interrupt Triggered"]
    #[inline(always)]
    pub fn enc_intr(&self) -> ENC_INTR_R {
        ENC_INTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&self) -> HVLDO_LV_DETECT_POS_R {
        HVLDO_LV_DETECT_POS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&self) -> HVLDO_LV_DETECT_NEG_R {
        HVLDO_LV_DETECT_NEG_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&mut self) -> DSM_ENTERED_INTR_W<0> {
        DSM_ENTERED_INTR_W::new(self)
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&mut self) -> DSM_EXITED_INTR_W<1> {
        DSM_EXITED_INTR_W::new(self)
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&mut self) -> BLERD_ACTIVE_INTR_W<3> {
        BLERD_ACTIVE_INTR_W::new(self)
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&mut self) -> GPIO_INTR_W<6> {
        GPIO_INTR_W::new(self)
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&mut self) -> EFUSE_INTR_W<7> {
        EFUSE_INTR_W::new(self)
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&mut self) -> XTAL_ON_INTR_W<8> {
        XTAL_ON_INTR_W::new(self)
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&mut self) -> HVLDO_LV_DETECT_POS_W<10> {
        HVLDO_LV_DETECT_POS_W::new(self)
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&mut self) -> HVLDO_LV_DETECT_NEG_W<11> {
        HVLDO_LV_DETECT_NEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link Layer interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_stat](index.html) module"]
pub struct INTR_STAT_SPEC;
impl crate::RegisterSpec for INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_stat::R](R) reader structure"]
impl crate::Readable for INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_stat::W](W) writer structure"]
impl crate::Writable for INTR_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_STAT to value 0"]
impl crate::Resettable for INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
