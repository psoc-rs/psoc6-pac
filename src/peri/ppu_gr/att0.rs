#[doc = "Reader of register ATT0"]
pub type R = crate::R<u32, super::ATT0>;
#[doc = "Writer for register ATT0"]
pub type W = crate::W<u32, super::ATT0>;
#[doc = "Register ATT0 `reset()`'s with value 0x0124"]
impl crate::ResetValue for super::ATT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0124
    }
}
#[doc = "Reader of field `UR`"]
pub type UR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR`"]
pub struct UR_W<'a> {
    w: &'a mut W,
}
impl<'a> UR_W<'a> {
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
#[doc = "Reader of field `UW`"]
pub type UW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UW`"]
pub struct UW_W<'a> {
    w: &'a mut W,
}
impl<'a> UW_W<'a> {
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
#[doc = "Reader of field `UX`"]
pub type UX_R = crate::R<bool, bool>;
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
#[doc = "Reader of field `PW`"]
pub type PW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PW`"]
pub struct PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_W<'a> {
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
#[doc = "Reader of field `PX`"]
pub type PX_R = crate::R<bool, bool>;
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NS`"]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
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
#[doc = "Reader of field `PC_MASK_0`"]
pub type PC_MASK_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC_MASK_15_TO_1`"]
pub type PC_MASK_15_TO_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PC_MASK_15_TO_1`"]
pub struct PC_MASK_15_TO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MASK_15_TO_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 9)) | (((value as u32) & 0x7fff) << 9);
        self.w
    }
}
#[doc = "Reader of field `REGION_SIZE`"]
pub type REGION_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PC_MATCH`"]
pub type PC_MATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC_MATCH`"]
pub struct PC_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MATCH_W<'a> {
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
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bit 0 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&self) -> UW_R {
        UW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn ux(&self) -> UX_R {
        UX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - See corresponding field for PPU structure with programmable address. Note that this register is constant '1'; i.e. user execute accesses are ALWAYS allowed."]
    #[inline(always)]
    pub fn px(&self) -> PX_R {
        PX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - See corresponding field for PPU structure with programmable address. Note: this field is read-only. Its value is chip specific."]
    #[inline(always)]
    pub fn region_size(&self) -> REGION_SIZE_R {
        REGION_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&self) -> PC_MATCH_R {
        PC_MATCH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W {
        UR_W { w: self }
    }
    #[doc = "Bit 1 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn uw(&mut self) -> UW_W {
        UW_W { w: self }
    }
    #[doc = "Bit 3 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 4 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W { w: self }
    }
    #[doc = "Bit 6 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bits 9:23 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W {
        PC_MASK_15_TO_1_W { w: self }
    }
    #[doc = "Bit 30 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn pc_match(&mut self) -> PC_MATCH_W {
        PC_MATCH_W { w: self }
    }
    #[doc = "Bit 31 - See corresponding field for PPU structure with programmable address."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
