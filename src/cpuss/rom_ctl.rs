#[doc = "Register `ROM_CTL` reader"]
pub struct R(crate::R<ROM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_CTL` writer"]
pub struct W(crate::W<ROM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_CTL_SPEC>;
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
impl From<crate::W<ROM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
pub type SLOW_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
pub type SLOW_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROM_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ROM_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    pub fn slow_ws(&self) -> SLOW_WS_R {
        SLOW_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&self) -> FAST_WS_R {
        FAST_WS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles. Timing paths to and from the memory have a (fixed) minimum duration that always needs to be considered/met. The 'clk_hf' clock domain frequency determines this field's value such that the timing paths minimum duration is met. A table/formula will be provided for this field's values for different 'clk_hf' frequencies."]
    #[inline(always)]
    pub fn slow_ws(&mut self) -> SLOW_WS_W<0> {
        SLOW_WS_W::new(self)
    }
    #[doc = "Bits 8:9 - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
    #[inline(always)]
    pub fn fast_ws(&mut self) -> FAST_WS_W<8> {
        FAST_WS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctl](index.html) module"]
pub struct ROM_CTL_SPEC;
impl crate::RegisterSpec for ROM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_ctl::R](R) reader structure"]
impl crate::Readable for ROM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_ctl::W](W) writer structure"]
impl crate::Writable for ROM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_CTL to value 0x01"]
impl crate::Resettable for ROM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
