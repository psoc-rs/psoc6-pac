#[doc = "Reader of register TRIM_ROM_CTL"]
pub type R = crate::R<u32, super::TRIM_ROM_CTL>;
#[doc = "Writer for register TRIM_ROM_CTL"]
pub type W = crate::W<u32, super::TRIM_ROM_CTL>;
#[doc = "Register TRIM_ROM_CTL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::TRIM_ROM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RM`"]
pub type RM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RM`"]
pub struct RM_W<'a> {
    w: &'a mut W,
}
impl<'a> RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RME`"]
pub type RME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RME`"]
pub struct RME_W<'a> {
    w: &'a mut W,
}
impl<'a> RME_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin settting."]
    #[inline(always)]
    pub fn rme(&self) -> RME_R {
        RME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W {
        RM_W { w: self }
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin settting."]
    #[inline(always)]
    pub fn rme(&mut self) -> RME_W {
        RME_W { w: self }
    }
}
