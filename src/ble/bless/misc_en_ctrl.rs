#[doc = "Reader of register MISC_EN_CTRL"]
pub type R = crate::R<u32, super::MISC_EN_CTRL>;
#[doc = "Writer for register MISC_EN_CTRL"]
pub type W = crate::W<u32, super::MISC_EN_CTRL>;
#[doc = "Register MISC_EN_CTRL `reset()`'s with value 0x08"]
impl crate::ResetValue for super::MISC_EN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `BUCK_EN_CTRL`"]
pub type BUCK_EN_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUCK_EN_CTRL`"]
pub struct BUCK_EN_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_EN_CTRL_W<'a> {
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
#[doc = "Reader of field `ACT_REG_EN_CTRL`"]
pub type ACT_REG_EN_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACT_REG_EN_CTRL`"]
pub struct ACT_REG_EN_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_EN_CTRL_W<'a> {
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
#[doc = "Reader of field `LPM_DRIFT_EN`"]
pub type LPM_DRIFT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_DRIFT_EN`"]
pub struct LPM_DRIFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_DRIFT_EN_W<'a> {
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
#[doc = "Reader of field `LPM_DRIFT_MULTI`"]
pub type LPM_DRIFT_MULTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_DRIFT_MULTI`"]
pub struct LPM_DRIFT_MULTI_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_DRIFT_MULTI_W<'a> {
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
#[doc = "Reader of field `LPM_ENTRY_CTRL_MODE`"]
pub type LPM_ENTRY_CTRL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_ENTRY_CTRL_MODE`"]
pub struct LPM_ENTRY_CTRL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_ENTRY_CTRL_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn buck_en_ctrl(&self) -> BUCK_EN_CTRL_R {
        BUCK_EN_CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn act_reg_en_ctrl(&self) -> ACT_REG_EN_CTRL_R {
        ACT_REG_EN_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
    #[inline(always)]
    pub fn lpm_drift_en(&self) -> LPM_DRIFT_EN_R {
        LPM_DRIFT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
    #[inline(always)]
    pub fn lpm_drift_multi(&self) -> LPM_DRIFT_MULTI_R {
        LPM_DRIFT_MULTI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
    #[inline(always)]
    pub fn lpm_entry_ctrl_mode(&self) -> LPM_ENTRY_CTRL_MODE_R {
        LPM_ENTRY_CTRL_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn buck_en_ctrl(&mut self) -> BUCK_EN_CTRL_W {
        BUCK_EN_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn act_reg_en_ctrl(&mut self) -> ACT_REG_EN_CTRL_W {
        ACT_REG_EN_CTRL_W { w: self }
    }
    #[doc = "Bit 2 - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
    #[inline(always)]
    pub fn lpm_drift_en(&mut self) -> LPM_DRIFT_EN_W {
        LPM_DRIFT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
    #[inline(always)]
    pub fn lpm_drift_multi(&mut self) -> LPM_DRIFT_MULTI_W {
        LPM_DRIFT_MULTI_W { w: self }
    }
    #[doc = "Bit 4 - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
    #[inline(always)]
    pub fn lpm_entry_ctrl_mode(&mut self) -> LPM_ENTRY_CTRL_MODE_W {
        LPM_ENTRY_CTRL_MODE_W { w: self }
    }
}
