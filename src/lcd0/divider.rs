#[doc = "Register `DIVIDER` reader"]
pub struct R(crate::R<DIVIDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVIDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVIDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVIDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIVIDER` writer"]
pub struct W(crate::W<DIVIDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIVIDER_SPEC>;
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
impl From<crate::W<DIVIDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIVIDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBFR_DIV` reader - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SUBFR_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBFR_DIV` writer - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SUBFR_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVIDER_SPEC, u16, u16, 16, O>;
#[doc = "Field `DEAD_DIV` reader - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DEAD_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEAD_DIV` writer - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DEAD_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIVIDER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&self) -> SUBFR_DIV_R {
        SUBFR_DIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&self) -> DEAD_DIV_R {
        DEAD_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&mut self) -> SUBFR_DIV_W<0> {
        SUBFR_DIV_W::new(self)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&mut self) -> DEAD_DIV_W<16> {
        DEAD_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divider](index.html) module"]
pub struct DIVIDER_SPEC;
impl crate::RegisterSpec for DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divider::R](R) reader structure"]
impl crate::Readable for DIVIDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [divider::W](W) writer structure"]
impl crate::Writable for DIVIDER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIVIDER to value 0"]
impl crate::Resettable for DIVIDER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
