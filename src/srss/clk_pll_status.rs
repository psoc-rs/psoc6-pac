#[doc = "Register `CLK_PLL_STATUS[%s]` reader"]
pub struct R(crate::R<CLK_PLL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PLL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PLL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PLL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PLL_STATUS[%s]` writer"]
pub struct W(crate::W<CLK_PLL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PLL_STATUS_SPEC>;
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
impl From<crate::W<CLK_PLL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PLL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKED` reader - PLL Lock Indicator"]
pub type LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK_OCCURRED` reader - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK_OCCURRED` writer - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UNLOCK_OCCURRED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_PLL_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W<1> {
        UNLOCK_OCCURRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_status](index.html) module"]
pub struct CLK_PLL_STATUS_SPEC;
impl crate::RegisterSpec for CLK_PLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pll_status::R](R) reader structure"]
impl crate::Readable for CLK_PLL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pll_status::W](W) writer structure"]
impl crate::Writable for CLK_PLL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PLL_STATUS[%s]
to value 0"]
impl crate::Resettable for CLK_PLL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
