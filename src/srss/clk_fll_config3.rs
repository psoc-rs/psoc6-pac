#[doc = "Reader of register CLK_FLL_CONFIG3"]
pub type R = crate::R<u32, super::CLK_FLL_CONFIG3>;
#[doc = "Writer for register CLK_FLL_CONFIG3"]
pub type W = crate::W<u32, super::CLK_FLL_CONFIG3>;
#[doc = "Register CLK_FLL_CONFIG3 `reset()`'s with value 0x2800"]
impl crate::ResetValue for super::CLK_FLL_CONFIG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2800
    }
}
#[doc = "Reader of field `FLL_LF_IGAIN`"]
pub type FLL_LF_IGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLL_LF_IGAIN`"]
pub struct FLL_LF_IGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_LF_IGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FLL_LF_PGAIN`"]
pub type FLL_LF_PGAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLL_LF_PGAIN`"]
pub struct FLL_LF_PGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_LF_PGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SETTLING_COUNT`"]
pub type SETTLING_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SETTLING_COUNT`"]
pub struct SETTLING_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLING_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 8)) | (((value as u32) & 0x1fff) << 8);
        self.w
    }
}
#[doc = "Bypass mux located just after FLL output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: Automatic using lock indicator.  When unlocked, automatically selects FLL reference input (bypass mode).  When locked, automatically selects FLL output."]
    AUTO = 0,
    #[doc = "1: Same as AUTO"]
    AUTO1 = 1,
    #[doc = "2: Select FLL reference input (bypass mode).  Ignores lock indicator"]
    FLL_REF = 2,
    #[doc = "3: Select FLL output.  Ignores lock indicator."]
    FLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BYPASS_SEL`"]
pub type BYPASS_SEL_R = crate::R<u8, BYPASS_SEL_A>;
impl BYPASS_SEL_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `BYPASS_SEL`"]
pub struct BYPASS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects FLL reference input (bypass mode). When locked, automatically selects FLL output."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "Same as AUTO"]
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - FLL Loop Filter Integral Gain Setting 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&self) -> FLL_LF_IGAIN_R {
        FLL_LF_IGAIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Proportional Gain Setting 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&self) -> FLL_LF_PGAIN_R {
        FLL_LF_PGAIN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&self) -> SETTLING_COUNT_R {
        SETTLING_COUNT_R::new(((self.bits >> 8) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLL Loop Filter Integral Gain Setting 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_igain(&mut self) -> FLL_LF_IGAIN_W {
        FLL_LF_IGAIN_W { w: self }
    }
    #[doc = "Bits 4:7 - FLL Loop Filter Proportional Gain Setting 0: 1/256 1: 1/128 2: 1/64 3: 1/32 4: 1/16 5: 1/8 6: 1/4 7: 1/2 8: 1.0 9: 2.0 10: 4.0 11: 8.0 >=12: illegal"]
    #[inline(always)]
    pub fn fll_lf_pgain(&mut self) -> FLL_LF_PGAIN_W {
        FLL_LF_PGAIN_W { w: self }
    }
    #[doc = "Bits 8:20 - Number of undivided reference clock cycles to wait after changing the CCO trim until the loop measurement restarts. A delay allows the CCO output to settle and gives a more accurate measurement. The default is tuned to an 8MHz reference clock since the IMO is expected to be the most common use case. 0: no settling time 1: wait one reference clock cycle ... 8191: wait 8191 reference clock cycles"]
    #[inline(always)]
    pub fn settling_count(&mut self) -> SETTLING_COUNT_W {
        SETTLING_COUNT_W { w: self }
    }
    #[doc = "Bits 28:29 - Bypass mux located just after FLL output."]
    #[inline(always)]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W {
        BYPASS_SEL_W { w: self }
    }
}
