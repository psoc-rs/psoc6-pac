#[doc = "Reader of register SW_FW_MOD_SEL"]
pub type R = crate::R<u32, super::SW_FW_MOD_SEL>;
#[doc = "Writer for register SW_FW_MOD_SEL"]
pub type W = crate::W<u32, super::SW_FW_MOD_SEL>;
#[doc = "Register SW_FW_MOD_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_FW_MOD_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_F1PM`"]
pub type SW_F1PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_F1PM`"]
pub struct SW_F1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1PM_W<'a> {
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
#[doc = "Reader of field `SW_F1MA`"]
pub type SW_F1MA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_F1MA`"]
pub struct SW_F1MA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SW_F1CA`"]
pub type SW_F1CA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_F1CA`"]
pub struct SW_F1CA_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_F1CA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_C1CC`"]
pub type SW_C1CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C1CC`"]
pub struct SW_C1CC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1CC_W<'a> {
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
#[doc = "Reader of field `SW_C1CD`"]
pub type SW_C1CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C1CD`"]
pub struct SW_C1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1CD_W<'a> {
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
#[doc = "Reader of field `SW_C1F1`"]
pub type SW_C1F1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_C1F1`"]
pub struct SW_C1F1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_C1F1_W<'a> {
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
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&self) -> SW_F1PM_R {
        SW_F1PM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&self) -> SW_F1MA_R {
        SW_F1MA_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&self) -> SW_F1CA_R {
        SW_F1CA_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&self) -> SW_C1CC_R {
        SW_C1CC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&self) -> SW_C1CD_R {
        SW_C1CD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&self) -> SW_C1F1_R {
        SW_C1F1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&mut self) -> SW_F1PM_W {
        SW_F1PM_W { w: self }
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&mut self) -> SW_F1MA_W {
        SW_F1MA_W { w: self }
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&mut self) -> SW_F1CA_W {
        SW_F1CA_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&mut self) -> SW_C1CC_W {
        SW_C1CC_W { w: self }
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&mut self) -> SW_C1CD_W {
        SW_C1CD_W { w: self }
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&mut self) -> SW_C1F1_W {
        SW_C1F1_W { w: self }
    }
}
