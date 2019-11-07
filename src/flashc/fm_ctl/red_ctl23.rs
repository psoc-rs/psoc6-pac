#[doc = "Reader of register RED_CTL23"]
pub type R = crate::R<u32, super::RED_CTL23>;
#[doc = "Writer for register RED_CTL23"]
pub type W = crate::W<u32, super::RED_CTL23>;
#[doc = "Register RED_CTL23 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RED_ADDR_2`"]
pub type RED_ADDR_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_2`"]
pub struct RED_ADDR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_2`"]
pub type RED_EN_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_2`"]
pub struct RED_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_2_W<'a> {
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
#[doc = "Reader of field `RED_ADDR_3`"]
pub type RED_ADDR_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED_ADDR_3`"]
pub struct RED_ADDR_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_ADDR_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RED_EN_3`"]
pub type RED_EN_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RED_EN_3`"]
pub struct RED_EN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_EN_3_W<'a> {
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
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn red_addr_2(&self) -> RED_ADDR_2_R {
        RED_ADDR_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1': Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn red_en_2(&self) -> RED_EN_2_R {
        RED_EN_2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn red_addr_3(&self) -> RED_ADDR_3_R {
        RED_ADDR_3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1': Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn red_en_3(&self) -> RED_EN_3_R {
        RED_EN_3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Sector 2"]
    #[inline(always)]
    pub fn red_addr_2(&mut self) -> RED_ADDR_2_W {
        RED_ADDR_2_W { w: self }
    }
    #[doc = "Bit 8 - 1': Redundancy Enable for Sector 2"]
    #[inline(always)]
    pub fn red_en_2(&mut self) -> RED_EN_2_W {
        RED_EN_2_W { w: self }
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Sector 3"]
    #[inline(always)]
    pub fn red_addr_3(&mut self) -> RED_ADDR_3_W {
        RED_ADDR_3_W { w: self }
    }
    #[doc = "Bit 24 - 1': Redundancy Enable for Sector 3"]
    #[inline(always)]
    pub fn red_en_3(&mut self) -> RED_EN_3_W {
        RED_EN_3_W { w: self }
    }
}
