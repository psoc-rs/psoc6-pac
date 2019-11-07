#[doc = "Reader of register INTR_STAT"]
pub type R = crate::R<u32, super::INTR_STAT>;
#[doc = "Writer for register INTR_STAT"]
pub type W = crate::W<u32, super::INTR_STAT>;
#[doc = "Register INTR_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSM_ENTERED_INTR`"]
pub type DSM_ENTERED_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_ENTERED_INTR`"]
pub struct DSM_ENTERED_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_ENTERED_INTR_W<'a> {
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
#[doc = "Reader of field `DSM_EXITED_INTR`"]
pub type DSM_EXITED_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_EXITED_INTR`"]
pub struct DSM_EXITED_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_EXITED_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RCBLL_DONE_INTR`"]
pub type RCBLL_DONE_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLERD_ACTIVE_INTR`"]
pub type BLERD_ACTIVE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLERD_ACTIVE_INTR`"]
pub struct BLERD_ACTIVE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_ACTIVE_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RCB_INTR`"]
pub type RCB_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LL_INTR`"]
pub type LL_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIO_INTR`"]
pub type GPIO_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_INTR`"]
pub struct GPIO_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_INTR`"]
pub type EFUSE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_INTR`"]
pub struct EFUSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `XTAL_ON_INTR`"]
pub type XTAL_ON_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_ON_INTR`"]
pub struct XTAL_ON_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_ON_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENC_INTR`"]
pub type ENC_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HVLDO_LV_DETECT_POS`"]
pub type HVLDO_LV_DETECT_POS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLDO_LV_DETECT_POS`"]
pub struct HVLDO_LV_DETECT_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_POS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `HVLDO_LV_DETECT_NEG`"]
pub type HVLDO_LV_DETECT_NEG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLDO_LV_DETECT_NEG`"]
pub struct HVLDO_LV_DETECT_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_NEG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&self) -> DSM_ENTERED_INTR_R {
        DSM_ENTERED_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&self) -> DSM_EXITED_INTR_R {
        DSM_EXITED_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RCB transaction Complete"]
    #[inline(always)]
    pub fn rcbll_done_intr(&self) -> RCBLL_DONE_INTR_R {
        RCBLL_DONE_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&self) -> BLERD_ACTIVE_INTR_R {
        BLERD_ACTIVE_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RCB controller Interrupt - Refer to RCB_INTR_STAT register"]
    #[inline(always)]
    pub fn rcb_intr(&self) -> RCB_INTR_R {
        RCB_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LL controller interrupt - Refer to EVENT_INTR register"]
    #[inline(always)]
    pub fn ll_intr(&self) -> LL_INTR_R {
        LL_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&self) -> GPIO_INTR_R {
        GPIO_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&self) -> EFUSE_INTR_R {
        EFUSE_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&self) -> XTAL_ON_INTR_R {
        XTAL_ON_INTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Encryption Interrupt Triggered"]
    #[inline(always)]
    pub fn enc_intr(&self) -> ENC_INTR_R {
        ENC_INTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&self) -> HVLDO_LV_DETECT_POS_R {
        HVLDO_LV_DETECT_POS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&self) -> HVLDO_LV_DETECT_NEG_R {
        HVLDO_LV_DETECT_NEG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - On a firmware request to LL to enter into state machine, working on LF clock, LL transitions into Deep Sleep Mode and asserts this interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_entered_intr(&mut self) -> DSM_ENTERED_INTR_W {
        DSM_ENTERED_INTR_W { w: self }
    }
    #[doc = "Bit 1 - On a firmware request to LL to exit from Deep Sleep Mode, working on LF clock, LL transitions from Deep Sleep Mode and asserts this interrupt when the Deep Sleep clock gater is turned ON. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn dsm_exited_intr(&mut self) -> DSM_EXITED_INTR_W {
        DSM_EXITED_INTR_W { w: self }
    }
    #[doc = "Bit 3 - CYBLERD55 is in active mode. RF is active"]
    #[inline(always)]
    pub fn blerd_active_intr(&mut self) -> BLERD_ACTIVE_INTR_W {
        BLERD_ACTIVE_INTR_W { w: self }
    }
    #[doc = "Bit 6 - GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr(&mut self) -> GPIO_INTR_W {
        GPIO_INTR_W { w: self }
    }
    #[doc = "Bit 7 - This bit when set by efuse controller logic when the efuse read/write is completed"]
    #[inline(always)]
    pub fn efuse_intr(&mut self) -> EFUSE_INTR_W {
        EFUSE_INTR_W { w: self }
    }
    #[doc = "Bit 8 - enabled crystal stable signal rising edge interrupt. The interrupt can be cleared by writing one into this location."]
    #[inline(always)]
    pub fn xtal_on_intr(&mut self) -> XTAL_ON_INTR_W {
        XTAL_ON_INTR_W { w: self }
    }
    #[doc = "Bit 10 - This interrupt is set on HVLDO LV Detector Rise edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos(&mut self) -> HVLDO_LV_DETECT_POS_W {
        HVLDO_LV_DETECT_POS_W { w: self }
    }
    #[doc = "Bit 11 - This interrupt is set on HVLDO LV Detector Fall edge. There is a 1cycle AHB clock glitch filter on the HVLDO LV Detector output"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg(&mut self) -> HVLDO_LV_DETECT_NEG_W {
        HVLDO_LV_DETECT_NEG_W { w: self }
    }
}
