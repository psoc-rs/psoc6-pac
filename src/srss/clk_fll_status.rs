#[doc = "Reader of register CLK_FLL_STATUS"]
pub type R = crate::R<u32, super::CLK_FLL_STATUS>;
#[doc = "Writer for register CLK_FLL_STATUS"]
pub type W = crate::W<u32, super::CLK_FLL_STATUS>;
#[doc = "Register CLK_FLL_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_FLL_STATUS {
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
#[doc = "Reader of field `CCO_READY`"]
pub type CCO_READY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This indicates that the CCO is internally settled and ready to use."]
    #[inline(always)]
    pub fn cco_ready(&self) -> CCO_READY_R {
        CCO_READY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W {
        UNLOCK_OCCURRED_W { w: self }
    }
}
