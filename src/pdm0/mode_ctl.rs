#[doc = "Reader of register MODE_CTL"]
pub type R = crate::R<u32, super::MODE_CTL>;
#[doc = "Writer for register MODE_CTL"]
pub type W = crate::W<u32, super::MODE_CTL>;
#[doc = "Register MODE_CTL `reset()`'s with value 0x1b00_0103"]
impl crate::ResetValue for super::MODE_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1b00_0103
    }
}
#[doc = "Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_CH_SET_A {
    #[doc = "0: Channel disabled"]
    DISABLED,
    #[doc = "1: Mono left channel enable"]
    MONO_L,
    #[doc = "2: Mono right channel enable"]
    MONO_R,
    #[doc = "3: Stereo channel enable"]
    STEREO,
}
impl From<PCM_CH_SET_A> for u8 {
    #[inline(always)]
    fn from(variant: PCM_CH_SET_A) -> Self {
        match variant {
            PCM_CH_SET_A::DISABLED => 0,
            PCM_CH_SET_A::MONO_L => 1,
            PCM_CH_SET_A::MONO_R => 2,
            PCM_CH_SET_A::STEREO => 3,
        }
    }
}
#[doc = "Reader of field `PCM_CH_SET`"]
pub type PCM_CH_SET_R = crate::R<u8, PCM_CH_SET_A>;
impl PCM_CH_SET_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `PCM_CH_SET`"]
pub struct PCM_CH_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM_CH_SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCM_CH_SET_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SWAP_LR`"]
pub type SWAP_LR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP_LR`"]
pub struct SWAP_LR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_LR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_CYCLES_A {
    #[doc = "0: 64steps"]
    STEP_NUM64,
    #[doc = "1: 96steps"]
    STEP_NUM96,
    #[doc = "2: 128steps"]
    STEP_NUM128,
    #[doc = "3: 160steps"]
    STEP_NUM160,
    #[doc = "4: 192steps"]
    STEP_NUM192,
    #[doc = "5: 256steps"]
    STEP_NUM256,
    #[doc = "6: 384steps"]
    STEP_NUM384,
    #[doc = "7: 512steps"]
    STEP_NUM512,
}
impl From<S_CYCLES_A> for u8 {
    #[inline(always)]
    fn from(variant: S_CYCLES_A) -> Self {
        match variant {
            S_CYCLES_A::STEP_NUM64 => 0,
            S_CYCLES_A::STEP_NUM96 => 1,
            S_CYCLES_A::STEP_NUM128 => 2,
            S_CYCLES_A::STEP_NUM160 => 3,
            S_CYCLES_A::STEP_NUM192 => 4,
            S_CYCLES_A::STEP_NUM256 => 5,
            S_CYCLES_A::STEP_NUM384 => 6,
            S_CYCLES_A::STEP_NUM512 => 7,
        }
    }
}
#[doc = "Reader of field `S_CYCLES`"]
pub type S_CYCLES_R = crate::R<u8, S_CYCLES_A>;
impl S_CYCLES_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `S_CYCLES`"]
pub struct S_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> S_CYCLES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_CYCLES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKO_DELAY_A {
    #[doc = "0: CLK_IS is 3*PDM_CLK period early"]
    ADV3,
    #[doc = "1: CLK_IS is 2*PDM_CLK period early"]
    ADV2,
    #[doc = "2: CLK_IS is 1*PDM_CLK period early"]
    ADV1,
    #[doc = "3: CLK_IS is the same as PDM_CKO"]
    NO_DELAY,
    #[doc = "4: CLK_IS is 1*PDM_CLK period late"]
    DLY1,
    #[doc = "5: CLK_IS is 2*PDM_CLK period late"]
    DLY2,
    #[doc = "6: CLK_IS is 3*PDM_CLK period late"]
    DLY3,
    #[doc = "7: CLK_IS is 4*PDM_CLK period late"]
    DLY4,
}
impl From<CKO_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CKO_DELAY_A) -> Self {
        match variant {
            CKO_DELAY_A::ADV3 => 0,
            CKO_DELAY_A::ADV2 => 1,
            CKO_DELAY_A::ADV1 => 2,
            CKO_DELAY_A::NO_DELAY => 3,
            CKO_DELAY_A::DLY1 => 4,
            CKO_DELAY_A::DLY2 => 5,
            CKO_DELAY_A::DLY3 => 6,
            CKO_DELAY_A::DLY4 => 7,
        }
    }
}
#[doc = "Reader of field `CKO_DELAY`"]
pub type CKO_DELAY_R = crate::R<u8, CKO_DELAY_A>;
impl CKO_DELAY_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CKO_DELAY`"]
pub struct CKO_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CKO_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKO_DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `HPF_GAIN`"]
pub type HPF_GAIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPF_GAIN`"]
pub struct HPF_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HPF_EN_N`"]
pub type HPF_EN_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPF_EN_N`"]
pub struct HPF_EN_N_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_EN_N_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn pcm_ch_set(&self) -> PCM_CH_SET_R {
        PCM_CH_SET_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn swap_lr(&self) -> SWAP_LR_R {
        SWAP_LR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn s_cycles(&self) -> S_CYCLES_R {
        S_CYCLES_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn cko_delay(&self) -> CKO_DELAY_R {
        CKO_DELAY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\] (Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn hpf_gain(&self) -> HPF_GAIN_R {
        HPF_GAIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn hpf_en_n(&self) -> HPF_EN_N_R {
        HPF_EN_N_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn pcm_ch_set(&mut self) -> PCM_CH_SET_W {
        PCM_CH_SET_W { w: self }
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn swap_lr(&mut self) -> SWAP_LR_W {
        SWAP_LR_W { w: self }
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn s_cycles(&mut self) -> S_CYCLES_W {
        S_CYCLES_W { w: self }
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn cko_delay(&mut self) -> CKO_DELAY_W {
        CKO_DELAY_W { w: self }
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\] (Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn hpf_gain(&mut self) -> HPF_GAIN_W {
        HPF_GAIN_W { w: self }
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn hpf_en_n(&mut self) -> HPF_EN_N_W {
        HPF_EN_N_W { w: self }
    }
}
