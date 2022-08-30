#[doc = "Register `MODE_CTL` reader"]
pub struct R(crate::R<MODE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CTL` writer"]
pub struct W(crate::W<MODE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CTL_SPEC>;
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
impl From<crate::W<MODE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCM_CH_SET_A {
    #[doc = "0: Channel disabled"]
    DISABLED = 0,
    #[doc = "1: Mono left channel enable"]
    MONO_L = 1,
    #[doc = "2: Mono right channel enable"]
    MONO_R = 2,
    #[doc = "3: Stereo channel enable"]
    STEREO = 3,
}
impl From<PCM_CH_SET_A> for u8 {
    #[inline(always)]
    fn from(variant: PCM_CH_SET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PCM_CH_SET` reader - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
pub type PCM_CH_SET_R = crate::FieldReader<u8, PCM_CH_SET_A>;
impl PCM_CH_SET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_CH_SET_A {
        match self.bits {
            0 => PCM_CH_SET_A::DISABLED,
            1 => PCM_CH_SET_A::MONO_L,
            2 => PCM_CH_SET_A::MONO_R,
            3 => PCM_CH_SET_A::STEREO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCM_CH_SET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `MONO_L`"]
    #[inline(always)]
    pub fn is_mono_l(&self) -> bool {
        *self == PCM_CH_SET_A::MONO_L
    }
    #[doc = "Checks if the value of the field is `MONO_R`"]
    #[inline(always)]
    pub fn is_mono_r(&self) -> bool {
        *self == PCM_CH_SET_A::MONO_R
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == PCM_CH_SET_A::STEREO
    }
}
#[doc = "Field `PCM_CH_SET` writer - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
pub type PCM_CH_SET_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODE_CTL_SPEC, u8, PCM_CH_SET_A, 2, O>;
impl<'a, const O: u8> PCM_CH_SET_W<'a, O> {
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCM_CH_SET_A::DISABLED)
    }
    #[doc = "Mono left channel enable"]
    #[inline(always)]
    pub fn mono_l(self) -> &'a mut W {
        self.variant(PCM_CH_SET_A::MONO_L)
    }
    #[doc = "Mono right channel enable"]
    #[inline(always)]
    pub fn mono_r(self) -> &'a mut W {
        self.variant(PCM_CH_SET_A::MONO_R)
    }
    #[doc = "Stereo channel enable"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(PCM_CH_SET_A::STEREO)
    }
}
#[doc = "Field `SWAP_LR` reader - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
pub type SWAP_LR_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_LR` writer - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
pub type SWAP_LR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CTL_SPEC, bool, O>;
#[doc = "Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S_CYCLES_A {
    #[doc = "0: 64steps"]
    STEP_NUM64 = 0,
    #[doc = "1: 96steps"]
    STEP_NUM96 = 1,
    #[doc = "2: 128steps"]
    STEP_NUM128 = 2,
    #[doc = "3: 160steps"]
    STEP_NUM160 = 3,
    #[doc = "4: 192steps"]
    STEP_NUM192 = 4,
    #[doc = "5: 256steps"]
    STEP_NUM256 = 5,
    #[doc = "6: 384steps"]
    STEP_NUM384 = 6,
    #[doc = "7: 512steps"]
    STEP_NUM512 = 7,
}
impl From<S_CYCLES_A> for u8 {
    #[inline(always)]
    fn from(variant: S_CYCLES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `S_CYCLES` reader - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
pub type S_CYCLES_R = crate::FieldReader<u8, S_CYCLES_A>;
impl S_CYCLES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_CYCLES_A {
        match self.bits {
            0 => S_CYCLES_A::STEP_NUM64,
            1 => S_CYCLES_A::STEP_NUM96,
            2 => S_CYCLES_A::STEP_NUM128,
            3 => S_CYCLES_A::STEP_NUM160,
            4 => S_CYCLES_A::STEP_NUM192,
            5 => S_CYCLES_A::STEP_NUM256,
            6 => S_CYCLES_A::STEP_NUM384,
            7 => S_CYCLES_A::STEP_NUM512,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STEP_NUM64`"]
    #[inline(always)]
    pub fn is_step_num64(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM64
    }
    #[doc = "Checks if the value of the field is `STEP_NUM96`"]
    #[inline(always)]
    pub fn is_step_num96(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM96
    }
    #[doc = "Checks if the value of the field is `STEP_NUM128`"]
    #[inline(always)]
    pub fn is_step_num128(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM128
    }
    #[doc = "Checks if the value of the field is `STEP_NUM160`"]
    #[inline(always)]
    pub fn is_step_num160(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM160
    }
    #[doc = "Checks if the value of the field is `STEP_NUM192`"]
    #[inline(always)]
    pub fn is_step_num192(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM192
    }
    #[doc = "Checks if the value of the field is `STEP_NUM256`"]
    #[inline(always)]
    pub fn is_step_num256(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM256
    }
    #[doc = "Checks if the value of the field is `STEP_NUM384`"]
    #[inline(always)]
    pub fn is_step_num384(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM384
    }
    #[doc = "Checks if the value of the field is `STEP_NUM512`"]
    #[inline(always)]
    pub fn is_step_num512(&self) -> bool {
        *self == S_CYCLES_A::STEP_NUM512
    }
}
#[doc = "Field `S_CYCLES` writer - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
pub type S_CYCLES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODE_CTL_SPEC, u8, S_CYCLES_A, 3, O>;
impl<'a, const O: u8> S_CYCLES_W<'a, O> {
    #[doc = "64steps"]
    #[inline(always)]
    pub fn step_num64(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM64)
    }
    #[doc = "96steps"]
    #[inline(always)]
    pub fn step_num96(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM96)
    }
    #[doc = "128steps"]
    #[inline(always)]
    pub fn step_num128(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM128)
    }
    #[doc = "160steps"]
    #[inline(always)]
    pub fn step_num160(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM160)
    }
    #[doc = "192steps"]
    #[inline(always)]
    pub fn step_num192(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM192)
    }
    #[doc = "256steps"]
    #[inline(always)]
    pub fn step_num256(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM256)
    }
    #[doc = "384steps"]
    #[inline(always)]
    pub fn step_num384(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM384)
    }
    #[doc = "512steps"]
    #[inline(always)]
    pub fn step_num512(self) -> &'a mut W {
        self.variant(S_CYCLES_A::STEP_NUM512)
    }
}
#[doc = "Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKO_DELAY_A {
    #[doc = "0: CLK_IS is 3*PDM_CLK period early"]
    ADV3 = 0,
    #[doc = "1: CLK_IS is 2*PDM_CLK period early"]
    ADV2 = 1,
    #[doc = "2: CLK_IS is 1*PDM_CLK period early"]
    ADV1 = 2,
    #[doc = "3: CLK_IS is the same as PDM_CKO"]
    NO_DELAY = 3,
    #[doc = "4: CLK_IS is 1*PDM_CLK period late"]
    DLY1 = 4,
    #[doc = "5: CLK_IS is 2*PDM_CLK period late"]
    DLY2 = 5,
    #[doc = "6: CLK_IS is 3*PDM_CLK period late"]
    DLY3 = 6,
    #[doc = "7: CLK_IS is 4*PDM_CLK period late"]
    DLY4 = 7,
}
impl From<CKO_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CKO_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKO_DELAY` reader - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
pub type CKO_DELAY_R = crate::FieldReader<u8, CKO_DELAY_A>;
impl CKO_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKO_DELAY_A {
        match self.bits {
            0 => CKO_DELAY_A::ADV3,
            1 => CKO_DELAY_A::ADV2,
            2 => CKO_DELAY_A::ADV1,
            3 => CKO_DELAY_A::NO_DELAY,
            4 => CKO_DELAY_A::DLY1,
            5 => CKO_DELAY_A::DLY2,
            6 => CKO_DELAY_A::DLY3,
            7 => CKO_DELAY_A::DLY4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADV3`"]
    #[inline(always)]
    pub fn is_adv3(&self) -> bool {
        *self == CKO_DELAY_A::ADV3
    }
    #[doc = "Checks if the value of the field is `ADV2`"]
    #[inline(always)]
    pub fn is_adv2(&self) -> bool {
        *self == CKO_DELAY_A::ADV2
    }
    #[doc = "Checks if the value of the field is `ADV1`"]
    #[inline(always)]
    pub fn is_adv1(&self) -> bool {
        *self == CKO_DELAY_A::ADV1
    }
    #[doc = "Checks if the value of the field is `NO_DELAY`"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == CKO_DELAY_A::NO_DELAY
    }
    #[doc = "Checks if the value of the field is `DLY1`"]
    #[inline(always)]
    pub fn is_dly1(&self) -> bool {
        *self == CKO_DELAY_A::DLY1
    }
    #[doc = "Checks if the value of the field is `DLY2`"]
    #[inline(always)]
    pub fn is_dly2(&self) -> bool {
        *self == CKO_DELAY_A::DLY2
    }
    #[doc = "Checks if the value of the field is `DLY3`"]
    #[inline(always)]
    pub fn is_dly3(&self) -> bool {
        *self == CKO_DELAY_A::DLY3
    }
    #[doc = "Checks if the value of the field is `DLY4`"]
    #[inline(always)]
    pub fn is_dly4(&self) -> bool {
        *self == CKO_DELAY_A::DLY4
    }
}
#[doc = "Field `CKO_DELAY` writer - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
pub type CKO_DELAY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MODE_CTL_SPEC, u8, CKO_DELAY_A, 3, O>;
impl<'a, const O: u8> CKO_DELAY_W<'a, O> {
    #[doc = "CLK_IS is 3*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv3(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::ADV3)
    }
    #[doc = "CLK_IS is 2*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv2(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::ADV2)
    }
    #[doc = "CLK_IS is 1*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv1(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::ADV1)
    }
    #[doc = "CLK_IS is the same as PDM_CKO"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::NO_DELAY)
    }
    #[doc = "CLK_IS is 1*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly1(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::DLY1)
    }
    #[doc = "CLK_IS is 2*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly2(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::DLY2)
    }
    #[doc = "CLK_IS is 3*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly3(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::DLY3)
    }
    #[doc = "CLK_IS is 4*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly4(self) -> &'a mut W {
        self.variant(CKO_DELAY_A::DLY4)
    }
}
#[doc = "Field `HPF_GAIN` reader - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
pub type HPF_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPF_GAIN` writer - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
pub type HPF_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `HPF_EN_N` reader - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
pub type HPF_EN_N_R = crate::BitReader<bool>;
#[doc = "Field `HPF_EN_N` writer - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
pub type HPF_EN_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn pcm_ch_set(&self) -> PCM_CH_SET_R {
        PCM_CH_SET_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn swap_lr(&self) -> SWAP_LR_R {
        SWAP_LR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn s_cycles(&self) -> S_CYCLES_R {
        S_CYCLES_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn cko_delay(&self) -> CKO_DELAY_R {
        CKO_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn hpf_gain(&self) -> HPF_GAIN_R {
        HPF_GAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn hpf_en_n(&self) -> HPF_EN_N_R {
        HPF_EN_N_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn pcm_ch_set(&mut self) -> PCM_CH_SET_W<0> {
        PCM_CH_SET_W::new(self)
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn swap_lr(&mut self) -> SWAP_LR_W<2> {
        SWAP_LR_W::new(self)
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn s_cycles(&mut self) -> S_CYCLES_W<8> {
        S_CYCLES_W::new(self)
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn cko_delay(&mut self) -> CKO_DELAY_W<16> {
        CKO_DELAY_W::new(self)
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn hpf_gain(&mut self) -> HPF_GAIN_W<24> {
        HPF_GAIN_W::new(self)
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn hpf_en_n(&mut self) -> HPF_EN_N_W<28> {
        HPF_EN_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_ctl](index.html) module"]
pub struct MODE_CTL_SPEC;
impl crate::RegisterSpec for MODE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_ctl::R](R) reader structure"]
impl crate::Readable for MODE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_ctl::W](W) writer structure"]
impl crate::Writable for MODE_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_CTL to value 0x1b00_0103"]
impl crate::Resettable for MODE_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1b00_0103
    }
}
