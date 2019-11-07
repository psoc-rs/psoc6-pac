#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCB_LL_DONE`"]
pub type RCB_LL_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCB_LL_DONE`"]
pub struct RCB_LL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCB_LL_DONE_W<'a> {
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
#[doc = "Reader of field `SINGLE_WRITE_DONE`"]
pub type SINGLE_WRITE_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE_WRITE_DONE`"]
pub struct SINGLE_WRITE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_WRITE_DONE_W<'a> {
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
#[doc = "Reader of field `SINGLE_READ_DONE`"]
pub type SINGLE_READ_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE_READ_DONE`"]
pub struct SINGLE_READ_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_READ_DONE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RCB_LL is done and the access is given back to CPU"]
    #[inline(always)]
    pub fn rcb_ll_done(&self) -> RCB_LL_DONE_R {
        RCB_LL_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&self) -> SINGLE_WRITE_DONE_R {
        SINGLE_WRITE_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&self) -> SINGLE_READ_DONE_R {
        SINGLE_READ_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCB_LL is done and the access is given back to CPU"]
    #[inline(always)]
    pub fn rcb_ll_done(&mut self) -> RCB_LL_DONE_W {
        RCB_LL_DONE_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&mut self) -> SINGLE_WRITE_DONE_W {
        SINGLE_WRITE_DONE_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&mut self) -> SINGLE_READ_DONE_W {
        SINGLE_READ_DONE_W { w: self }
    }
}
