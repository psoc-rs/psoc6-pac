#[doc = "Reader of register INTR_CFG"]
pub type R = crate::R<u32, super::INTR_CFG>;
#[doc = "Writer for register INTR_CFG"]
pub type W = crate::W<u32, super::INTR_CFG>;
#[doc = "Register INTR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets which edge will trigger an IRQ for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGE0_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<EDGE0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGE0_SEL`"]
pub type EDGE0_SEL_R = crate::R<u8, EDGE0_SEL_A>;
impl EDGE0_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE0_SEL_A {
        match self.bits {
            0 => EDGE0_SEL_A::DISABLE,
            1 => EDGE0_SEL_A::RISING,
            2 => EDGE0_SEL_A::FALLING,
            3 => EDGE0_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EDGE0_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGE0_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGE0_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EDGE0_SEL_A::BOTH
    }
}
#[doc = "Write proxy for field `EDGE0_SEL`"]
pub struct EDGE0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE0_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EDGE0_SEL_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `EDGE1_SEL`"]
pub type EDGE1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE1_SEL`"]
pub struct EDGE1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `EDGE2_SEL`"]
pub type EDGE2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE2_SEL`"]
pub struct EDGE2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EDGE3_SEL`"]
pub type EDGE3_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE3_SEL`"]
pub struct EDGE3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EDGE4_SEL`"]
pub type EDGE4_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE4_SEL`"]
pub struct EDGE4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EDGE5_SEL`"]
pub type EDGE5_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE5_SEL`"]
pub struct EDGE5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE5_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `EDGE6_SEL`"]
pub type EDGE6_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE6_SEL`"]
pub struct EDGE6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE6_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `EDGE7_SEL`"]
pub type EDGE7_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE7_SEL`"]
pub struct EDGE7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT_EDGE_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<FLT_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_EDGE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLT_EDGE_SEL`"]
pub type FLT_EDGE_SEL_R = crate::R<u8, FLT_EDGE_SEL_A>;
impl FLT_EDGE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_EDGE_SEL_A {
        match self.bits {
            0 => FLT_EDGE_SEL_A::DISABLE,
            1 => FLT_EDGE_SEL_A::RISING,
            2 => FLT_EDGE_SEL_A::FALLING,
            3 => FLT_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLT_EDGE_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FLT_EDGE_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FLT_EDGE_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == FLT_EDGE_SEL_A::BOTH
    }
}
#[doc = "Write proxy for field `FLT_EDGE_SEL`"]
pub struct FLT_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_EDGE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_EDGE_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(FLT_EDGE_SEL_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `FLT_SEL`"]
pub type FLT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT_SEL`"]
pub struct FLT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&self) -> EDGE0_SEL_R {
        EDGE0_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&self) -> EDGE1_SEL_R {
        EDGE1_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&self) -> EDGE2_SEL_R {
        EDGE2_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&self) -> EDGE3_SEL_R {
        EDGE3_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&self) -> EDGE4_SEL_R {
        EDGE4_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&self) -> EDGE5_SEL_R {
        EDGE5_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&self) -> EDGE6_SEL_R {
        EDGE6_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&self) -> EDGE7_SEL_R {
        EDGE7_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FLT_EDGE_SEL_R {
        FLT_EDGE_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&self) -> FLT_SEL_R {
        FLT_SEL_R::new(((self.bits >> 18) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&mut self) -> EDGE0_SEL_W {
        EDGE0_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&mut self) -> EDGE1_SEL_W {
        EDGE1_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&mut self) -> EDGE2_SEL_W {
        EDGE2_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&mut self) -> EDGE3_SEL_W {
        EDGE3_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&mut self) -> EDGE4_SEL_W {
        EDGE4_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&mut self) -> EDGE5_SEL_W {
        EDGE5_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&mut self) -> EDGE6_SEL_W {
        EDGE6_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&mut self) -> EDGE7_SEL_W {
        EDGE7_SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&mut self) -> FLT_EDGE_SEL_W {
        FLT_EDGE_SEL_W { w: self }
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&mut self) -> FLT_SEL_W {
        FLT_SEL_W { w: self }
    }
}
