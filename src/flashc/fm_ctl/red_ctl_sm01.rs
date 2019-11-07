#[doc = "Reader of register RED_CTL_SM01"]
pub type R = crate::R<u32, super::RED_CTL_SM01>;
#[doc = "Writer for register RED_CTL_SM01"]
pub type W = crate::W<u32, super::RED_CTL_SM01>;
#[doc = "Register RED_CTL_SM01 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL_SM01 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RED_ADDR_SM0`"]
pub type RED_ADDR_SM0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_SM0`"]
pub struct RED_ADDR_SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_SM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_SM0`"]
pub type RED_EN_SM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_SM0`"]
pub struct RED_EN_SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_SM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RED_ADDR_SM1`"]
pub type RED_ADDR_SM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_SM1`"]
pub struct RED_ADDR_SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_SM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_SM1`"]
pub type RED_EN_SM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_SM1`"]
pub struct RED_EN_SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_SM1_W<'a> {
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
#[doc = "Reader of field `TRKD`"]
pub type TRKD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRKD`"]
pub struct TRKD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRKD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `R_GRANT_EN`"]
pub type R_GRANT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R_GRANT_EN`"]
pub struct R_GRANT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> R_GRANT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&self) -> RED_ADDR_SM0_R {
        RED_ADDR_SM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&self) -> RED_EN_SM0_R {
        RED_EN_SM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&self) -> RED_ADDR_SM1_R {
        RED_ADDR_SM1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&self) -> RED_EN_SM1_R {
        RED_EN_SM1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Sense Amp Control tracking delay"]
    #[inline(always)]
    pub fn trkd(&self) -> TRKD_R {
        TRKD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en(&self) -> R_GRANT_EN_R {
        R_GRANT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&mut self) -> RED_ADDR_SM0_W {
        RED_ADDR_SM0_W { w: self }
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&mut self) -> RED_EN_SM0_W {
        RED_EN_SM0_W { w: self }
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&mut self) -> RED_ADDR_SM1_W {
        RED_ADDR_SM1_W { w: self }
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&mut self) -> RED_EN_SM1_W {
        RED_EN_SM1_W { w: self }
    }
    #[doc = "Bit 30 - Sense Amp Control tracking delay"]
    #[inline(always)]
    pub fn trkd(&mut self) -> TRKD_W {
        TRKD_W { w: self }
    }
    #[doc = "Bit 31 - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en(&mut self) -> R_GRANT_EN_W {
        R_GRANT_EN_W { w: self }
    }
}
