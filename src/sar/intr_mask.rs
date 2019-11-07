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
#[doc = "Reader of field `EOS_MASK`"]
pub type EOS_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOS_MASK`"]
pub struct EOS_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_MASK_W<'a> {
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
#[doc = "Reader of field `OVERFLOW_MASK`"]
pub type OVERFLOW_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERFLOW_MASK`"]
pub struct OVERFLOW_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_MASK_W<'a> {
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
#[doc = "Reader of field `FW_COLLISION_MASK`"]
pub type FW_COLLISION_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW_COLLISION_MASK`"]
pub struct FW_COLLISION_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_COLLISION_MASK_W<'a> {
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
#[doc = "Reader of field `DSI_COLLISION_MASK`"]
pub type DSI_COLLISION_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_COLLISION_MASK`"]
pub struct DSI_COLLISION_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_COLLISION_MASK_W<'a> {
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
#[doc = "Reader of field `INJ_EOC_MASK`"]
pub type INJ_EOC_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_EOC_MASK`"]
pub struct INJ_EOC_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_EOC_MASK_W<'a> {
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
#[doc = "Reader of field `INJ_SATURATE_MASK`"]
pub type INJ_SATURATE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_SATURATE_MASK`"]
pub struct INJ_SATURATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_SATURATE_MASK_W<'a> {
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
#[doc = "Reader of field `INJ_RANGE_MASK`"]
pub type INJ_RANGE_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_RANGE_MASK`"]
pub struct INJ_RANGE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_RANGE_MASK_W<'a> {
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
#[doc = "Reader of field `INJ_COLLISION_MASK`"]
pub type INJ_COLLISION_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_COLLISION_MASK`"]
pub struct INJ_COLLISION_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_COLLISION_MASK_W<'a> {
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
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&self) -> EOS_MASK_R {
        EOS_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&self) -> OVERFLOW_MASK_R {
        OVERFLOW_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&self) -> FW_COLLISION_MASK_R {
        FW_COLLISION_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&self) -> DSI_COLLISION_MASK_R {
        DSI_COLLISION_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&self) -> INJ_EOC_MASK_R {
        INJ_EOC_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&self) -> INJ_SATURATE_MASK_R {
        INJ_SATURATE_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&self) -> INJ_RANGE_MASK_R {
        INJ_RANGE_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&self) -> INJ_COLLISION_MASK_R {
        INJ_COLLISION_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&mut self) -> EOS_MASK_W {
        EOS_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&mut self) -> OVERFLOW_MASK_W {
        OVERFLOW_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&mut self) -> FW_COLLISION_MASK_W {
        FW_COLLISION_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&mut self) -> DSI_COLLISION_MASK_W {
        DSI_COLLISION_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&mut self) -> INJ_EOC_MASK_W {
        INJ_EOC_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&mut self) -> INJ_SATURATE_MASK_W {
        INJ_SATURATE_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&mut self) -> INJ_RANGE_MASK_W {
        INJ_RANGE_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&mut self) -> INJ_COLLISION_MASK_W {
        INJ_COLLISION_MASK_W { w: self }
    }
}
