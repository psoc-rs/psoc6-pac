#[doc = "Register `AREF_CTRL` reader"]
pub struct R(crate::R<AREF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AREF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AREF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AREF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AREF_CTRL` writer"]
pub struct W(crate::W<AREF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AREF_CTRL_SPEC>;
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
impl From<crate::W<AREF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AREF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Control bit to trade off AREF settling and noise performance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AREF_MODE_A {
    #[doc = "0: Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    NORMAL = 0,
    #[doc = "1: High noise fast startup mode (meets fast mode settling and noise specifications)"]
    FAST_START = 1,
}
impl From<AREF_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: AREF_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AREF_MODE` reader - Control bit to trade off AREF settling and noise performance"]
pub type AREF_MODE_R = crate::BitReader<AREF_MODE_A>;
impl AREF_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AREF_MODE_A {
        match self.bits {
            false => AREF_MODE_A::NORMAL,
            true => AREF_MODE_A::FAST_START,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == AREF_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FAST_START`"]
    #[inline(always)]
    pub fn is_fast_start(&self) -> bool {
        *self == AREF_MODE_A::FAST_START
    }
}
#[doc = "Field `AREF_MODE` writer - Control bit to trade off AREF settling and noise performance"]
pub type AREF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AREF_CTRL_SPEC, AREF_MODE_A, O>;
impl<'a, const O: u8> AREF_MODE_W<'a, O> {
    #[doc = "Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AREF_MODE_A::NORMAL)
    }
    #[doc = "High noise fast startup mode (meets fast mode settling and noise specifications)"]
    #[inline(always)]
    pub fn fast_start(self) -> &'a mut W {
        self.variant(AREF_MODE_A::FAST_START)
    }
}
#[doc = "Field `AREF_BIAS_SCALE` reader - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub type AREF_BIAS_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREF_BIAS_SCALE` writer - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub type AREF_BIAS_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AREF_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AREF_RMB` reader - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub type AREF_RMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AREF_RMB` writer - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub type AREF_RMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AREF_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CTB_IPTAT_SCALE` reader - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub type CTB_IPTAT_SCALE_R = crate::BitReader<bool>;
#[doc = "Field `CTB_IPTAT_SCALE` writer - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub type CTB_IPTAT_SCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AREF_CTRL_SPEC, bool, O>;
#[doc = "Field `CTB_IPTAT_REDIRECT` reader - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
pub type CTB_IPTAT_REDIRECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTB_IPTAT_REDIRECT` writer - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
pub type CTB_IPTAT_REDIRECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AREF_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "iztat current select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IZTAT_SEL_A {
    #[doc = "0: Use 250nA IZTAT from SRSS"]
    SRSS = 0,
    #[doc = "1: Use locally generated 250nA"]
    LOCAL = 1,
}
impl From<IZTAT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IZTAT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IZTAT_SEL` reader - iztat current select control"]
pub type IZTAT_SEL_R = crate::BitReader<IZTAT_SEL_A>;
impl IZTAT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IZTAT_SEL_A {
        match self.bits {
            false => IZTAT_SEL_A::SRSS,
            true => IZTAT_SEL_A::LOCAL,
        }
    }
    #[doc = "Checks if the value of the field is `SRSS`"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        *self == IZTAT_SEL_A::SRSS
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == IZTAT_SEL_A::LOCAL
    }
}
#[doc = "Field `IZTAT_SEL` writer - iztat current select control"]
pub type IZTAT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AREF_CTRL_SPEC, IZTAT_SEL_A, O>;
impl<'a, const O: u8> IZTAT_SEL_W<'a, O> {
    #[doc = "Use 250nA IZTAT from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut W {
        self.variant(IZTAT_SEL_A::SRSS)
    }
    #[doc = "Use locally generated 250nA"]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(IZTAT_SEL_A::LOCAL)
    }
}
#[doc = "Field `CLOCK_PUMP_PERI_SEL` reader - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub type CLOCK_PUMP_PERI_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_PUMP_PERI_SEL` writer - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub type CLOCK_PUMP_PERI_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AREF_CTRL_SPEC, bool, O>;
#[doc = "bandgap voltage select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREF_SEL_A {
    #[doc = "0: Use 0.8V Vref from SRSS"]
    SRSS = 0,
    #[doc = "1: Use locally generated Vref"]
    LOCAL = 1,
    #[doc = "2: Use externally supplied Vref (aref_ext_vref)"]
    EXTERNAL = 2,
}
impl From<VREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREF_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREF_SEL` reader - bandgap voltage select control"]
pub type VREF_SEL_R = crate::FieldReader<u8, VREF_SEL_A>;
impl VREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREF_SEL_A> {
        match self.bits {
            0 => Some(VREF_SEL_A::SRSS),
            1 => Some(VREF_SEL_A::LOCAL),
            2 => Some(VREF_SEL_A::EXTERNAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SRSS`"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        *self == VREF_SEL_A::SRSS
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == VREF_SEL_A::LOCAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == VREF_SEL_A::EXTERNAL
    }
}
#[doc = "Field `VREF_SEL` writer - bandgap voltage select control"]
pub type VREF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AREF_CTRL_SPEC, u8, VREF_SEL_A, 2, O>;
impl<'a, const O: u8> VREF_SEL_W<'a, O> {
    #[doc = "Use 0.8V Vref from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut W {
        self.variant(VREF_SEL_A::SRSS)
    }
    #[doc = "Use locally generated Vref"]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(VREF_SEL_A::LOCAL)
    }
    #[doc = "Use externally supplied Vref (aref_ext_vref)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(VREF_SEL_A::EXTERNAL)
    }
}
#[doc = "AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEEPSLEEP_MODE_A {
    #[doc = "0: All blocks 'OFF' in DeepSleep"]
    OFF = 0,
    #[doc = "1: IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    IPTAT = 1,
    #[doc = "2: IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    IPTAT_IZTAT = 2,
    #[doc = "3: IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    IPTAT_IZTAT_VREF = 3,
}
impl From<DEEPSLEEP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEEPSLEEP_MODE` reader - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub type DEEPSLEEP_MODE_R = crate::FieldReader<u8, DEEPSLEEP_MODE_A>;
impl DEEPSLEEP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPSLEEP_MODE_A {
        match self.bits {
            0 => DEEPSLEEP_MODE_A::OFF,
            1 => DEEPSLEEP_MODE_A::IPTAT,
            2 => DEEPSLEEP_MODE_A::IPTAT_IZTAT,
            3 => DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `IPTAT`"]
    #[inline(always)]
    pub fn is_iptat(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::IPTAT
    }
    #[doc = "Checks if the value of the field is `IPTAT_IZTAT`"]
    #[inline(always)]
    pub fn is_iptat_iztat(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::IPTAT_IZTAT
    }
    #[doc = "Checks if the value of the field is `IPTAT_IZTAT_VREF`"]
    #[inline(always)]
    pub fn is_iptat_iztat_vref(&self) -> bool {
        *self == DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF
    }
}
#[doc = "Field `DEEPSLEEP_MODE` writer - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub type DEEPSLEEP_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AREF_CTRL_SPEC, u8, DEEPSLEEP_MODE_A, 2, O>;
impl<'a, const O: u8> DEEPSLEEP_MODE_W<'a, O> {
    #[doc = "All blocks 'OFF' in DeepSleep"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::OFF)
    }
    #[doc = "IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    #[inline(always)]
    pub fn iptat(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT)
    }
    #[doc = "IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    #[inline(always)]
    pub fn iptat_iztat(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT_IZTAT)
    }
    #[doc = "IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    #[inline(always)]
    pub fn iptat_iztat_vref(self) -> &'a mut W {
        self.variant(DEEPSLEEP_MODE_A::IPTAT_IZTAT_VREF)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AREF_CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Disable AREF"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Disable AREF"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, AREF_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn aref_mode(&self) -> AREF_MODE_R {
        AREF_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn aref_bias_scale(&self) -> AREF_BIAS_SCALE_R {
        AREF_BIAS_SCALE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn aref_rmb(&self) -> AREF_RMB_R {
        AREF_RMB_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn ctb_iptat_scale(&self) -> CTB_IPTAT_SCALE_R {
        CTB_IPTAT_SCALE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn ctb_iptat_redirect(&self) -> CTB_IPTAT_REDIRECT_R {
        CTB_IPTAT_REDIRECT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    pub fn iztat_sel(&self) -> IZTAT_SEL_R {
        IZTAT_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn clock_pump_peri_sel(&self) -> CLOCK_PUMP_PERI_SEL_R {
        CLOCK_PUMP_PERI_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VREF_SEL_R {
        VREF_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn deepsleep_mode(&self) -> DEEPSLEEP_MODE_R {
        DEEPSLEEP_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable AREF"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn aref_mode(&mut self) -> AREF_MODE_W<0> {
        AREF_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn aref_bias_scale(&mut self) -> AREF_BIAS_SCALE_W<2> {
        AREF_BIAS_SCALE_W::new(self)
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn aref_rmb(&mut self) -> AREF_RMB_W<4> {
        AREF_RMB_W::new(self)
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn ctb_iptat_scale(&mut self) -> CTB_IPTAT_SCALE_W<7> {
        CTB_IPTAT_SCALE_W::new(self)
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp<n>.IPTAT = AREF.IPTAT and Opamp<n>.IZTAT= AREF.IZTAT 1: Opamp<n>.IPTAT = HiZ and Opamp<n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp<n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn ctb_iptat_redirect(&mut self) -> CTB_IPTAT_REDIRECT_W<8> {
        CTB_IPTAT_REDIRECT_W::new(self)
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    pub fn iztat_sel(&mut self) -> IZTAT_SEL_W<16> {
        IZTAT_SEL_W::new(self)
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn clock_pump_peri_sel(&mut self) -> CLOCK_PUMP_PERI_SEL_W<19> {
        CLOCK_PUMP_PERI_SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VREF_SEL_W<20> {
        VREF_SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn deepsleep_mode(&mut self) -> DEEPSLEEP_MODE_W<28> {
        DEEPSLEEP_MODE_W::new(self)
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W<30> {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 31 - Disable AREF"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "global AREF control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aref_ctrl](index.html) module"]
pub struct AREF_CTRL_SPEC;
impl crate::RegisterSpec for AREF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aref_ctrl::R](R) reader structure"]
impl crate::Readable for AREF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aref_ctrl::W](W) writer structure"]
impl crate::Writable for AREF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AREF_CTRL to value 0"]
impl crate::Resettable for AREF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
