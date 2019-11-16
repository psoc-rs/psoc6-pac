#[doc = "Reader of register CFG_IN"]
pub type R = crate::R<u32, super::CFG_IN>;
#[doc = "Writer for register CFG_IN"]
pub type W = crate::W<u32, super::CFG_IN>;
#[doc = "Register CFG_IN `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_IN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configures the pin 0 input buffer mode (trip points and hysteresis)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VTRIP_SEL0_0_A {
    #[doc = "0: Input buffer compatible with CMOS and I2C interfaces"]
    CMOS = 0,
    #[doc = "1: Input buffer compatible with TTL and MediaLB interfaces"]
    TTL = 1,
}
impl From<VTRIP_SEL0_0_A> for bool {
    #[inline(always)]
    fn from(variant: VTRIP_SEL0_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VTRIP_SEL0_0`"]
pub type VTRIP_SEL0_0_R = crate::R<bool, VTRIP_SEL0_0_A>;
impl VTRIP_SEL0_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VTRIP_SEL0_0_A {
        match self.bits {
            false => VTRIP_SEL0_0_A::CMOS,
            true => VTRIP_SEL0_0_A::TTL,
        }
    }
    #[doc = "Checks if the value of the field is `CMOS`"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        *self == VTRIP_SEL0_0_A::CMOS
    }
    #[doc = "Checks if the value of the field is `TTL`"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        *self == VTRIP_SEL0_0_A::TTL
    }
}
#[doc = "Write proxy for field `VTRIP_SEL0_0`"]
pub struct VTRIP_SEL0_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL0_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VTRIP_SEL0_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input buffer compatible with CMOS and I2C interfaces"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::CMOS)
    }
    #[doc = "Input buffer compatible with TTL and MediaLB interfaces"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut W {
        self.variant(VTRIP_SEL0_0_A::TTL)
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
#[doc = "Reader of field `VTRIP_SEL1_0`"]
pub type VTRIP_SEL1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL1_0`"]
pub struct VTRIP_SEL1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL1_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL2_0`"]
pub type VTRIP_SEL2_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL2_0`"]
pub struct VTRIP_SEL2_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL2_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL3_0`"]
pub type VTRIP_SEL3_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL3_0`"]
pub struct VTRIP_SEL3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL3_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL4_0`"]
pub type VTRIP_SEL4_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL4_0`"]
pub struct VTRIP_SEL4_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL4_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL5_0`"]
pub type VTRIP_SEL5_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL5_0`"]
pub struct VTRIP_SEL5_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL5_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL6_0`"]
pub type VTRIP_SEL6_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL6_0`"]
pub struct VTRIP_SEL6_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL6_0_W<'a> {
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
#[doc = "Reader of field `VTRIP_SEL7_0`"]
pub type VTRIP_SEL7_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTRIP_SEL7_0`"]
pub struct VTRIP_SEL7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIP_SEL7_0_W<'a> {
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
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&self) -> VTRIP_SEL0_0_R {
        VTRIP_SEL0_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&self) -> VTRIP_SEL1_0_R {
        VTRIP_SEL1_0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&self) -> VTRIP_SEL2_0_R {
        VTRIP_SEL2_0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&self) -> VTRIP_SEL3_0_R {
        VTRIP_SEL3_0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&self) -> VTRIP_SEL4_0_R {
        VTRIP_SEL4_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&self) -> VTRIP_SEL5_0_R {
        VTRIP_SEL5_0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&self) -> VTRIP_SEL6_0_R {
        VTRIP_SEL6_0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&self) -> VTRIP_SEL7_0_R {
        VTRIP_SEL7_0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&mut self) -> VTRIP_SEL0_0_W {
        VTRIP_SEL0_0_W { w: self }
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&mut self) -> VTRIP_SEL1_0_W {
        VTRIP_SEL1_0_W { w: self }
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&mut self) -> VTRIP_SEL2_0_W {
        VTRIP_SEL2_0_W { w: self }
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&mut self) -> VTRIP_SEL3_0_W {
        VTRIP_SEL3_0_W { w: self }
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&mut self) -> VTRIP_SEL4_0_W {
        VTRIP_SEL4_0_W { w: self }
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&mut self) -> VTRIP_SEL5_0_W {
        VTRIP_SEL5_0_W { w: self }
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&mut self) -> VTRIP_SEL6_0_W {
        VTRIP_SEL6_0_W { w: self }
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&mut self) -> VTRIP_SEL7_0_W {
        VTRIP_SEL7_0_W { w: self }
    }
}
