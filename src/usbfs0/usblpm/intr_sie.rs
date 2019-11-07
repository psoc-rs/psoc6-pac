#[doc = "Reader of register INTR_SIE"]
pub type R = crate::R<u32, super::INTR_SIE>;
#[doc = "Writer for register INTR_SIE"]
pub type W = crate::W<u32, super::INTR_SIE>;
#[doc = "Register INTR_SIE `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_SIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF_INTR`"]
pub type SOF_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_INTR`"]
pub struct SOF_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INTR_W<'a> {
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
#[doc = "Reader of field `BUS_RESET_INTR`"]
pub type BUS_RESET_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_RESET_INTR`"]
pub struct BUS_RESET_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_INTR_W<'a> {
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
#[doc = "Reader of field `EP0_INTR`"]
pub type EP0_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP0_INTR`"]
pub struct EP0_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_INTR_W<'a> {
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
#[doc = "Reader of field `LPM_INTR`"]
pub type LPM_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_INTR`"]
pub struct LPM_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_INTR_W<'a> {
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
#[doc = "Reader of field `RESUME_INTR`"]
pub type RESUME_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_INTR`"]
pub struct RESUME_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_INTR_W<'a> {
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
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SOF_INTR_R {
        SOF_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BUS_RESET_INTR_R {
        BUS_RESET_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> EP0_INTR_R {
        EP0_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LPM_INTR_R {
        LPM_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&self) -> RESUME_INTR_R {
        RESUME_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status for USB SOF"]
    #[inline(always)]
    pub fn sof_intr(&mut self) -> SOF_INTR_W {
        SOF_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt status for BUS RESET"]
    #[inline(always)]
    pub fn bus_reset_intr(&mut self) -> BUS_RESET_INTR_W {
        BUS_RESET_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt status for EP0"]
    #[inline(always)]
    pub fn ep0_intr(&mut self) -> EP0_INTR_W {
        EP0_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt status for LPM (Link Power Management, L1 entry)"]
    #[inline(always)]
    pub fn lpm_intr(&mut self) -> LPM_INTR_W {
        LPM_INTR_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt status for Resume"]
    #[inline(always)]
    pub fn resume_intr(&mut self) -> RESUME_INTR_W {
        RESUME_INTR_W { w: self }
    }
}
