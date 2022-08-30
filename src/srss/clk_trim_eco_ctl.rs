#[doc = "Register `CLK_TRIM_ECO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_ECO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_ECO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_ECO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_ECO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_ECO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_ECO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_ECO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_ECO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_ECO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTRIM` reader - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub type WDTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDTRIM` writer - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
pub type WDTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `ATRIM` reader - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub type ATRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATRIM` writer - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
pub type ATRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `FTRIM` reader - Filter Trim - 3rd harmonic oscillation"]
pub type FTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTRIM` writer - Filter Trim - 3rd harmonic oscillation"]
pub type FTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTRIM` reader - Feedback resistor Trim"]
pub type RTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTRIM` writer - Feedback resistor Trim"]
pub type RTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `GTRIM` reader - Gain Trim - Startup time"]
pub type GTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTRIM` writer - Gain Trim - Startup time"]
pub type GTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ITRIM` reader - Current Trim"]
pub type ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITRIM` writer - Current Trim"]
pub type ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_ECO_CTL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&self) -> WDTRIM_R {
        WDTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&self) -> ATRIM_R {
        ATRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&self) -> FTRIM_R {
        FTRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&self) -> RTRIM_R {
        RTRIM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&self) -> GTRIM_R {
        GTRIM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below steady state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&mut self) -> WDTRIM_W<0> {
        WDTRIM_W::new(self)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&mut self) -> ATRIM_W<4> {
        ATRIM_W::new(self)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&mut self) -> FTRIM_W<8> {
        FTRIM_W::new(self)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&mut self) -> RTRIM_W<10> {
        RTRIM_W::new(self)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&mut self) -> GTRIM_W<12> {
        GTRIM_W::new(self)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W<16> {
        ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_eco_ctl](index.html) module"]
pub struct CLK_TRIM_ECO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_ECO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_eco_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_ECO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_eco_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_ECO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_ECO_CTL to value 0x001f_0003"]
impl crate::Resettable for CLK_TRIM_ECO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001f_0003
    }
}
