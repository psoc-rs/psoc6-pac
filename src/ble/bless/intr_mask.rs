#[doc = "Reader of register INTR_MASK"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register INTR_MASK"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSM_EXIT`"]
pub type DSM_EXIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_EXIT`"]
pub struct DSM_EXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_EXIT_W<'a> {
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
#[doc = "Reader of field `DSM_ENTERED_INTR_MASK`"]
pub type DSM_ENTERED_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_ENTERED_INTR_MASK`"]
pub struct DSM_ENTERED_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_ENTERED_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `DSM_EXITED_INTR_MASK`"]
pub type DSM_EXITED_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSM_EXITED_INTR_MASK`"]
pub struct DSM_EXITED_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSM_EXITED_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `XTAL_ON_INTR_MASK`"]
pub type XTAL_ON_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_ON_INTR_MASK`"]
pub struct XTAL_ON_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_ON_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `RCBLL_INTR_MASK`"]
pub type RCBLL_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCBLL_INTR_MASK`"]
pub struct RCBLL_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCBLL_INTR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `BLERD_ACTIVE_INTR_MASK`"]
pub type BLERD_ACTIVE_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BLERD_ACTIVE_INTR_MASK`"]
pub struct BLERD_ACTIVE_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLERD_ACTIVE_INTR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RCB_INTR_MASK`"]
pub type RCB_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCB_INTR_MASK`"]
pub struct RCB_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `LL_INTR_MASK`"]
pub type LL_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LL_INTR_MASK`"]
pub struct LL_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `GPIO_INTR_MASK`"]
pub type GPIO_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_INTR_MASK`"]
pub struct GPIO_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `EFUSE_INTR_MASK`"]
pub type EFUSE_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_INTR_MASK`"]
pub struct EFUSE_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_INTR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ENC_INTR_MASK`"]
pub type ENC_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENC_INTR_MASK`"]
pub struct ENC_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `HVLDO_LV_DETECT_POS_MASK`"]
pub type HVLDO_LV_DETECT_POS_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLDO_LV_DETECT_POS_MASK`"]
pub struct HVLDO_LV_DETECT_POS_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_POS_MASK_W<'a> {
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
#[doc = "Reader of field `HVLDO_LV_DETECT_NEG_MASK`"]
pub type HVLDO_LV_DETECT_NEG_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLDO_LV_DETECT_NEG_MASK`"]
pub struct HVLDO_LV_DETECT_NEG_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLDO_LV_DETECT_NEG_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
    #[inline(always)]
    pub fn dsm_exit(&self) -> DSM_EXIT_R {
        DSM_EXIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Masks the DSM Entered Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_entered_intr_mask(&self) -> DSM_ENTERED_INTR_MASK_R {
        DSM_ENTERED_INTR_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Masks the DSM Exited Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_exited_intr_mask(&self) -> DSM_EXITED_INTR_MASK_R {
        DSM_EXITED_INTR_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Masks the Crystal Stable Interrupt, when disabled."]
    #[inline(always)]
    pub fn xtal_on_intr_mask(&self) -> XTAL_ON_INTR_MASK_R {
        XTAL_ON_INTR_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask for RCBLL interrupt"]
    #[inline(always)]
    pub fn rcbll_intr_mask(&self) -> RCBLL_INTR_MASK_R {
        RCBLL_INTR_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask for CYBLERD55 Active Interrupt"]
    #[inline(always)]
    pub fn blerd_active_intr_mask(&self) -> BLERD_ACTIVE_INTR_MASK_R {
        BLERD_ACTIVE_INTR_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask for RCB interrupt"]
    #[inline(always)]
    pub fn rcb_intr_mask(&self) -> RCB_INTR_MASK_R {
        RCB_INTR_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask for LL interrupt"]
    #[inline(always)]
    pub fn ll_intr_mask(&self) -> LL_INTR_MASK_R {
        LL_INTR_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask for GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr_mask(&self) -> GPIO_INTR_MASK_R {
        GPIO_INTR_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit enables the efuse interrupt to firmware"]
    #[inline(always)]
    pub fn efuse_intr_mask(&self) -> EFUSE_INTR_MASK_R {
        EFUSE_INTR_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask for Encryption interrupt"]
    #[inline(always)]
    pub fn enc_intr_mask(&self) -> ENC_INTR_MASK_R {
        ENC_INTR_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask for HVLDO LV Detector Rise edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos_mask(&self) -> HVLDO_LV_DETECT_POS_MASK_R {
        HVLDO_LV_DETECT_POS_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask for HVLDO LV Detector Fall edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg_mask(&self) -> HVLDO_LV_DETECT_NEG_MASK_R {
        HVLDO_LV_DETECT_NEG_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the Link Layer is in Deep Sleep Mode, firmware can set this bit to wake the Link Layer."]
    #[inline(always)]
    pub fn dsm_exit(&mut self) -> DSM_EXIT_W {
        DSM_EXIT_W { w: self }
    }
    #[doc = "Bit 1 - Masks the DSM Entered Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_entered_intr_mask(&mut self) -> DSM_ENTERED_INTR_MASK_W {
        DSM_ENTERED_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Masks the DSM Exited Interrupt, when disabled."]
    #[inline(always)]
    pub fn dsm_exited_intr_mask(&mut self) -> DSM_EXITED_INTR_MASK_W {
        DSM_EXITED_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Masks the Crystal Stable Interrupt, when disabled."]
    #[inline(always)]
    pub fn xtal_on_intr_mask(&mut self) -> XTAL_ON_INTR_MASK_W {
        XTAL_ON_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Mask for RCBLL interrupt"]
    #[inline(always)]
    pub fn rcbll_intr_mask(&mut self) -> RCBLL_INTR_MASK_W {
        RCBLL_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Mask for CYBLERD55 Active Interrupt"]
    #[inline(always)]
    pub fn blerd_active_intr_mask(&mut self) -> BLERD_ACTIVE_INTR_MASK_W {
        BLERD_ACTIVE_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Mask for RCB interrupt"]
    #[inline(always)]
    pub fn rcb_intr_mask(&mut self) -> RCB_INTR_MASK_W {
        RCB_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Mask for LL interrupt"]
    #[inline(always)]
    pub fn ll_intr_mask(&mut self) -> LL_INTR_MASK_W {
        LL_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 8 - Mask for GPIO interrupt"]
    #[inline(always)]
    pub fn gpio_intr_mask(&mut self) -> GPIO_INTR_MASK_W {
        GPIO_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 9 - This bit enables the efuse interrupt to firmware"]
    #[inline(always)]
    pub fn efuse_intr_mask(&mut self) -> EFUSE_INTR_MASK_W {
        EFUSE_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 10 - Mask for Encryption interrupt"]
    #[inline(always)]
    pub fn enc_intr_mask(&mut self) -> ENC_INTR_MASK_W {
        ENC_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 11 - Mask for HVLDO LV Detector Rise edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_pos_mask(&mut self) -> HVLDO_LV_DETECT_POS_MASK_W {
        HVLDO_LV_DETECT_POS_MASK_W { w: self }
    }
    #[doc = "Bit 12 - Mask for HVLDO LV Detector Fall edge interrupt"]
    #[inline(always)]
    pub fn hvldo_lv_detect_neg_mask(&mut self) -> HVLDO_LV_DETECT_NEG_MASK_W {
        HVLDO_LV_DETECT_NEG_MASK_W { w: self }
    }
}
