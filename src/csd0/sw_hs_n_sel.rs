#[doc = "Reader of register SW_HS_N_SEL"]
pub type R = crate::R<u32, super::SW_HS_N_SEL>;
#[doc = "Writer for register SW_HS_N_SEL"]
pub type W = crate::W<u32, super::SW_HS_N_SEL>;
#[doc = "Register SW_HS_N_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_HS_N_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_HCCC`"]
pub type SW_HCCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HCCC`"]
pub struct SW_HCCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SW_HCCD`"]
pub type SW_HCCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_HCCD`"]
pub struct SW_HCCD_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCCD_W<'a> {
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
#[doc = "Reader of field `SW_HCRH`"]
pub type SW_HCRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCRH`"]
pub struct SW_HCRH_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `SW_HCRL`"]
pub type SW_HCRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_HCRL`"]
pub struct SW_HCRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_HCRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&self) -> SW_HCCC_R {
        SW_HCCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&self) -> SW_HCCD_R {
        SW_HCCD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&self) -> SW_HCRH_R {
        SW_HCRH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&self) -> SW_HCRL_R {
        SW_HCRL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&mut self) -> SW_HCCC_W {
        SW_HCCC_W { w: self }
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&mut self) -> SW_HCCD_W {
        SW_HCCD_W { w: self }
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&mut self) -> SW_HCRH_W {
        SW_HCRH_W { w: self }
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&mut self) -> SW_HCRL_W {
        SW_HCRL_W { w: self }
    }
}
