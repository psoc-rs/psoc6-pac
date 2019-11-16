#[doc = "Reader of register SRSS_INTR_CFG"]
pub type R = crate::R<u32, super::SRSS_INTR_CFG>;
#[doc = "Writer for register SRSS_INTR_CFG"]
pub type W = crate::W<u32, super::SRSS_INTR_CFG>;
#[doc = "Register SRSS_INTR_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SRSS_INTR_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets which edge(s) will trigger an IRQ for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HVLVD1_EDGE_SEL_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<HVLVD1_EDGE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVLVD1_EDGE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HVLVD1_EDGE_SEL`"]
pub type HVLVD1_EDGE_SEL_R = crate::R<u8, HVLVD1_EDGE_SEL_A>;
impl HVLVD1_EDGE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HVLVD1_EDGE_SEL_A {
        match self.bits {
            0 => HVLVD1_EDGE_SEL_A::DISABLE,
            1 => HVLVD1_EDGE_SEL_A::RISING,
            2 => HVLVD1_EDGE_SEL_A::FALLING,
            3 => HVLVD1_EDGE_SEL_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == HVLVD1_EDGE_SEL_A::BOTH
    }
}
#[doc = "Write proxy for field `HVLVD1_EDGE_SEL`"]
pub struct HVLVD1_EDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_EDGE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HVLVD1_EDGE_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HVLVD1_EDGE_SEL_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(HVLVD1_EDGE_SEL_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(HVLVD1_EDGE_SEL_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(HVLVD1_EDGE_SEL_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&self) -> HVLVD1_EDGE_SEL_R {
        HVLVD1_EDGE_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&mut self) -> HVLVD1_EDGE_SEL_W {
        HVLVD1_EDGE_SEL_W { w: self }
    }
}
