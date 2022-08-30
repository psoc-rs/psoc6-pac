#[doc = "Register `CLK_PILO_CONFIG` reader"]
pub struct R(crate::R<CLK_PILO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PILO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PILO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PILO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PILO_CONFIG` writer"]
pub struct W(crate::W<CLK_PILO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PILO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_PILO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PILO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PILO_FFREQ` reader - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PILO_FFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PILO_FFREQ` writer - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PILO_FFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_PILO_CONFIG_SPEC, u16, u16, 10, O>;
#[doc = "Field `PILO_CLK_EN` reader - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PILO_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `PILO_CLK_EN` writer - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PILO_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_PILO_CONFIG_SPEC, bool, O>;
#[doc = "Field `PILO_RESET_N` reader - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PILO_RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `PILO_RESET_N` writer - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PILO_RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_PILO_CONFIG_SPEC, bool, O>;
#[doc = "Field `PILO_EN` reader - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PILO_EN_R = crate::BitReader<bool>;
#[doc = "Field `PILO_EN` writer - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PILO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_PILO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&self) -> PILO_FFREQ_R {
        PILO_FFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&self) -> PILO_CLK_EN_R {
        PILO_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&self) -> PILO_RESET_N_R {
        PILO_RESET_N_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&self) -> PILO_EN_R {
        PILO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&mut self) -> PILO_FFREQ_W<0> {
        PILO_FFREQ_W::new(self)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&mut self) -> PILO_CLK_EN_W<29> {
        PILO_CLK_EN_W::new(self)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&mut self) -> PILO_RESET_N_W<30> {
        PILO_RESET_N_W::new(self)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&mut self) -> PILO_EN_W<31> {
        PILO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Precision ILO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pilo_config](index.html) module"]
pub struct CLK_PILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_PILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pilo_config::R](R) reader structure"]
impl crate::Readable for CLK_PILO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pilo_config::W](W) writer structure"]
impl crate::Writable for CLK_PILO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PILO_CONFIG to value 0x80"]
impl crate::Resettable for CLK_PILO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
