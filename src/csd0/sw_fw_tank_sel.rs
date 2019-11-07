#[doc = "Reader of register SW_FW_TANK_SEL"]
pub type R = crate::R<u32, super::SW_FW_TANK_SEL>;
#[doc = "Writer for register SW_FW_TANK_SEL"]
pub type W = crate::W<u32, super::SW_FW_TANK_SEL>;
#[doc = "Register SW_FW_TANK_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_FW_TANK_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_F2PT`"]
pub type SW_F2PT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_F2PT`"]
pub struct SW_F2PT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2PT_W<'a> {
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
#[doc = "Reader of field `SW_F2MA`"]
pub type SW_F2MA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_F2MA`"]
pub struct SW_F2MA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_F2CA`"]
pub type SW_F2CA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_F2CA`"]
pub struct SW_F2CA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2CA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SW_F2CB`"]
pub type SW_F2CB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_F2CB`"]
pub struct SW_F2CB_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F2CB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_C2CC`"]
pub type SW_C2CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C2CC`"]
pub struct SW_C2CC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2CC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SW_C2CD`"]
pub type SW_C2CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C2CD`"]
pub struct SW_C2CD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2CD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SW_C2F2`"]
pub type SW_C2F2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C2F2`"]
pub struct SW_C2F2_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C2F2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&self) -> SW_F2PT_R {
        SW_F2PT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&self) -> SW_F2MA_R {
        SW_F2MA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&self) -> SW_F2CA_R {
        SW_F2CA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&self) -> SW_F2CB_R {
        SW_F2CB_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&self) -> SW_C2CC_R {
        SW_C2CC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&self) -> SW_C2CD_R {
        SW_C2CD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&self) -> SW_C2F2_R {
        SW_C2F2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&mut self) -> SW_F2PT_W {
        SW_F2PT_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&mut self) -> SW_F2MA_W {
        SW_F2MA_W { w: self }
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&mut self) -> SW_F2CA_W {
        SW_F2CA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&mut self) -> SW_F2CB_W {
        SW_F2CB_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&mut self) -> SW_C2CC_W {
        SW_C2CC_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&mut self) -> SW_C2CD_W {
        SW_C2CD_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&mut self) -> SW_C2F2_W {
        SW_C2F2_W { w: self }
    }
}
