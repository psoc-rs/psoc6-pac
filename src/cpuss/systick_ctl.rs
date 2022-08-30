#[doc = "Register `SYSTICK_CTL` reader"]
pub struct R(crate::R<SYSTICK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTICK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTICK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTICK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTICK_CTL` writer"]
pub struct W(crate::W<SYSTICK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTICK_CTL_SPEC>;
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
impl From<crate::W<SYSTICK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTICK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TENMS` writer - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
pub type TENMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSTICK_CTL_SPEC, u32, u32, 24, O>;
#[doc = "Field `CLOCK_SOURCE` reader - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
pub type CLOCK_SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_SOURCE` writer - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
pub type CLOCK_SOURCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSTICK_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKEW` reader - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSTICK_CTL_SPEC, bool, O>;
#[doc = "Field `NOREF` reader - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
pub type NOREF_R = crate::BitReader<bool>;
#[doc = "Field `NOREF` writer - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSTICK_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:25 - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    pub fn clock_source(&self) -> CLOCK_SOURCE_R {
        CLOCK_SOURCE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W<0> {
        TENMS_W::new(self)
    }
    #[doc = "Bits 24:25 - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    pub fn clock_source(&mut self) -> CLOCK_SOURCE_W<24> {
        CLOCK_SOURCE_W::new(self)
    }
    #[doc = "Bit 30 - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W<30> {
        SKEW_W::new(self)
    }
    #[doc = "Bit 31 - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W<31> {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systick_ctl](index.html) module"]
pub struct SYSTICK_CTL_SPEC;
impl crate::RegisterSpec for SYSTICK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systick_ctl::R](R) reader structure"]
impl crate::Readable for SYSTICK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systick_ctl::W](W) writer structure"]
impl crate::Writable for SYSTICK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTICK_CTL to value 0x4000_0147"]
impl crate::Resettable for SYSTICK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0147
    }
}
