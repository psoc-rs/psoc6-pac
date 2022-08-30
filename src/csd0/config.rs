#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREF_SEL_A {
    #[doc = "0: N/A"]
    IREF_SRSS = 0,
    #[doc = "1: N/A"]
    IREF_PASS = 1,
}
impl From<IREF_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: IREF_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREF_SEL` reader - N/A"]
pub type IREF_SEL_R = crate::BitReader<IREF_SEL_A>;
impl IREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREF_SEL_A {
        match self.bits {
            false => IREF_SEL_A::IREF_SRSS,
            true => IREF_SEL_A::IREF_PASS,
        }
    }
    #[doc = "Checks if the value of the field is `IREF_SRSS`"]
    #[inline(always)]
    pub fn is_iref_srss(&self) -> bool {
        *self == IREF_SEL_A::IREF_SRSS
    }
    #[doc = "Checks if the value of the field is `IREF_PASS`"]
    #[inline(always)]
    pub fn is_iref_pass(&self) -> bool {
        *self == IREF_SEL_A::IREF_PASS
    }
}
#[doc = "Field `IREF_SEL` writer - N/A"]
pub type IREF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, IREF_SEL_A, O>;
impl<'a, const O: u8> IREF_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn iref_srss(self) -> &'a mut W {
        self.variant(IREF_SEL_A::IREF_SRSS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn iref_pass(self) -> &'a mut W {
        self.variant(IREF_SEL_A::IREF_PASS)
    }
}
#[doc = "Field `FILTER_DELAY` reader - Enables the digital filtering on the CSD comparator"]
pub type FILTER_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER_DELAY` writer - Enables the digital filtering on the CSD comparator"]
pub type FILTER_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Configures the delay between shield clock and sensor clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHIELD_DELAY_A {
    #[doc = "0: Delay line is off; sensor clock = shield clock"]
    OFF = 0,
    #[doc = "1: shield clock is delayed by 5ns delay w.r.t sensor clock"]
    D5NS = 1,
    #[doc = "2: shield clock is delayed by 10ns delay w.r.t sensor clock"]
    D10NS = 2,
    #[doc = "3: shield clock is delayed by 20ns delay w.r.t sensor clock"]
    D20NS = 3,
}
impl From<SHIELD_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: SHIELD_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SHIELD_DELAY` reader - Configures the delay between shield clock and sensor clock"]
pub type SHIELD_DELAY_R = crate::FieldReader<u8, SHIELD_DELAY_A>;
impl SHIELD_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELD_DELAY_A {
        match self.bits {
            0 => SHIELD_DELAY_A::OFF,
            1 => SHIELD_DELAY_A::D5NS,
            2 => SHIELD_DELAY_A::D10NS,
            3 => SHIELD_DELAY_A::D20NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SHIELD_DELAY_A::OFF
    }
    #[doc = "Checks if the value of the field is `D5NS`"]
    #[inline(always)]
    pub fn is_d5ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D5NS
    }
    #[doc = "Checks if the value of the field is `D10NS`"]
    #[inline(always)]
    pub fn is_d10ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D10NS
    }
    #[doc = "Checks if the value of the field is `D20NS`"]
    #[inline(always)]
    pub fn is_d20ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D20NS
    }
}
#[doc = "Field `SHIELD_DELAY` writer - Configures the delay between shield clock and sensor clock"]
pub type SHIELD_DELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, SHIELD_DELAY_A, 2, O>;
impl<'a, const O: u8> SHIELD_DELAY_W<'a, O> {
    #[doc = "Delay line is off; sensor clock = shield clock"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::OFF)
    }
    #[doc = "shield clock is delayed by 5ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d5ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D5NS)
    }
    #[doc = "shield clock is delayed by 10ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d10ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D10NS)
    }
    #[doc = "shield clock is delayed by 20ns delay w.r.t sensor clock"]
    #[inline(always)]
    pub fn d20ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D20NS)
    }
}
#[doc = "Field `SENSE_EN` reader - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
pub type SENSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SENSE_EN` writer - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
pub type SENSE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_WAVE_A {
    #[doc = "0: Half Wave mode"]
    HALFWAVE = 0,
    #[doc = "1: Full Wave mode"]
    FULLWAVE = 1,
}
impl From<FULL_WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: FULL_WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL_WAVE` reader - N/A"]
pub type FULL_WAVE_R = crate::BitReader<FULL_WAVE_A>;
impl FULL_WAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL_WAVE_A {
        match self.bits {
            false => FULL_WAVE_A::HALFWAVE,
            true => FULL_WAVE_A::FULLWAVE,
        }
    }
    #[doc = "Checks if the value of the field is `HALFWAVE`"]
    #[inline(always)]
    pub fn is_halfwave(&self) -> bool {
        *self == FULL_WAVE_A::HALFWAVE
    }
    #[doc = "Checks if the value of the field is `FULLWAVE`"]
    #[inline(always)]
    pub fn is_fullwave(&self) -> bool {
        *self == FULL_WAVE_A::FULLWAVE
    }
}
#[doc = "Field `FULL_WAVE` writer - N/A"]
pub type FULL_WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, FULL_WAVE_A, O>;
impl<'a, const O: u8> FULL_WAVE_W<'a, O> {
    #[doc = "Half Wave mode"]
    #[inline(always)]
    pub fn halfwave(self) -> &'a mut W {
        self.variant(FULL_WAVE_A::HALFWAVE)
    }
    #[doc = "Full Wave mode"]
    #[inline(always)]
    pub fn fullwave(self) -> &'a mut W {
        self.variant(FULL_WAVE_A::FULLWAVE)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTUAL_CAP_A {
    #[doc = "0: Self-cap mode"]
    SELFCAP = 0,
    #[doc = "1: Mutual-cap mode"]
    MUTUALCAP = 1,
}
impl From<MUTUAL_CAP_A> for bool {
    #[inline(always)]
    fn from(variant: MUTUAL_CAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTUAL_CAP` reader - N/A"]
