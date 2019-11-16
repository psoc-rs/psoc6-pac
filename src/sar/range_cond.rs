#[doc = "Reader of register RANGE_COND"]
pub type R = crate::R<u32, super::RANGE_COND>;
#[doc = "Writer for register RANGE_COND"]
pub type W = crate::W<u32, super::RANGE_COND>;
#[doc = "Register RANGE_COND `reset()`'s with value 0"]
impl crate::ResetValue for super::RANGE_COND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Range condition select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_COND_A {
    #[doc = "0: result < RANGE_LOW"]
    BELOW = 0,
    #[doc = "1: RANGE_LOW <= result < RANGE_HIGH"]
    INSIDE = 1,
    #[doc = "2: RANGE_HIGH <= result"]
    ABOVE = 2,
    #[doc = "3: result < RANGE_LOW || RANGE_HIGH <= result"]
    OUTSIDE = 3,
}
impl From<RANGE_COND_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_COND_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RANGE_COND`"]
pub type RANGE_COND_R = crate::R<u8, RANGE_COND_A>;
impl RANGE_COND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGE_COND_A {
        match self.bits {
            0 => RANGE_COND_A::BELOW,
            1 => RANGE_COND_A::INSIDE,
            2 => RANGE_COND_A::ABOVE,
            3 => RANGE_COND_A::OUTSIDE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RANGE_COND_A::BELOW
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == RANGE_COND_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RANGE_COND_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == RANGE_COND_A::OUTSIDE
    }
}
#[doc = "Write proxy for field `RANGE_COND`"]
pub struct RANGE_COND_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_COND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_COND_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "result < RANGE_LOW"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(RANGE_COND_A::BELOW)
    }
    #[doc = "RANGE_LOW <= result < RANGE_HIGH"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(RANGE_COND_A::INSIDE)
    }
    #[doc = "RANGE_HIGH <= result"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(RANGE_COND_A::ABOVE)
    }
    #[doc = "result < RANGE_LOW || RANGE_HIGH <= result"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(RANGE_COND_A::OUTSIDE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&self) -> RANGE_COND_R {
        RANGE_COND_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&mut self) -> RANGE_COND_W {
        RANGE_COND_W { w: self }
    }
}
