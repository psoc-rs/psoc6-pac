#[doc = "Reader of register CFG_OUT"]
pub type R = crate::R<u32, super::CFG_OUT>;
#[doc = "Writer for register CFG_OUT"]
pub type W = crate::W<u32, super::CFG_OUT>;
#[doc = "Register CFG_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLOW0`"]
pub type SLOW0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW0`"]
pub struct SLOW0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW0_W<'a> {
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
#[doc = "Reader of field `SLOW1`"]
pub type SLOW1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW1`"]
pub struct SLOW1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW1_W<'a> {
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
#[doc = "Reader of field `SLOW2`"]
pub type SLOW2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW2`"]
pub struct SLOW2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW2_W<'a> {
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
#[doc = "Reader of field `SLOW3`"]
pub type SLOW3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW3`"]
pub struct SLOW3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW3_W<'a> {
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
#[doc = "Reader of field `SLOW4`"]
pub type SLOW4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW4`"]
pub struct SLOW4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW4_W<'a> {
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
#[doc = "Reader of field `SLOW5`"]
pub type SLOW5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW5`"]
pub struct SLOW5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW5_W<'a> {
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
#[doc = "Reader of field `SLOW6`"]
pub type SLOW6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW6`"]
pub struct SLOW6_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW6_W<'a> {
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
#[doc = "Reader of field `SLOW7`"]
pub type SLOW7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLOW7`"]
pub struct SLOW7_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW7_W<'a> {
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
#[doc = "Sets the GPIO drive strength for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_SEL0_A {
    #[doc = "0: Full drive strength: GPIO drives current at its max rated spec."]
    FULL_DRIVE = 0,
    #[doc = "1: 1/2 drive strength: GPIO drives current at 1/2 of its max rated spec"]
    ONE_HALF_DRIVE = 1,
    #[doc = "2: 1/4 drive strength: GPIO drives current at 1/4 of its max rated spec."]
    ONE_QUARTER_DRIVE = 2,
    #[doc = "3: 1/8 drive strength: GPIO drives current at 1/8 of its max rated spec."]
    ONE_EIGHTH_DRIVE = 3,
}
impl From<DRIVE_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVE_SEL0`"]
pub type DRIVE_SEL0_R = crate::R<u8, DRIVE_SEL0_A>;
impl DRIVE_SEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_SEL0_A {
        match self.bits {
            0 => DRIVE_SEL0_A::FULL_DRIVE,
            1 => DRIVE_SEL0_A::ONE_HALF_DRIVE,
            2 => DRIVE_SEL0_A::ONE_QUARTER_DRIVE,
            3 => DRIVE_SEL0_A::ONE_EIGHTH_DRIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_DRIVE`"]
    #[inline(always)]
    pub fn is_full_drive(&self) -> bool {
        *self == DRIVE_SEL0_A::FULL_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_HALF_DRIVE`"]
    #[inline(always)]
    pub fn is_one_half_drive(&self) -> bool {
        *self == DRIVE_SEL0_A::ONE_HALF_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_QUARTER_DRIVE`"]
    #[inline(always)]
    pub fn is_one_quarter_drive(&self) -> bool {
        *self == DRIVE_SEL0_A::ONE_QUARTER_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_EIGHTH_DRIVE`"]
    #[inline(always)]
    pub fn is_one_eighth_drive(&self) -> bool {
        *self == DRIVE_SEL0_A::ONE_EIGHTH_DRIVE
    }
}
#[doc = "Write proxy for field `DRIVE_SEL0`"]
pub struct DRIVE_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_SEL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Full drive strength: GPIO drives current at its max rated spec."]
    #[inline(always)]
    pub fn full_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::FULL_DRIVE)
    }
    #[doc = "1/2 drive strength: GPIO drives current at 1/2 of its max rated spec"]
    #[inline(always)]
    pub fn one_half_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_HALF_DRIVE)
    }
    #[doc = "1/4 drive strength: GPIO drives current at 1/4 of its max rated spec."]
    #[inline(always)]
    pub fn one_quarter_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_QUARTER_DRIVE)
    }
    #[doc = "1/8 drive strength: GPIO drives current at 1/8 of its max rated spec."]
    #[inline(always)]
    pub fn one_eighth_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_EIGHTH_DRIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL1`"]
pub type DRIVE_SEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL1`"]
pub struct DRIVE_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL2`"]
pub type DRIVE_SEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL2`"]
pub struct DRIVE_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL3`"]
pub type DRIVE_SEL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL3`"]
pub struct DRIVE_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL4`"]
pub type DRIVE_SEL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL4`"]
pub struct DRIVE_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL5`"]
pub type DRIVE_SEL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL5`"]
pub struct DRIVE_SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL6`"]
pub type DRIVE_SEL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL6`"]
pub struct DRIVE_SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_SEL7`"]
pub type DRIVE_SEL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_SEL7`"]
pub struct DRIVE_SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&self) -> SLOW0_R {
        SLOW0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&self) -> SLOW1_R {
        SLOW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&self) -> SLOW2_R {
        SLOW2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&self) -> SLOW3_R {
        SLOW3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&self) -> SLOW4_R {
        SLOW4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&self) -> SLOW5_R {
        SLOW5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&self) -> SLOW6_R {
        SLOW6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&self) -> SLOW7_R {
        SLOW7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&self) -> DRIVE_SEL0_R {
        DRIVE_SEL0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&self) -> DRIVE_SEL1_R {
        DRIVE_SEL1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&self) -> DRIVE_SEL2_R {
        DRIVE_SEL2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&self) -> DRIVE_SEL3_R {
        DRIVE_SEL3_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&self) -> DRIVE_SEL4_R {
        DRIVE_SEL4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&self) -> DRIVE_SEL5_R {
        DRIVE_SEL5_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&self) -> DRIVE_SEL6_R {
        DRIVE_SEL6_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&self) -> DRIVE_SEL7_R {
        DRIVE_SEL7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&mut self) -> SLOW0_W {
        SLOW0_W { w: self }
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&mut self) -> SLOW1_W {
        SLOW1_W { w: self }
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&mut self) -> SLOW2_W {
        SLOW2_W { w: self }
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&mut self) -> SLOW3_W {
        SLOW3_W { w: self }
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&mut self) -> SLOW4_W {
        SLOW4_W { w: self }
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&mut self) -> SLOW5_W {
        SLOW5_W { w: self }
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&mut self) -> SLOW6_W {
        SLOW6_W { w: self }
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&mut self) -> SLOW7_W {
        SLOW7_W { w: self }
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&mut self) -> DRIVE_SEL0_W {
        DRIVE_SEL0_W { w: self }
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&mut self) -> DRIVE_SEL1_W {
        DRIVE_SEL1_W { w: self }
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&mut self) -> DRIVE_SEL2_W {
        DRIVE_SEL2_W { w: self }
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&mut self) -> DRIVE_SEL3_W {
        DRIVE_SEL3_W { w: self }
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&mut self) -> DRIVE_SEL4_W {
        DRIVE_SEL4_W { w: self }
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&mut self) -> DRIVE_SEL5_W {
        DRIVE_SEL5_W { w: self }
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&mut self) -> DRIVE_SEL6_W {
        DRIVE_SEL6_W { w: self }
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&mut self) -> DRIVE_SEL7_W {
        DRIVE_SEL7_W { w: self }
    }
}
