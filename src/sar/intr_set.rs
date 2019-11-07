#[doc = "Reader of register INTR_SET"]
pub type R = crate::R<u32, super::INTR_SET>;
#[doc = "Writer for register INTR_SET"]
pub type W = crate::W<u32, super::INTR_SET>;
#[doc = "Register INTR_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOS_SET`"]
pub type EOS_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOS_SET`"]
pub struct EOS_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_SET_W<'a> {
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
#[doc = "Reader of field `OVERFLOW_SET`"]
pub type OVERFLOW_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERFLOW_SET`"]
pub struct OVERFLOW_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_SET_W<'a> {
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
#[doc = "Reader of field `FW_COLLISION_SET`"]
pub type FW_COLLISION_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW_COLLISION_SET`"]
pub struct FW_COLLISION_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_COLLISION_SET_W<'a> {
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
#[doc = "Reader of field `DSI_COLLISION_SET`"]
pub type DSI_COLLISION_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_COLLISION_SET`"]
pub struct DSI_COLLISION_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_COLLISION_SET_W<'a> {
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
#[doc = "Reader of field `INJ_EOC_SET`"]
pub type INJ_EOC_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_EOC_SET`"]
pub struct INJ_EOC_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_EOC_SET_W<'a> {
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
#[doc = "Reader of field `INJ_SATURATE_SET`"]
pub type INJ_SATURATE_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_SATURATE_SET`"]
pub struct INJ_SATURATE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_SATURATE_SET_W<'a> {
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
#[doc = "Reader of field `INJ_RANGE_SET`"]
pub type INJ_RANGE_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_RANGE_SET`"]
pub struct INJ_RANGE_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_RANGE_SET_W<'a> {
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
#[doc = "Reader of field `INJ_COLLISION_SET`"]
pub type INJ_COLLISION_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_COLLISION_SET`"]
pub struct INJ_COLLISION_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_COLLISION_SET_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&self) -> EOS_SET_R {
        EOS_SET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&self) -> OVERFLOW_SET_R {
        OVERFLOW_SET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&self) -> FW_COLLISION_SET_R {
        FW_COLLISION_SET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&self) -> DSI_COLLISION_SET_R {
        DSI_COLLISION_SET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&self) -> INJ_EOC_SET_R {
        INJ_EOC_SET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&self) -> INJ_SATURATE_SET_R {
        INJ_SATURATE_SET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&self) -> INJ_RANGE_SET_R {
        INJ_RANGE_SET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&self) -> INJ_COLLISION_SET_R {
        INJ_COLLISION_SET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&mut self) -> EOS_SET_W {
        EOS_SET_W { w: self }
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&mut self) -> OVERFLOW_SET_W {
        OVERFLOW_SET_W { w: self }
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&mut self) -> FW_COLLISION_SET_W {
        FW_COLLISION_SET_W { w: self }
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&mut self) -> DSI_COLLISION_SET_W {
        DSI_COLLISION_SET_W { w: self }
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&mut self) -> INJ_EOC_SET_W {
        INJ_EOC_SET_W { w: self }
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&mut self) -> INJ_SATURATE_SET_W {
        INJ_SATURATE_SET_W { w: self }
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&mut self) -> INJ_RANGE_SET_W {
        INJ_RANGE_SET_W { w: self }
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&mut self) -> INJ_COLLISION_SET_W {
        INJ_COLLISION_SET_W { w: self }
    }
}
