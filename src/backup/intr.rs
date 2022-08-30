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
#[doc = "Field `ALARM1` reader - Alarm 1 Interrupt"]
pub type ALARM1_R = crate::BitReader<bool>;
#[doc = "Field `ALARM1` writer - Alarm 1 Interrupt"]
pub type ALARM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `ALARM2` reader - Alarm 2 Interrupt"]
pub type ALARM2_R = crate::BitReader<bool>;
#[doc = "Field `ALARM2` writer - Alarm 2 Interrupt"]
pub type ALARM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CENTURY` reader - Century overflow interrupt"]
pub type CENTURY_R = crate::BitReader<bool>;
#[doc = "Field `CENTURY` writer - Century overflow interrupt"]
pub type CENTURY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Alarm 1 Interrupt"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm 2 Interrupt"]
    #[inline(always)]
    pub fn alarm2(&self) -> ALARM2_R {
        ALARM2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Century overflow interrupt"]
    #[inline(always)]
    pub fn century(&self) -> CENTURY_R {
        CENTURY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm 1 Interrupt"]
    #[inline(always)]
    pub fn alarm1(&mut self) -> ALARM1_W<0> {
        ALARM1_W::new(self)
    }
    #[doc = "Bit 1 - Alarm 2 Interrupt"]
    #[inline(always)]
    pub fn alarm2(&mut self) -> ALARM2_W<1> {
        ALARM2_W::new(self)
    }
    #[doc = "Bit 2 - Century overflow interrupt"]
    #[inline(always)]
    pub fn century(&mut self) -> CENTURY_W<2> {
        CENTURY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
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
