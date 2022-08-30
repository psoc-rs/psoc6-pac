#[doc = "Register `CLK_TRIM_ILO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_ILO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_ILO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_ILO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_ILO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_ILO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_ILO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_ILO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_ILO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_ILO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILO_FTRIM` reader - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO_FTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILO_FTRIM` writer - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type ILO_FTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ILO_CTL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&self) -> ILO_FTRIM_R {
        ILO_FTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&mut self) -> ILO_FTRIM_W<0> {
        ILO_FTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ILO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_ilo_ctl](index.html) module"]
pub struct CLK_TRIM_ILO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ILO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_ilo_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_ILO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_ilo_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_ILO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_ILO_CTL to value 0x2c"]
impl crate::Resettable for CLK_TRIM_ILO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2c
    }
}
