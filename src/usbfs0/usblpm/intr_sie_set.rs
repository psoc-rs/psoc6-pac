#[doc = "Reader of register INTR_SIE_SET"]
pub type R = crate::R<u32, super::INTR_SIE_SET>;
#[doc = "Writer for register INTR_SIE_SET"]
pub type W = crate::W<u32, super::INTR_SIE_SET>;
#[doc = "Register INTR_SIE_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SIE_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF_INTR_SET`"]
pub type SOF_INTR_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_INTR_SET`"]
pub struct SOF_INTR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INTR_SET_W<'a> {
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
#[doc = "Reader of field `BUS_RESET_INTR_SET`"]
pub type BUS_RESET_INTR_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_RESET_INTR_SET`"]
pub struct BUS_RESET_INTR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_INTR_SET_W<'a> {
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
#[doc = "Reader of field `EP0_INTR_SET`"]
pub type EP0_INTR_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INTR_SET`"]
pub struct EP0_INTR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INTR_SET_W<'a> {
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
#[doc = "Reader of field `LPM_INTR_SET`"]
pub type LPM_INTR_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_INTR_SET`"]
pub struct LPM_INTR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INTR_SET_W<'a> {
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
#[doc = "Reader of field `RESUME_INTR_SET`"]
pub type RESUME_INTR_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_INTR_SET`"]
pub struct RESUME_INTR_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_INTR_SET_W<'a> {
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
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&self) -> SOF_INTR_SET_R {
        SOF_INTR_SET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&self) -> BUS_RESET_INTR_SET_R {
        BUS_RESET_INTR_SET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&self) -> EP0_INTR_SET_R {
        EP0_INTR_SET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&self) -> LPM_INTR_SET_R {
        LPM_INTR_SET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&self) -> RESUME_INTR_SET_R {
        RESUME_INTR_SET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sof_intr_set(&mut self) -> SOF_INTR_SET_W {
        SOF_INTR_SET_W { w: self }
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn bus_reset_intr_set(&mut self) -> BUS_RESET_INTR_SET_W {
        BUS_RESET_INTR_SET_W { w: self }
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ep0_intr_set(&mut self) -> EP0_INTR_SET_W {
        EP0_INTR_SET_W { w: self }
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn lpm_intr_set(&mut self) -> LPM_INTR_SET_W {
        LPM_INTR_SET_W { w: self }
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn resume_intr_set(&mut self) -> RESUME_INTR_SET_W {
        RESUME_INTR_SET_W { w: self }
    }
}
