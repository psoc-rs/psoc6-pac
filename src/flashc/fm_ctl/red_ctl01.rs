#[doc = "Reader of register RED_CTL01"]
pub type R = crate::R<u32, super::RED_CTL01>;
#[doc = "Writer for register RED_CTL01"]
pub type W = crate::W<u32, super::RED_CTL01>;
#[doc = "Register RED_CTL01 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL01 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RED_ADDR_0`"]
pub type RED_ADDR_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_0`"]
pub struct RED_ADDR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_0`"]
pub type RED_EN_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_0`"]
pub struct RED_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_0_W<'a> {
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
#[doc = "Reader of field `RED_ADDR_1`"]
pub type RED_ADDR_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_1`"]
pub struct RED_ADDR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_1`"]
pub type RED_EN_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_1`"]
pub struct RED_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_1_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&self) -> RED_ADDR_0_R {
        RED_ADDR_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - '1': Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&self) -> RED_EN_0_R {
        RED_EN_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&self) -> RED_ADDR_1_R {
        RED_ADDR_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - '1': Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&self) -> RED_EN_1_R {
        RED_EN_1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 0"]
    #[inline(always)]
    pub fn red_addr_0(&mut self) -> RED_ADDR_0_W {
        RED_ADDR_0_W { w: self }
    }
    #[doc = "Bit 8 - '1': Redundancy Enable for Sector 0"]
    #[inline(always)]
    pub fn red_en_0(&mut self) -> RED_EN_0_W {
        RED_EN_0_W { w: self }
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 1"]
    #[inline(always)]
    pub fn red_addr_1(&mut self) -> RED_ADDR_1_W {
        RED_ADDR_1_W { w: self }
    }
    #[doc = "Bit 24 - '1': Redundancy Enable for Sector 1"]
    #[inline(always)]
    pub fn red_en_1(&mut self) -> RED_EN_1_W {
        RED_EN_1_W { w: self }
    }
}
