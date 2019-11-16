#[doc = "Reader of register CFG_IN_GPIO5V"]
pub type R = crate::R<u32, super::CFG_IN_GPIO5V>;
#[doc = "Writer for register CFG_IN_GPIO5V"]
pub type W = crate::W<u32, super::CFG_IN_GPIO5V>;
#[doc = "Register CFG_IN_GPIO5V `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_IN_GPIO5V {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automative. 1: input buffer is compatible with automative. Use CFG_IN.VTRIP_SEL0_0 fieds set as CMOS only when this bit needs to be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTRIP_SEL0_1_A {
    #[doc = "0: Input buffer not compatible with automotive (elevated Vil) interfaces."]
    DISABLE = 0,
    #[doc = "1: Input buffer compatible with automotive (elevated Vil) interfaces."]
    AUTO = 1,
}
impl From<VTRIP_SEL0_1_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VTRIP_SEL0_1`"]
pub type VTRIP_SEL0_1_R = crate::R<bool, VTRIP_SEL0_1_A>;
impl VTRIP_SEL0_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTRIP_SEL0_1_A {
        match self.bits {
            false => VTRIP_SEL0_1_A::DISABLE,
            true => VTRIP_SEL0_1_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VTRIP_SEL0_1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VTRIP_SEL0_1_A::AUTO
    }
}
#[doc = "Write proxy for field `VTRIP_SEL0_1`"]
pub struct VTRIP_SEL0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIP_SEL0_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input buffer not compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_1_A::DISABLE)
    }
    #[doc = "Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_1_A::AUTO)
    }
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
#[doc = "Reader of field `VTRIP_SEL1_1`"]
pub type VTRIP_SEL1_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL1_1`"]
pub struct VTRIP_SEL1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL1_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL2_1`"]
pub type VTRIP_SEL2_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL2_1`"]
pub struct VTRIP_SEL2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL2_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL3_1`"]
pub type VTRIP_SEL3_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL3_1`"]
pub struct VTRIP_SEL3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL3_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL4_1`"]
pub type VTRIP_SEL4_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL4_1`"]
pub struct VTRIP_SEL4_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL4_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL5_1`"]
pub type VTRIP_SEL5_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL5_1`"]
pub struct VTRIP_SEL5_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL5_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL6_1`"]
pub type VTRIP_SEL6_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL6_1`"]
pub struct VTRIP_SEL6_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL6_1_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL7_1`"]
pub type VTRIP_SEL7_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL7_1`"]
pub struct VTRIP_SEL7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL7_1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automative. 1: input buffer is compatible with automative. Use CFG_IN.VTRIP_SEL0_0 fieds set as CMOS only when this bit needs to be set."]
    #[inline(always)]
    pub fn vtrip_sel0_1(&self) -> VTRIP_SEL0_1_R {
        VTRIP_SEL0_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&self) -> VTRIP_SEL1_1_R {
        VTRIP_SEL1_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&self) -> VTRIP_SEL2_1_R {
        VTRIP_SEL2_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&self) -> VTRIP_SEL3_1_R {
        VTRIP_SEL3_1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&self) -> VTRIP_SEL4_1_R {
        VTRIP_SEL4_1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&self) -> VTRIP_SEL5_1_R {
        VTRIP_SEL5_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&self) -> VTRIP_SEL6_1_R {
        VTRIP_SEL6_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&self) -> VTRIP_SEL7_1_R {
        VTRIP_SEL7_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO5V upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. 0: input buffer is not compatible with automative. 1: input buffer is compatible with automative. Use CFG_IN.VTRIP_SEL0_0 fieds set as CMOS only when this bit needs to be set."]
    #[inline(always)]
    pub fn vtrip_sel0_1(&mut self) -> VTRIP_SEL0_1_W {
        VTRIP_SEL0_1_W { w: self }
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&mut self) -> VTRIP_SEL1_1_W {
        VTRIP_SEL1_1_W { w: self }
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&mut self) -> VTRIP_SEL2_1_W {
        VTRIP_SEL2_1_W { w: self }
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&mut self) -> VTRIP_SEL3_1_W {
        VTRIP_SEL3_1_W { w: self }
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&mut self) -> VTRIP_SEL4_1_W {
        VTRIP_SEL4_1_W { w: self }
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&mut self) -> VTRIP_SEL5_1_W {
        VTRIP_SEL5_1_W { w: self }
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&mut self) -> VTRIP_SEL6_1_W {
        VTRIP_SEL6_1_W { w: self }
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&mut self) -> VTRIP_SEL7_1_W {
        VTRIP_SEL7_1_W { w: self }
    }
}
