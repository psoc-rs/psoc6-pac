#[doc = "Reader of register INTR_MASK"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register INTR_MASK"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EDGE0`"]
pub type EDGE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE0`"]
pub struct EDGE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE0_W<'a> {
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
#[doc = "Reader of field `EDGE1`"]
pub type EDGE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE1`"]
pub struct EDGE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE1_W<'a> {
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
#[doc = "Reader of field `EDGE2`"]
pub type EDGE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE2`"]
pub struct EDGE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE2_W<'a> {
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
#[doc = "Reader of field `EDGE3`"]
pub type EDGE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE3`"]
pub struct EDGE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE3_W<'a> {
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
#[doc = "Reader of field `EDGE4`"]
pub type EDGE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE4`"]
pub struct EDGE4_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE4_W<'a> {
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
#[doc = "Reader of field `EDGE5`"]
pub type EDGE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE5`"]
pub struct EDGE5_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EDGE6`"]
pub type EDGE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE6`"]
pub struct EDGE6_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EDGE7`"]
pub type EDGE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE7`"]
pub struct EDGE7_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FLT_EDGE`"]
pub type FLT_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLT_EDGE`"]
pub struct FLT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_EDGE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub fn edge0(&mut self) -> EDGE0_W {
        EDGE0_W { w: self }
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&mut self) -> EDGE1_W {
        EDGE1_W { w: self }
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&mut self) -> EDGE2_W {
        EDGE2_W { w: self }
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&mut self) -> EDGE3_W {
        EDGE3_W { w: self }
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&mut self) -> EDGE4_W {
        EDGE4_W { w: self }
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&mut self) -> EDGE5_W {
        EDGE5_W { w: self }
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&mut self) -> EDGE6_W {
        EDGE6_W { w: self }
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&mut self) -> EDGE7_W {
        EDGE7_W { w: self }
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&mut self) -> FLT_EDGE_W {
        FLT_EDGE_W { w: self }
    }
}
