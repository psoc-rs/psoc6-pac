#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB_LL_DONE` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RCB_LL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RCB_LL_DONE` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RCB_LL_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `SINGLE_WRITE_DONE` reader - N/A"]
pub type SINGLE_WRITE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE_WRITE_DONE` writer - N/A"]
pub type SINGLE_WRITE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `SINGLE_READ_DONE` reader - N/A"]
pub type SINGLE_READ_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE_READ_DONE` writer - N/A"]
pub type SINGLE_READ_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcb_ll_done(&self) -> RCB_LL_DONE_R {
        RCB_LL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&self) -> SINGLE_WRITE_DONE_R {
        SINGLE_WRITE_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&self) -> SINGLE_READ_DONE_R {
        SINGLE_READ_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rcb_ll_done(&mut self) -> RCB_LL_DONE_W<0> {
        RCB_LL_DONE_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn single_write_done(&mut self) -> SINGLE_WRITE_DONE_W<2> {
        SINGLE_WRITE_DONE_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn single_read_done(&mut self) -> SINGLE_READ_DONE_W<3> {
        SINGLE_READ_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
