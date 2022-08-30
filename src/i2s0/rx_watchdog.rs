#[doc = "Register `RX_WATCHDOG` reader"]
pub struct R(crate::R<RX_WATCHDOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_WATCHDOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_WATCHDOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_WATCHDOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_WATCHDOG` writer"]
pub struct W(crate::W<RX_WATCHDOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_WATCHDOG_SPEC>;
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
impl From<crate::W<RX_WATCHDOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_WATCHDOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WD_COUNTER` reader - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WD_COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WD_COUNTER` writer - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WD_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_WATCHDOG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&self) -> WD_COUNTER_R {
        WD_COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value of the RX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&mut self) -> WD_COUNTER_W<0> {
        WD_COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver watchdog\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_watchdog](index.html) module"]
pub struct RX_WATCHDOG_SPEC;
impl crate::RegisterSpec for RX_WATCHDOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_watchdog::R](R) reader structure"]
impl crate::Readable for RX_WATCHDOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_watchdog::W](W) writer structure"]
impl crate::Writable for RX_WATCHDOG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_WATCHDOG to value 0"]
impl crate::Resettable for RX_WATCHDOG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
