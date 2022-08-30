#[doc = "Register `SRSS_INTR_CFG` reader"]
pub struct R(crate::R<SRSS_INTR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_INTR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_INTR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_INTR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSS_INTR_CFG` writer"]
pub struct W(crate::W<SRSS_INTR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSS_INTR_CFG_SPEC>;
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
impl From<crate::W<SRSS_INTR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSS_INTR_CFG_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `HVLVD1_EDGE_SEL` reader - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type HVLVD1_EDGE_SEL_R = crate::FieldReader<u8, HVLVD1_EDGE_SEL_A>;
impl HVLVD1_EDGE_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `HVLVD1_EDGE_SEL` writer - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type HVLVD1_EDGE_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SRSS_INTR_CFG_SPEC, u8, HVLVD1_EDGE_SEL_A, 2, O>;
impl<'a, const O: u8> HVLVD1_EDGE_SEL_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&self) -> HVLVD1_EDGE_SEL_R {
        HVLVD1_EDGE_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&mut self) -> HVLVD1_EDGE_SEL_W<0> {
        HVLVD1_EDGE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRSS Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_cfg](index.html) module"]
pub struct SRSS_INTR_CFG_SPEC;
impl crate::RegisterSpec for SRSS_INTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_intr_cfg::R](R) reader structure"]
impl crate::Readable for SRSS_INTR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srss_intr_cfg::W](W) writer structure"]
impl crate::Writable for SRSS_INTR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSS_INTR_CFG to value 0"]
impl crate::Resettable for SRSS_INTR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
