#[doc = "Register `RANGE_COND` reader"]
pub struct R(crate::R<RANGE_COND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANGE_COND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANGE_COND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANGE_COND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RANGE_COND` writer"]
pub struct W(crate::W<RANGE_COND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RANGE_COND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RANGE_COND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RANGE_COND_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `RANGE_COND` reader - Range condition select."]
pub type RANGE_COND_R = crate::FieldReader<u8, RANGE_COND_A>;
impl RANGE_COND_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `RANGE_COND` writer - Range condition select."]
pub type RANGE_COND_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RANGE_COND_SPEC, u8, RANGE_COND_A, 2, O>;
impl<'a, const O: u8> RANGE_COND_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&self) -> RANGE_COND_R {
        RANGE_COND_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&mut self) -> RANGE_COND_W<30> {
        RANGE_COND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global range detect mode register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [range_cond](index.html) module"]
pub struct RANGE_COND_SPEC;
impl crate::RegisterSpec for RANGE_COND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [range_cond::R](R) reader structure"]
impl crate::Readable for RANGE_COND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [range_cond::W](W) writer structure"]
impl crate::Writable for RANGE_COND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RANGE_COND to value 0"]
impl crate::Resettable for RANGE_COND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
