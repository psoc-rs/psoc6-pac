#[doc = "Reader of register INTR_SIE_MASK"]
pub type R = crate::R<u32, super::INTR_SIE_MASK>;
#[doc = "Writer for register INTR_SIE_MASK"]
pub type W = crate::W<u32, super::INTR_SIE_MASK>;
#[doc = "Register INTR_SIE_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SIE_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF_INTR_MASK`"]
pub type SOF_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_INTR_MASK`"]
pub struct SOF_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `BUS_RESET_INTR_MASK`"]
pub type BUS_RESET_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_RESET_INTR_MASK`"]
pub struct BUS_RESET_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `EP0_INTR_MASK`"]
pub type EP0_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INTR_MASK`"]
pub struct EP0_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `LPM_INTR_MASK`"]
pub type LPM_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_INTR_MASK`"]
pub struct LPM_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INTR_MASK_W<'a> {
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
#[doc = "Reader of field `RESUME_INTR_MASK`"]
pub type RESUME_INTR_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_INTR_MASK`"]
pub struct RESUME_INTR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_INTR_MASK_W<'a> {
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
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&self) -> SOF_INTR_MASK_R {
        SOF_INTR_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&self) -> BUS_RESET_INTR_MASK_R {
        BUS_RESET_INTR_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&self) -> EP0_INTR_MASK_R {
        EP0_INTR_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&self) -> LPM_INTR_MASK_R {
        LPM_INTR_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&self) -> RESUME_INTR_MASK_R {
        RESUME_INTR_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn sof_intr_mask(&mut self) -> SOF_INTR_MASK_W {
        SOF_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn bus_reset_intr_mask(&mut self) -> BUS_RESET_INTR_MASK_W {
        BUS_RESET_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn ep0_intr_mask(&mut self) -> EP0_INTR_MASK_W {
        EP0_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn lpm_intr_mask(&mut self) -> LPM_INTR_MASK_W {
        LPM_INTR_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Set to 1 to enable interrupt corresponding to interrupt request register"]
    #[inline(always)]
    pub fn resume_intr_mask(&mut self) -> RESUME_INTR_MASK_W {
        RESUME_INTR_MASK_W { w: self }
    }
}
