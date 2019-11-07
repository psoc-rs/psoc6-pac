#[doc = "Reader of register CLK_PLL_STATUS[%s]"]
pub type R = crate::R<u32, super::CLK_PLL_STATUS>;
#[doc = "Writer for register CLK_PLL_STATUS[%s]"]
pub type W = crate::W<u32, super::CLK_PLL_STATUS>;
#[doc = "Register CLK_PLL_STATUS[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_PLL_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKED`"]
pub type LOCKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNLOCK_OCCURRED`"]
pub type UNLOCK_OCCURRED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNLOCK_OCCURRED`"]
pub struct UNLOCK_OCCURRED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_OCCURRED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W {
        UNLOCK_OCCURRED_W { w: self }
    }
}