pub type MUTUAL_CAP_R = crate::BitReader<MUTUAL_CAP_A>;
impl MUTUAL_CAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTUAL_CAP_A {
        match self.bits {
            false => MUTUAL_CAP_A::SELFCAP,
            true => MUTUAL_CAP_A::MUTUALCAP,
        }
    }
    #[doc = "Checks if the value of the field is `SELFCAP`"]
    #[inline(always)]
    pub fn is_selfcap(&self) -> bool {
        *self == MUTUAL_CAP_A::SELFCAP
    }
    #[doc = "Checks if the value of the field is `MUTUALCAP`"]
    #[inline(always)]
    pub fn is_mutualcap(&self) -> bool {
        *self == MUTUAL_CAP_A::MUTUALCAP
    }
}
#[doc = "Field `MUTUAL_CAP` writer - N/A"]
pub type MUTUAL_CAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, MUTUAL_CAP_A, O>;
impl<'a, const O: u8> MUTUAL_CAP_W<'a, O> {
    #[doc = "Self-cap mode"]
    #[inline(always)]
    pub fn selfcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::SELFCAP)
    }
    #[doc = "Mutual-cap mode"]
    #[inline(always)]
    pub fn mutualcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::MUTUALCAP)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSX_DUAL_CNT_A {
    #[doc = "0: N/A"]
    ONE = 0,
    #[doc = "1: N/A"]
    TWO = 1,
}
impl From<CSX_DUAL_CNT_A> for bool {
    #[inline(always)]
    fn from(variant: CSX_DUAL_CNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSX_DUAL_CNT` reader - N/A"]
pub type CSX_DUAL_CNT_R = crate::BitReader<CSX_DUAL_CNT_A>;
impl CSX_DUAL_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSX_DUAL_CNT_A {
        match self.bits {
            false => CSX_DUAL_CNT_A::ONE,
            true => CSX_DUAL_CNT_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSX_DUAL_CNT_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSX_DUAL_CNT_A::TWO
    }
}
#[doc = "Field `CSX_DUAL_CNT` writer - N/A"]
pub type CSX_DUAL_CNT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, CSX_DUAL_CNT_A, O>;
impl<'a, const O: u8> CSX_DUAL_CNT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::TWO)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_COUNT_SEL_A {
    #[doc = "0: N/A"]
    CSD_RESULT = 0,
    #[doc = "1: N/A"]
    ADC_RESULT = 1,
}
impl From<DSI_COUNT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_COUNT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_COUNT_SEL` reader - N/A"]
pub type DSI_COUNT_SEL_R = crate::BitReader<DSI_COUNT_SEL_A>;
impl DSI_COUNT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_COUNT_SEL_A {
        match self.bits {
            false => DSI_COUNT_SEL_A::CSD_RESULT,
            true => DSI_COUNT_SEL_A::ADC_RESULT,
        }
    }
    #[doc = "Checks if the value of the field is `CSD_RESULT`"]
    #[inline(always)]
    pub fn is_csd_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::CSD_RESULT
    }
    #[doc = "Checks if the value of the field is `ADC_RESULT`"]
    #[inline(always)]
    pub fn is_adc_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::ADC_RESULT
    }
}
#[doc = "Field `DSI_COUNT_SEL` writer - N/A"]
pub type DSI_COUNT_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SPEC, DSI_COUNT_SEL_A, O>;
impl<'a, const O: u8> DSI_COUNT_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::CSD_RESULT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn adc_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::ADC_RESULT)
    }
}
#[doc = "Field `DSI_SAMPLE_EN` reader - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
pub type DSI_SAMPLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SAMPLE_EN` writer - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
pub type DSI_SAMPLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `SAMPLE_SYNC` reader - N/A"]
pub type SAMPLE_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE_SYNC` writer - N/A"]
pub type SAMPLE_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `DSI_SENSE_EN` reader - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
pub type DSI_SENSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SENSE_EN` writer - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
pub type DSI_SENSE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `LP_MODE` reader - N/A"]
pub type LP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LP_MODE` writer - N/A"]
pub type LP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - N/A"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - N/A"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn iref_sel(&self) -> IREF_SEL_R {
        IREF_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Enables the digital filtering on the CSD comparator"]
    #[inline(always)]
    pub fn filter_delay(&self) -> FILTER_DELAY_R {
        FILTER_DELAY_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - Configures the delay between shield clock and sensor clock"]
    #[inline(always)]
    pub fn shield_delay(&self) -> SHIELD_DELAY_R {
        SHIELD_DELAY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
    #[inline(always)]
    pub fn sense_en(&self) -> SENSE_EN_R {
        SENSE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn full_wave(&self) -> FULL_WAVE_R {
        FULL_WAVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn mutual_cap(&self) -> MUTUAL_CAP_R {
        MUTUAL_CAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn csx_dual_cnt(&self) -> CSX_DUAL_CNT_R {
        CSX_DUAL_CNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn dsi_count_sel(&self) -> DSI_COUNT_SEL_R {
        DSI_COUNT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
    #[inline(always)]
    pub fn dsi_sample_en(&self) -> DSI_SAMPLE_EN_R {
        DSI_SAMPLE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn sample_sync(&self) -> SAMPLE_SYNC_R {
        SAMPLE_SYNC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
    #[inline(always)]
    pub fn dsi_sense_en(&self) -> DSI_SENSE_EN_R {
        DSI_SENSE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn iref_sel(&mut self) -> IREF_SEL_W<0> {
        IREF_SEL_W::new(self)
    }
    #[doc = "Bits 4:8 - Enables the digital filtering on the CSD comparator"]
    #[inline(always)]
    pub fn filter_delay(&mut self) -> FILTER_DELAY_W<4> {
        FILTER_DELAY_W::new(self)
    }
    #[doc = "Bits 10:11 - Configures the delay between shield clock and sensor clock"]
    #[inline(always)]
    pub fn shield_delay(&mut self) -> SHIELD_DELAY_W<10> {
        SHIELD_DELAY_W::new(self)
    }
    #[doc = "Bit 12 - Enables the sensor and shield clocks, CSD modulator output and turns on the IDAC compensation current as selected by CSD_IDAC."]
    #[inline(always)]
    pub fn sense_en(&mut self) -> SENSE_EN_W<12> {
        SENSE_EN_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn full_wave(&mut self) -> FULL_WAVE_W<17> {
        FULL_WAVE_W::new(self)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn mutual_cap(&mut self) -> MUTUAL_CAP_W<18> {
        MUTUAL_CAP_W::new(self)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn csx_dual_cnt(&mut self) -> CSX_DUAL_CNT_W<19> {
        CSX_DUAL_CNT_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn dsi_count_sel(&mut self) -> DSI_COUNT_SEL_W<24> {
        DSI_COUNT_SEL_W::new(self)
    }
    #[doc = "Bit 25 - DSI_SAMPLE_EN = 1 -> COUNTER will count the samples generated by DSI DSI_SAMPLE_EN = 0 -> COUNTER will count the samples generated by CSD modulator"]
    #[inline(always)]
    pub fn dsi_sample_en(&mut self) -> DSI_SAMPLE_EN_W<25> {
        DSI_SAMPLE_EN_W::new(self)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn sample_sync(&mut self) -> SAMPLE_SYNC_W<26> {
        SAMPLE_SYNC_W::new(self)
    }
    #[doc = "Bit 27 - DSI_SENSE_EN = 1-> sensor clock is driven directly by DSI DSI_SENSE_EN = 0-> sensor clock is driven by PRS/divide-by-2/DIRECT_CLOCK"]
    #[inline(always)]
    pub fn dsi_sense_en(&mut self) -> DSI_SENSE_EN_W<27> {
        DSI_SENSE_EN_W::new(self)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W<30> {
        LP_MODE_W::new(self)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x0400_0000"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
