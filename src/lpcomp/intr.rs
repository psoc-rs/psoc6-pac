#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0` reader - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `COMP0` writer - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
pub type COMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `COMP1` reader - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type COMP1_R = crate::BitReader<bool>;
#[doc = "Field `COMP1` writer - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
pub type COMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Interrupt: hardware sets this interrupt when comparator 0 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W<0> {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 Interrupt: hardware sets this interrupt when comparator 1 triggers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W<1> {
        COMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
