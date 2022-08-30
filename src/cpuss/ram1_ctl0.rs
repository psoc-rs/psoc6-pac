#[doc = "Register `RAM1_CTL0` reader"]
pub struct R(crate::R<RAM1_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM1_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM1_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM1_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM1_CTL0` writer"]
pub struct W(crate::W<RAM1_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM1_CTL0_SPEC>;
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
impl From<crate::W<RAM1_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM1_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW_WS` reader - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SLOW_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOW_WS` writer - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type SLOW_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM1_CTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `FAST_WS` reader - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAST_WS` writer - Memory wait states for the fast clock domain ('clk_fast'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
pub type FAST_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM1_CTL0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
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
    #[doc = "Bits 0:1 - Memory wait states for the slow clock domain ('clk_slow'). The number of wait states is expressed in 'clk_hf' clock domain cycles."]
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
#[doc = "RAM 1 control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram1_ctl0](index.html) module"]
pub struct RAM1_CTL0_SPEC;
impl crate::RegisterSpec for RAM1_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram1_ctl0::R](R) reader structure"]
impl crate::Readable for RAM1_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram1_ctl0::W](W) writer structure"]
impl crate::Writable for RAM1_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM1_CTL0 to value 0x01"]
impl crate::Resettable for RAM1_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
