#[doc = "Register `MCWDT_INTR_SET` reader"]
pub struct R(crate::R<MCWDT_INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_INTR_SET` writer"]
pub struct W(crate::W<MCWDT_INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_INTR_SET_SPEC>;
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
impl From<crate::W<MCWDT_INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCWDT_INT0` reader - Set interrupt for MCWDT_INT0"]
pub type MCWDT_INT0_R = crate::BitReader<bool>;
#[doc = "Field `MCWDT_INT0` writer - Set interrupt for MCWDT_INT0"]
pub type MCWDT_INT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCWDT_INTR_SET_SPEC, bool, O>;
#[doc = "Field `MCWDT_INT1` reader - Set interrupt for MCWDT_INT1"]
pub type MCWDT_INT1_R = crate::BitReader<bool>;
#[doc = "Field `MCWDT_INT1` writer - Set interrupt for MCWDT_INT1"]
pub type MCWDT_INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCWDT_INTR_SET_SPEC, bool, O>;
#[doc = "Field `MCWDT_INT2` reader - Set interrupt for MCWDT_INT2"]
pub type MCWDT_INT2_R = crate::BitReader<bool>;
#[doc = "Field `MCWDT_INT2` writer - Set interrupt for MCWDT_INT2"]
pub type MCWDT_INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCWDT_INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> MCWDT_INT0_R {
        MCWDT_INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> MCWDT_INT1_R {
        MCWDT_INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> MCWDT_INT2_R {
        MCWDT_INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set interrupt for MCWDT_INT0"]
    #[inline(always)]
    pub fn mcwdt_int0(&mut self) -> MCWDT_INT0_W<0> {
        MCWDT_INT0_W::new(self)
    }
    #[doc = "Bit 1 - Set interrupt for MCWDT_INT1"]
    #[inline(always)]
    pub fn mcwdt_int1(&mut self) -> MCWDT_INT1_W<1> {
        MCWDT_INT1_W::new(self)
    }
    #[doc = "Bit 2 - Set interrupt for MCWDT_INT2"]
    #[inline(always)]
    pub fn mcwdt_int2(&mut self) -> MCWDT_INT2_W<2> {
        MCWDT_INT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_intr_set](index.html) module"]
pub struct MCWDT_INTR_SET_SPEC;
impl crate::RegisterSpec for MCWDT_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_intr_set::R](R) reader structure"]
impl crate::Readable for MCWDT_INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_intr_set::W](W) writer structure"]
impl crate::Writable for MCWDT_INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_INTR_SET to value 0"]
impl crate::Resettable for MCWDT_INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
