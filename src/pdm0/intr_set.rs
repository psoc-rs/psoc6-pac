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
#[doc = "Field `RX_TRIGGER` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `RX_TRIGGER` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_NOT_EMPTY` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RX_NOT_EMPTY` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_NOT_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
#[doc = "Field `RX_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `RX_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RX_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RX_TRIGGER_R {
        RX_TRIGGER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RX_NOT_EMPTY_R {
        RX_NOT_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RX_OVERFLOW_R {
        RX_OVERFLOW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RX_UNDERFLOW_R {
        RX_UNDERFLOW_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_trigger(&mut self) -> RX_TRIGGER_W<16> {
        RX_TRIGGER_W::new(self)
    }
    #[doc = "Bit 18 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_not_empty(&mut self) -> RX_NOT_EMPTY_W<18> {
        RX_NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 21 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RX_OVERFLOW_W<21> {
        RX_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 22 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_underflow(&mut self) -> RX_UNDERFLOW_W<22> {
        RX_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
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
