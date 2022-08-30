#[doc = "Register `XTAL_CLK_DIV_CONFIG` reader"]
pub struct R(crate::R<XTAL_CLK_DIV_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_CLK_DIV_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_CLK_DIV_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_CLK_DIV_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_CLK_DIV_CONFIG` writer"]
pub struct W(crate::W<XTAL_CLK_DIV_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_CLK_DIV_CONFIG_SPEC>;
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
impl From<crate::W<XTAL_CLK_DIV_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_CLK_DIV_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCLK_DIV` reader - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
pub type SYSCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSCLK_DIV` writer - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
pub type SYSCLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CLK_DIV_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `LLCLK_DIV` reader - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
pub type LLCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LLCLK_DIV` writer - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
pub type LLCLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_CLK_DIV_CONFIG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn sysclk_div(&self) -> SYSCLK_DIV_R {
        SYSCLK_DIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn llclk_div(&self) -> LLCLK_DIV_R {
        LLCLK_DIV_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock pre-divider value. The 24 MHz crystal clock is divided to generate the system clock. 0: NO_DIV: SYSCLK= XTALCLK/1 1: DIV_BY_2: SYSCLK= XTALCLK/2 2: DIV_BY_4: SYSCLK= XTALCLK/4 3: DIV_BY_8: SYSCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn sysclk_div(&mut self) -> SYSCLK_DIV_W<0> {
        SYSCLK_DIV_W::new(self)
    }
    #[doc = "Bits 2:3 - Link Layer clock pre-divider value. The 24 MHz crystal clock is divided to generate the Link Layer clock. 0: NO_DIV: LLCLK= XTALCLK/1 1: DIV_BY_2: LLCLK= XTALCLK/2 2: DIV_BY_4: LLCLK= XTALCLK/4 3: DIV_BY_8: LLCLK= XTALCLK/8"]
    #[inline(always)]
    pub fn llclk_div(&mut self) -> LLCLK_DIV_W<2> {
        LLCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal clock divider configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_clk_div_config](index.html) module"]
pub struct XTAL_CLK_DIV_CONFIG_SPEC;
impl crate::RegisterSpec for XTAL_CLK_DIV_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_clk_div_config::R](R) reader structure"]
impl crate::Readable for XTAL_CLK_DIV_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_clk_div_config::W](W) writer structure"]
impl crate::Writable for XTAL_CLK_DIV_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL_CLK_DIV_CONFIG to value 0"]
impl crate::Resettable for XTAL_CLK_DIV_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
