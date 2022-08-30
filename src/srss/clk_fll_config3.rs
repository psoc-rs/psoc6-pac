#[doc = "Register `CLK_FLL_CONFIG3` reader"]
pub struct R(crate::R<CLK_FLL_CONFIG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_FLL_CONFIG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_FLL_CONFIG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_FLL_CONFIG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_FLL_CONFIG3` writer"]
pub struct W(crate::W<CLK_FLL_CONFIG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_FLL_CONFIG3_SPEC>;
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
impl From<crate::W<CLK_FLL_CONFIG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_FLL_CONFIG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLL_LF_IGAIN` reader - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_IGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLL_LF_IGAIN` writer - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_IGAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG3_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLL_LF_PGAIN` reader - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_PGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLL_LF_PGAIN` writer - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
pub type FLL_LF_PGAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG3_SPEC, u8, u8, 4, O>;
#[doc = "Field `SETTLING_COUNT` reader - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SETTLING_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SETTLING_COUNT` writer - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
pub type SETTLING_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_FLL_CONFIG3_SPEC, u16, u16, 13, O>;
#[doc = "Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: N/A"]
    AUTO = 0,
    #[doc = "1: N/A"]
    AUTO1 = 1,
    #[doc = "2: Select FLL reference input (bypass mode). Ignores lock indicator"]
    FLL_REF = 2,
    #[doc = "3: Select FLL output. Ignores lock indicator."]
    FLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BYPASS_SEL_R = crate::FieldReader<u8, BYPASS_SEL_A>;
impl BYPASS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_SEL_A {
        match self.bits {
            0 => BYPASS_SEL_A::AUTO,
            1 => BYPASS_SEL_A::AUTO1,
            2 => BYPASS_SEL_A::FLL_REF,
            3 => BYPASS_SEL_A::FLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO
    }
    #[doc = "Checks if the value of the field is `AUTO1`"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO1
    }
    #[doc = "Checks if the value of the field is `FLL_REF`"]
    #[inline(always)]
    pub fn is_fll_ref(&self) -> bool {
        *self == BYPASS_SEL_A::FLL_REF
    }
    #[doc = "Checks if the value of the field is `FLL_OUT`"]
    #[inline(always)]
    pub fn is_fll_out(&self) -> bool {
        *self == BYPASS_SEL_A::FLL_OUT
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
pub type BYPASS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_FLL_CONFIG3_SPEC, u8, BYPASS_SEL_A, 2, O>;
impl<'a, const O: u8> BYPASS_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO1)
    }
    #[doc = "Select FLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn fll_ref(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::FLL_REF)
    }
    #[doc = "Select FLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn fll_out(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::FLL_OUT)
    }
}
impl R {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&self) -> FLL_LF_IGAIN_R {
        FLL_LF_IGAIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&self) -> FLL_LF_PGAIN_R {
        FLL_LF_PGAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&self) -> SETTLING_COUNT_R {
        SETTLING_COUNT_R::new(((self.bits >> 8) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLL Loop Filter Gain Setting #1. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&mut self) -> FLL_LF_IGAIN_W<0> {
        FLL_LF_IGAIN_W::new(self)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Gain Setting #2. The proportional gain is the sum of FLL_LF_IGAIN and FLL_LF_PGAIN. 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&mut self) -> FLL_LF_PGAIN_W<4> {
        FLL_LF_PGAIN_W::new(self)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&mut self) -> SETTLING_COUNT_W<8> {
        SETTLING_COUNT_W::new(self)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output. See FLL_ENABLE description for instructions on how to use this field when enabling/disabling the FLL."]
    #[inline(always)]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W<28> {
        BYPASS_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLL Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_fll_config3](index.html) module"]
pub struct CLK_FLL_CONFIG3_SPEC;
impl crate::RegisterSpec for CLK_FLL_CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_fll_config3::R](R) reader structure"]
impl crate::Readable for CLK_FLL_CONFIG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_fll_config3::W](W) writer structure"]
impl crate::Writable for CLK_FLL_CONFIG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG3 to value 0x2800"]
impl crate::Resettable for CLK_FLL_CONFIG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2800
    }
}
