#[doc = "Reader of register EXT_PA_LNA_CTRL"]
pub type R = crate::R<u32, super::EXT_PA_LNA_CTRL>;
#[doc = "Writer for register EXT_PA_LNA_CTRL"]
pub type W = crate::W<u32, super::EXT_PA_LNA_CTRL>;
#[doc = "Register EXT_PA_LNA_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXT_PA_LNA_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE_EXT_PA_LNA`"]
pub type ENABLE_EXT_PA_LNA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_EXT_PA_LNA`"]
pub struct ENABLE_EXT_PA_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_EXT_PA_LNA_W<'a> {
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
#[doc = "Reader of field `CHIP_EN_POL`"]
pub type CHIP_EN_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHIP_EN_POL`"]
pub struct CHIP_EN_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_EN_POL_W<'a> {
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
#[doc = "Reader of field `PA_CTRL_POL`"]
pub type PA_CTRL_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA_CTRL_POL`"]
pub struct PA_CTRL_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_CTRL_POL_W<'a> {
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
#[doc = "Reader of field `LNA_CTRL_POL`"]
pub type LNA_CTRL_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNA_CTRL_POL`"]
pub struct LNA_CTRL_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CTRL_POL_W<'a> {
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
#[doc = "Reader of field `OUT_EN_DRIVE_VAL`"]
pub type OUT_EN_DRIVE_VAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EN_DRIVE_VAL`"]
pub struct OUT_EN_DRIVE_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_DRIVE_VAL_W<'a> {
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
impl R {
    #[doc = "Bit 1 - When set to 1, enables the external PA & LNA"]
    #[inline(always)]
    pub fn enable_ext_pa_lna(&self) -> ENABLE_EXT_PA_LNA_R {
        ENABLE_EXT_PA_LNA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn chip_en_pol(&self) -> CHIP_EN_POL_R {
        CHIP_EN_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn pa_ctrl_pol(&self) -> PA_CTRL_POL_R {
        PA_CTRL_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn lna_ctrl_pol(&self) -> LNA_CTRL_POL_R {
        LNA_CTRL_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
    #[inline(always)]
    pub fn out_en_drive_val(&self) -> OUT_EN_DRIVE_VAL_R {
        OUT_EN_DRIVE_VAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When set to 1, enables the external PA & LNA"]
    #[inline(always)]
    pub fn enable_ext_pa_lna(&mut self) -> ENABLE_EXT_PA_LNA_W {
        ENABLE_EXT_PA_LNA_W { w: self }
    }
    #[doc = "Bit 2 - Controls the polarity of the chip enable control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn chip_en_pol(&mut self) -> CHIP_EN_POL_W {
        CHIP_EN_POL_W { w: self }
    }
    #[doc = "Bit 3 - Controls the polarity of the PA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn pa_ctrl_pol(&mut self) -> PA_CTRL_POL_W {
        PA_CTRL_POL_W { w: self }
    }
    #[doc = "Bit 4 - Controls the polarity of the LNA control signal 0 - High enable, low disable 1 - Low enable, High disable"]
    #[inline(always)]
    pub fn lna_ctrl_pol(&mut self) -> LNA_CTRL_POL_W {
        LNA_CTRL_POL_W { w: self }
    }
    #[doc = "Bit 5 - Configures the drive value on the output enables of PA, LNA and CHI_EN signals 0 - drive 0 on the output enable signals 1 - drive 1 on the output enable signals"]
    #[inline(always)]
    pub fn out_en_drive_val(&mut self) -> OUT_EN_DRIVE_VAL_W {
        OUT_EN_DRIVE_VAL_W { w: self }
    }
}
