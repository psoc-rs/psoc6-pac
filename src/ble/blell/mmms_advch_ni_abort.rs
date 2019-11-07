#[doc = "Reader of register MMMS_ADVCH_NI_ABORT"]
pub type R = crate::R<u32, super::MMMS_ADVCH_NI_ABORT>;
#[doc = "Writer for register MMMS_ADVCH_NI_ABORT"]
pub type W = crate::W<u32, super::MMMS_ADVCH_NI_ABORT>;
#[doc = "Register MMMS_ADVCH_NI_ABORT `reset()`'s with value 0"]
impl crate::ResetValue for super::MMMS_ADVCH_NI_ABORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ADVCH_NI_ABORT`"]
pub struct ADVCH_NI_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVCH_NI_ABORT_W<'a> {
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
#[doc = "Reader of field `ADVCH_ABORT_STATUS`"]
pub type ADVCH_ABORT_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVCH_ABORT_STATUS`"]
pub struct ADVCH_ABORT_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVCH_ABORT_STATUS_W<'a> {
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
    #[doc = "Bit 1 - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
    #[inline(always)]
    pub fn advch_abort_status(&self) -> ADVCH_ABORT_STATUS_R {
        ADVCH_ABORT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FW can use this bit to clear an unserviced NI_VALID for Advertisement or scanner or initiator. HW will clear NI_VALID for ADV/SCAN/INIT if the event has not yet started"]
    #[inline(always)]
    pub fn advch_ni_abort(&mut self) -> ADVCH_NI_ABORT_W {
        ADVCH_NI_ABORT_W { w: self }
    }
    #[doc = "Bit 1 - The link layer hardware logic will set this bit when the NI_TIMER is aborted. Firmware to clear this by writing 1'b1 to this register bit"]
    #[inline(always)]
    pub fn advch_abort_status(&mut self) -> ADVCH_ABORT_STATUS_W {
        ADVCH_ABORT_STATUS_W { w: self }
    }
}
