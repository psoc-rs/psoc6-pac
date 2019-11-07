#[doc = "Reader of register NI_ABORT"]
pub type R = crate::R<u32, super::NI_ABORT>;
#[doc = "Writer for register NI_ABORT"]
pub type W = crate::W<u32, super::NI_ABORT>;
#[doc = "Register NI_ABORT `reset()`'s with value 0"]
impl crate::ResetValue for super::NI_ABORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NI_ABORT`"]
pub type NI_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NI_ABORT`"]
pub struct NI_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> NI_ABORT_W<'a> {
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
#[doc = "Reader of field `ABORT_ACK`"]
pub type ABORT_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT_ACK`"]
pub struct ABORT_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_ACK_W<'a> {
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
    #[doc = "Bit 0 - Setting this bit clears the schedule NI"]
    #[inline(always)]
    pub fn ni_abort(&self) -> NI_ABORT_R {
        NI_ABORT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit will set if the scheduled NI is aborted"]
    #[inline(always)]
    pub fn abort_ack(&self) -> ABORT_ACK_R {
        ABORT_ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit clears the schedule NI"]
    #[inline(always)]
    pub fn ni_abort(&mut self) -> NI_ABORT_W {
        NI_ABORT_W { w: self }
    }
    #[doc = "Bit 1 - This bit will set if the scheduled NI is aborted"]
    #[inline(always)]
    pub fn abort_ack(&mut self) -> ABORT_ACK_W {
        ABORT_ACK_W { w: self }
    }
}
