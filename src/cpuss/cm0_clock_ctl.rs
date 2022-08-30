#[doc = "Register `CM0_CLOCK_CTL` reader"]
pub struct R(crate::R<CM0_CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_CLOCK_CTL` writer"]
pub struct W(crate::W<CM0_CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_CLOCK_CTL_SPEC>;
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
impl From<crate::W<CM0_CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_INT_DIV` reader - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type SLOW_INT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOW_INT_DIV` writer - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type SLOW_INT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM0_CLOCK_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `PERI_INT_DIV` reader - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
pub type PERI_INT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERI_INT_DIV` writer - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
pub type PERI_INT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM0_CLOCK_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn slow_int_div(&self) -> SLOW_INT_DIV_R {
        SLOW_INT_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn peri_int_div(&self) -> PERI_INT_DIV_R {
        PERI_INT_DIV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn slow_int_div(&mut self) -> SLOW_INT_DIV_W<8> {
        SLOW_INT_DIV_W::new(self)
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi <= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn peri_int_div(&mut self) -> PERI_INT_DIV_W<24> {
        PERI_INT_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_clock_ctl](index.html) module"]
pub struct CM0_CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CM0_CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_clock_ctl::R](R) reader structure"]
impl crate::Readable for CM0_CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_clock_ctl::W](W) writer structure"]
impl crate::Writable for CM0_CLOCK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_CLOCK_CTL to value 0x0100_0000"]
impl crate::Resettable for CM0_CLOCK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
