#[doc = "Reader of register SEQ_DEFAULT"]
pub type R = crate::R<u32, super::SEQ_DEFAULT>;
#[doc = "Writer for register SEQ_DEFAULT"]
pub type W = crate::W<u32, super::SEQ_DEFAULT>;
#[doc = "Register SEQ_DEFAULT `reset()`'s with value 0x001d_0000"]
impl crate::ResetValue for super::SEQ_DEFAULT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001d_0000
    }
}
#[doc = "Reader of field `STROBE_A`"]
pub type STROBE_A_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_A`"]
pub struct STROBE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_A_W<'a> {
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
#[doc = "Reader of field `STROBE_B`"]
pub type STROBE_B_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_B`"]
pub struct STROBE_B_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_B_W<'a> {
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
#[doc = "Reader of field `STROBE_C`"]
pub type STROBE_C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_C`"]
pub struct STROBE_C_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `STROBE_D`"]
pub type STROBE_D_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_D`"]
pub struct STROBE_D_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `STROBE_E`"]
pub type STROBE_E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_E`"]
pub struct STROBE_E_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `STROBE_F`"]
pub type STROBE_F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_F`"]
pub struct STROBE_F_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `STROBE_G`"]
pub type STROBE_G_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STROBE_G`"]
pub struct STROBE_G_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_G_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&self) -> STROBE_A_R {
        STROBE_A_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> STROBE_B_R {
        STROBE_B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> STROBE_C_R {
        STROBE_C_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> STROBE_D_R {
        STROBE_D_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> STROBE_E_R {
        STROBE_E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> STROBE_F_R {
        STROBE_F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> STROBE_G_R {
        STROBE_G_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&mut self) -> STROBE_A_W {
        STROBE_A_W { w: self }
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&mut self) -> STROBE_B_W {
        STROBE_B_W { w: self }
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&mut self) -> STROBE_C_W {
        STROBE_C_W { w: self }
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&mut self) -> STROBE_D_W {
        STROBE_D_W { w: self }
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&mut self) -> STROBE_E_W {
        STROBE_E_W { w: self }
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&mut self) -> STROBE_F_W {
        STROBE_F_W { w: self }
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&mut self) -> STROBE_G_W {
        STROBE_G_W { w: self }
    }
}
