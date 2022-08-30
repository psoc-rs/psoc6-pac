#[doc = "Register `INTR_CFG` reader"]
pub struct R(crate::R<INTR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_CFG` writer"]
pub struct W(crate::W<INTR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CFG_SPEC>;
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
impl From<crate::W<INTR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CFG_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `EDGE0_SEL` reader - Sets which edge will trigger an IRQ for IO pin 0"]
pub type EDGE0_SEL_R = crate::FieldReader<u8, EDGE0_SEL_A>;
impl EDGE0_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `EDGE0_SEL` writer - Sets which edge will trigger an IRQ for IO pin 0"]
pub type EDGE0_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INTR_CFG_SPEC, u8, EDGE0_SEL_A, 2, O>;
impl<'a, const O: u8> EDGE0_SEL_W<'a, O> {
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
}
#[doc = "Field `EDGE1_SEL` reader - Sets which edge will trigger an IRQ for IO pin 1"]
pub type EDGE1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE1_SEL` writer - Sets which edge will trigger an IRQ for IO pin 1"]
pub type EDGE1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE2_SEL` reader - Sets which edge will trigger an IRQ for IO pin 2"]
pub type EDGE2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE2_SEL` writer - Sets which edge will trigger an IRQ for IO pin 2"]
pub type EDGE2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE3_SEL` reader - Sets which edge will trigger an IRQ for IO pin 3"]
pub type EDGE3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE3_SEL` writer - Sets which edge will trigger an IRQ for IO pin 3"]
pub type EDGE3_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE4_SEL` reader - Sets which edge will trigger an IRQ for IO pin 4"]
pub type EDGE4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE4_SEL` writer - Sets which edge will trigger an IRQ for IO pin 4"]
pub type EDGE4_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE5_SEL` reader - Sets which edge will trigger an IRQ for IO pin 5"]
pub type EDGE5_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE5_SEL` writer - Sets which edge will trigger an IRQ for IO pin 5"]
pub type EDGE5_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE6_SEL` reader - Sets which edge will trigger an IRQ for IO pin 6"]
pub type EDGE6_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE6_SEL` writer - Sets which edge will trigger an IRQ for IO pin 6"]
pub type EDGE6_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EDGE7_SEL` reader - Sets which edge will trigger an IRQ for IO pin 7"]
pub type EDGE7_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGE7_SEL` writer - Sets which edge will trigger an IRQ for IO pin 7"]
pub type EDGE7_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 2, O>;
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
#[doc = "Field `FLT_EDGE_SEL` reader - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_SEL_R = crate::FieldReader<u8, FLT_EDGE_SEL_A>;
impl FLT_EDGE_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `FLT_EDGE_SEL` writer - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INTR_CFG_SPEC, u8, FLT_EDGE_SEL_A, 2, O>;
impl<'a, const O: u8> FLT_EDGE_SEL_W<'a, O> {
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
}
#[doc = "Field `FLT_SEL` reader - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FLT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLT_SEL` writer - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FLT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INTR_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&self) -> EDGE0_SEL_R {
        EDGE0_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&self) -> EDGE1_SEL_R {
        EDGE1_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&self) -> EDGE2_SEL_R {
        EDGE2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&self) -> EDGE3_SEL_R {
        EDGE3_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&self) -> EDGE4_SEL_R {
        EDGE4_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&self) -> EDGE5_SEL_R {
        EDGE5_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&self) -> EDGE6_SEL_R {
        EDGE6_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&self) -> EDGE7_SEL_R {
        EDGE7_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FLT_EDGE_SEL_R {
        FLT_EDGE_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&self) -> FLT_SEL_R {
        FLT_SEL_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&mut self) -> EDGE0_SEL_W<0> {
        EDGE0_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&mut self) -> EDGE1_SEL_W<2> {
        EDGE1_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&mut self) -> EDGE2_SEL_W<4> {
        EDGE2_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&mut self) -> EDGE3_SEL_W<6> {
        EDGE3_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&mut self) -> EDGE4_SEL_W<8> {
        EDGE4_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&mut self) -> EDGE5_SEL_W<10> {
        EDGE5_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&mut self) -> EDGE6_SEL_W<12> {
        EDGE6_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&mut self) -> EDGE7_SEL_W<14> {
        EDGE7_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&mut self) -> FLT_EDGE_SEL_W<16> {
        FLT_EDGE_SEL_W::new(self)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&mut self) -> FLT_SEL_W<18> {
        FLT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cfg](index.html) module"]
pub struct INTR_CFG_SPEC;
impl crate::RegisterSpec for INTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cfg::R](R) reader structure"]
impl crate::Readable for INTR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_cfg::W](W) writer structure"]
impl crate::Writable for INTR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_CFG to value 0"]
impl crate::Resettable for INTR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
