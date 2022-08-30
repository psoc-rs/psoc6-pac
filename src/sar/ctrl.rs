#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VREF buffer low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_CTRL_VREF_A {
    #[doc = "0: full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    PWR_100 = 0,
    #[doc = "1: 80 percent power"]
    PWR_80 = 1,
    #[doc = "2: 60 percent power"]
    PWR_60 = 2,
    #[doc = "3: 50 percent power"]
    PWR_50 = 3,
    #[doc = "4: 40 percent power"]
    PWR_40 = 4,
    #[doc = "5: 30 percent power"]
    PWR_30 = 5,
    #[doc = "6: 20 percent power"]
    PWR_20 = 6,
    #[doc = "7: 10 percent power"]
    PWR_10 = 7,
}
impl From<PWR_CTRL_VREF_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_CTRL_VREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWR_CTRL_VREF` reader - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_R = crate::FieldReader<u8, PWR_CTRL_VREF_A>;
impl PWR_CTRL_VREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_CTRL_VREF_A {
        match self.bits {
            0 => PWR_CTRL_VREF_A::PWR_100,
            1 => PWR_CTRL_VREF_A::PWR_80,
            2 => PWR_CTRL_VREF_A::PWR_60,
            3 => PWR_CTRL_VREF_A::PWR_50,
            4 => PWR_CTRL_VREF_A::PWR_40,
            5 => PWR_CTRL_VREF_A::PWR_30,
            6 => PWR_CTRL_VREF_A::PWR_20,
            7 => PWR_CTRL_VREF_A::PWR_10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWR_100`"]
    #[inline(always)]
    pub fn is_pwr_100(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_100
    }
    #[doc = "Checks if the value of the field is `PWR_80`"]
    #[inline(always)]
    pub fn is_pwr_80(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_80
    }
    #[doc = "Checks if the value of the field is `PWR_60`"]
    #[inline(always)]
    pub fn is_pwr_60(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_60
    }
    #[doc = "Checks if the value of the field is `PWR_50`"]
    #[inline(always)]
    pub fn is_pwr_50(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_50
    }
    #[doc = "Checks if the value of the field is `PWR_40`"]
    #[inline(always)]
    pub fn is_pwr_40(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_40
    }
    #[doc = "Checks if the value of the field is `PWR_30`"]
    #[inline(always)]
    pub fn is_pwr_30(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_30
    }
    #[doc = "Checks if the value of the field is `PWR_20`"]
    #[inline(always)]
    pub fn is_pwr_20(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_20
    }
    #[doc = "Checks if the value of the field is `PWR_10`"]
    #[inline(always)]
    pub fn is_pwr_10(&self) -> bool {
        *self == PWR_CTRL_VREF_A::PWR_10
    }
}
#[doc = "Field `PWR_CTRL_VREF` writer - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PWR_CTRL_VREF_A, 3, O>;
impl<'a, const O: u8> PWR_CTRL_VREF_W<'a, O> {
    #[doc = "full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn pwr_100(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_100)
    }
    #[doc = "80 percent power"]
    #[inline(always)]
    pub fn pwr_80(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_80)
    }
    #[doc = "60 percent power"]
    #[inline(always)]
    pub fn pwr_60(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_60)
    }
    #[doc = "50 percent power"]
    #[inline(always)]
    pub fn pwr_50(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_50)
    }
    #[doc = "40 percent power"]
    #[inline(always)]
    pub fn pwr_40(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_40)
    }
    #[doc = "30 percent power"]
    #[inline(always)]
    pub fn pwr_30(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_30)
    }
    #[doc = "20 percent power"]
    #[inline(always)]
    pub fn pwr_20(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_20)
    }
    #[doc = "10 percent power"]
    #[inline(always)]
    pub fn pwr_10(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::PWR_10)
    }
}
#[doc = "SARADC internal VREF selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREF_SEL_A {
    #[doc = "0: VREF0 from PRB (VREF buffer on)"]
    VREF0 = 0,
    #[doc = "1: VREF1 from PRB (VREF buffer on)"]
    VREF1 = 1,
    #[doc = "2: VREF2 from PRB (VREF buffer on)"]
    VREF2 = 2,
    #[doc = "3: VREF from AROUTE (VREF buffer on)"]
    VREF_AROUTE = 3,
    #[doc = "4: 1.024V from BandGap (VREF buffer on)"]
    VBGR = 4,
    #[doc = "5: External precision Vref direct from a pin (low impedance path)."]
    VREF_EXT = 5,
    #[doc = "6: Vdda/2 (VREF buffer on)"]
    VDDA_DIV_2 = 6,
    #[doc = "7: Vdda."]
    VDDA = 7,
}
impl From<VREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREF_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREF_SEL` reader - SARADC internal VREF selection."]
pub type VREF_SEL_R = crate::FieldReader<u8, VREF_SEL_A>;
impl VREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_SEL_A {
        match self.bits {
            0 => VREF_SEL_A::VREF0,
            1 => VREF_SEL_A::VREF1,
            2 => VREF_SEL_A::VREF2,
            3 => VREF_SEL_A::VREF_AROUTE,
            4 => VREF_SEL_A::VBGR,
            5 => VREF_SEL_A::VREF_EXT,
            6 => VREF_SEL_A::VDDA_DIV_2,
            7 => VREF_SEL_A::VDDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREF0`"]
    #[inline(always)]
    pub fn is_vref0(&self) -> bool {
        *self == VREF_SEL_A::VREF0
    }
    #[doc = "Checks if the value of the field is `VREF1`"]
    #[inline(always)]
    pub fn is_vref1(&self) -> bool {
        *self == VREF_SEL_A::VREF1
    }
    #[doc = "Checks if the value of the field is `VREF2`"]
    #[inline(always)]
    pub fn is_vref2(&self) -> bool {
        *self == VREF_SEL_A::VREF2
    }
    #[doc = "Checks if the value of the field is `VREF_AROUTE`"]
    #[inline(always)]
    pub fn is_vref_aroute(&self) -> bool {
        *self == VREF_SEL_A::VREF_AROUTE
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREF_SEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VREF_EXT`"]
    #[inline(always)]
    pub fn is_vref_ext(&self) -> bool {
        *self == VREF_SEL_A::VREF_EXT
    }
    #[doc = "Checks if the value of the field is `VDDA_DIV_2`"]
    #[inline(always)]
    pub fn is_vdda_div_2(&self) -> bool {
        *self == VREF_SEL_A::VDDA_DIV_2
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == VREF_SEL_A::VDDA
    }
}
#[doc = "Field `VREF_SEL` writer - SARADC internal VREF selection."]
pub type VREF_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, VREF_SEL_A, 3, O>;
impl<'a, const O: u8> VREF_SEL_W<'a, O> {
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref0(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF0)
    }
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref1(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF1)
    }
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref2(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF2)
    }
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    #[inline(always)]
    pub fn vref_aroute(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF_AROUTE)
    }
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VBGR)
    }
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    #[inline(always)]
    pub fn vref_ext(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF_EXT)
    }
    #[doc = "Vdda/2 (VREF buffer on)"]
    #[inline(always)]
    pub fn vdda_div_2(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VDDA_DIV_2)
    }
    #[doc = "Vdda."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VDDA)
    }
}
#[doc = "Field `VREF_BYP_CAP_EN` reader - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `VREF_BYP_CAP_EN` writer - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "SARADC internal NEG selection for Single ended conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEG_SEL_A {
    #[doc = "0: NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    VSSA_KELVIN = 0,
    #[doc = "1: NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    ART_VSSA = 1,
    #[doc = "2: NEG input of SARADC is connected to P1 pin of SARMUX"]
    P1 = 2,
    #[doc = "3: NEG input of SARADC is connected to P3 pin of SARMUX"]
    P3 = 3,
    #[doc = "4: NEG input of SARADC is connected to P5 pin of SARMUX"]
    P5 = 4,
    #[doc = "5: NEG input of SARADC is connected to P7 pin of SARMUX"]
    P7 = 5,
    #[doc = "6: NEG input of SARADC is connected to an ACORE in AROUTE"]
    ACORE = 6,
    #[doc = "7: NEG input of SARADC is shorted with VREF input of SARADC."]
    VREF = 7,
}
impl From<NEG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEG_SEL` reader - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_R = crate::FieldReader<u8, NEG_SEL_A>;
impl NEG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEG_SEL_A {
        match self.bits {
            0 => NEG_SEL_A::VSSA_KELVIN,
            1 => NEG_SEL_A::ART_VSSA,
            2 => NEG_SEL_A::P1,
            3 => NEG_SEL_A::P3,
            4 => NEG_SEL_A::P5,
            5 => NEG_SEL_A::P7,
            6 => NEG_SEL_A::ACORE,
            7 => NEG_SEL_A::VREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VSSA_KELVIN`"]
    #[inline(always)]
    pub fn is_vssa_kelvin(&self) -> bool {
        *self == NEG_SEL_A::VSSA_KELVIN
    }
    #[doc = "Checks if the value of the field is `ART_VSSA`"]
    #[inline(always)]
    pub fn is_art_vssa(&self) -> bool {
        *self == NEG_SEL_A::ART_VSSA
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == NEG_SEL_A::P1
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == NEG_SEL_A::P3
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == NEG_SEL_A::P5
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == NEG_SEL_A::P7
    }
    #[doc = "Checks if the value of the field is `ACORE`"]
    #[inline(always)]
    pub fn is_acore(&self) -> bool {
        *self == NEG_SEL_A::ACORE
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == NEG_SEL_A::VREF
    }
}
#[doc = "Field `NEG_SEL` writer - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, NEG_SEL_A, 3, O>;
impl<'a, const O: u8> NEG_SEL_W<'a, O> {
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    #[inline(always)]
    pub fn vssa_kelvin(self) -> &'a mut W {
        self.variant(NEG_SEL_A::VSSA_KELVIN)
    }
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    #[inline(always)]
    pub fn art_vssa(self) -> &'a mut W {
        self.variant(NEG_SEL_A::ART_VSSA)
    }
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P1)
    }
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P3)
    }
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P5)
    }
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P7)
    }
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    #[inline(always)]
    pub fn acore(self) -> &'a mut W {
        self.variant(NEG_SEL_A::ACORE)
    }
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(NEG_SEL_A::VREF)
    }
}
#[doc = "Field `SAR_HW_CTRL_NEGVREF` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SAR_HW_CTRL_NEGVREF_R = crate::BitReader<bool>;
#[doc = "Field `SAR_HW_CTRL_NEGVREF` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SAR_HW_CTRL_NEGVREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Set the comparator latch delay in accordance with SAR conversion rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_DLY_A {
    #[doc = "0: 2.5ns delay, use this for 2.5Msps"]
    D2P5 = 0,
    #[doc = "1: 4.0ns delay, use this for 2.0Msps"]
    D4 = 1,
    #[doc = "2: 10ns delay, use this for 1.5Msps"]
    D10 = 2,
    #[doc = "3: 12ns delay, use this for 1.0Msps or less"]
    D12 = 3,
}
impl From<COMP_DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_DLY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_DLY` reader - Set the comparator latch delay in accordance with SAR conversion rate"]
pub type COMP_DLY_R = crate::FieldReader<u8, COMP_DLY_A>;
impl COMP_DLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_DLY_A {
        match self.bits {
            0 => COMP_DLY_A::D2P5,
            1 => COMP_DLY_A::D4,
            2 => COMP_DLY_A::D10,
            3 => COMP_DLY_A::D12,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D2P5`"]
    #[inline(always)]
    pub fn is_d2p5(&self) -> bool {
        *self == COMP_DLY_A::D2P5
    }
    #[doc = "Checks if the value of the field is `D4`"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == COMP_DLY_A::D4
    }
    #[doc = "Checks if the value of the field is `D10`"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == COMP_DLY_A::D10
    }
    #[doc = "Checks if the value of the field is `D12`"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == COMP_DLY_A::D12
    }
}
#[doc = "Field `COMP_DLY` writer - Set the comparator latch delay in accordance with SAR conversion rate"]
pub type COMP_DLY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, COMP_DLY_A, 2, O>;
impl<'a, const O: u8> COMP_DLY_W<'a, O> {
    #[doc = "2.5ns delay, use this for 2.5Msps"]
    #[inline(always)]
    pub fn d2p5(self) -> &'a mut W {
        self.variant(COMP_DLY_A::D2P5)
    }
    #[doc = "4.0ns delay, use this for 2.0Msps"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut W {
        self.variant(COMP_DLY_A::D4)
    }
    #[doc = "10ns delay, use this for 1.5Msps"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut W {
        self.variant(COMP_DLY_A::D10)
    }
    #[doc = "12ns delay, use this for 1.0Msps or less"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut W {
        self.variant(COMP_DLY_A::D12)
    }
}
#[doc = "Field `SPARE` reader - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE` writer - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BOOSTPUMP_EN` reader - deprecated"]
pub type BOOSTPUMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `BOOSTPUMP_EN` writer - deprecated"]
pub type BOOSTPUMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REFBUF_EN` reader - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
pub type REFBUF_EN_R = crate::BitReader<bool>;
#[doc = "Field `REFBUF_EN` writer - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
pub type REFBUF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Comparator power mode. (Sample rate TBD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_PWR_A {
    #[doc = "0: Power = 100 percent, use this for >2000Ksps"]
    P100 = 0,
    #[doc = "1: Power = 80 percent, use this for 1500-2000Ksps"]
    P80 = 1,
    #[doc = "2: Power = 60 percent, use this for 1000-1500Ksps"]
    P60 = 2,
    #[doc = "3: Power = 50 percent, use this for 500-1000Ksps"]
    P50 = 3,
    #[doc = "4: Power = 40 percent, use this for 250-500Ksps"]
    P40 = 4,
    #[doc = "5: Power = 30 percent, use this for 100-250Ksps"]
    P30 = 5,
    #[doc = "6: Power = 20 percent, use this for 100-250Ksps (TBD!)"]
    P20 = 6,
    #[doc = "7: Power = 10 percent, use this for <100Ksps"]
    P10 = 7,
}
impl From<COMP_PWR_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_PWR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP_PWR` reader - Comparator power mode. (Sample rate TBD)"]
pub type COMP_PWR_R = crate::FieldReader<u8, COMP_PWR_A>;
impl COMP_PWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_PWR_A {
        match self.bits {
            0 => COMP_PWR_A::P100,
            1 => COMP_PWR_A::P80,
            2 => COMP_PWR_A::P60,
            3 => COMP_PWR_A::P50,
            4 => COMP_PWR_A::P40,
            5 => COMP_PWR_A::P30,
            6 => COMP_PWR_A::P20,
            7 => COMP_PWR_A::P10,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P100`"]
    #[inline(always)]
    pub fn is_p100(&self) -> bool {
        *self == COMP_PWR_A::P100
    }
    #[doc = "Checks if the value of the field is `P80`"]
    #[inline(always)]
    pub fn is_p80(&self) -> bool {
        *self == COMP_PWR_A::P80
    }
    #[doc = "Checks if the value of the field is `P60`"]
    #[inline(always)]
    pub fn is_p60(&self) -> bool {
        *self == COMP_PWR_A::P60
    }
    #[doc = "Checks if the value of the field is `P50`"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == COMP_PWR_A::P50
    }
    #[doc = "Checks if the value of the field is `P40`"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == COMP_PWR_A::P40
    }
    #[doc = "Checks if the value of the field is `P30`"]
    #[inline(always)]
    pub fn is_p30(&self) -> bool {
        *self == COMP_PWR_A::P30
    }
    #[doc = "Checks if the value of the field is `P20`"]
    #[inline(always)]
    pub fn is_p20(&self) -> bool {
        *self == COMP_PWR_A::P20
    }
    #[doc = "Checks if the value of the field is `P10`"]
    #[inline(always)]
    pub fn is_p10(&self) -> bool {
        *self == COMP_PWR_A::P10
    }
}
#[doc = "Field `COMP_PWR` writer - Comparator power mode. (Sample rate TBD)"]
pub type COMP_PWR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, COMP_PWR_A, 3, O>;
impl<'a, const O: u8> COMP_PWR_W<'a, O> {
    #[doc = "Power = 100 percent, use this for >2000Ksps"]
    #[inline(always)]
    pub fn p100(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P100)
    }
    #[doc = "Power = 80 percent, use this for 1500-2000Ksps"]
    #[inline(always)]
    pub fn p80(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P80)
    }
    #[doc = "Power = 60 percent, use this for 1000-1500Ksps"]
    #[inline(always)]
    pub fn p60(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P60)
    }
    #[doc = "Power = 50 percent, use this for 500-1000Ksps"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P50)
    }
    #[doc = "Power = 40 percent, use this for 250-500Ksps"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P40)
    }
    #[doc = "Power = 30 percent, use this for 100-250Ksps"]
    #[inline(always)]
    pub fn p30(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P30)
    }
    #[doc = "Power = 20 percent, use this for 100-250Ksps (TBD!)"]
    #[inline(always)]
    pub fn p20(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P20)
    }
    #[doc = "Power = 10 percent, use this for <100Ksps"]
    #[inline(always)]
    pub fn p10(self) -> &'a mut W {
        self.variant(COMP_PWR_A::P10)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DSI_SYNC_CONFIG` reader - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DSI_SYNC_CONFIG_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SYNC_CONFIG` writer - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DSI_SYNC_CONFIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DSI_MODE` reader - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DSI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DSI_MODE` writer - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DSI_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SWITCH_DISABLE` reader - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SWITCH_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SWITCH_DISABLE` writer - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SWITCH_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&self) -> PWR_CTRL_VREF_R {
        PWR_CTRL_VREF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    pub fn vref_sel(&self) -> VREF_SEL_R {
        VREF_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn vref_byp_cap_en(&self) -> VREF_BYP_CAP_EN_R {
        VREF_BYP_CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn neg_sel(&self) -> NEG_SEL_R {
        NEG_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&self) -> SAR_HW_CTRL_NEGVREF_R {
        SAR_HW_CTRL_NEGVREF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    pub fn comp_dly(&self) -> COMP_DLY_R {
        COMP_DLY_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - deprecated"]
    #[inline(always)]
    pub fn boostpump_en(&self) -> BOOSTPUMP_EN_R {
        BOOSTPUMP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    pub fn refbuf_en(&self) -> REFBUF_EN_R {
        REFBUF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator power mode. (Sample rate TBD)"]
    #[inline(always)]
    pub fn comp_pwr(&self) -> COMP_PWR_R {
        COMP_PWR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_config(&self) -> DSI_SYNC_CONFIG_R {
        DSI_SYNC_CONFIG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn dsi_mode(&self) -> DSI_MODE_R {
        DSI_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn switch_disable(&self) -> SWITCH_DISABLE_R {
        SWITCH_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&mut self) -> PWR_CTRL_VREF_W<0> {
        PWR_CTRL_VREF_W::new(self)
    }
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VREF_SEL_W<4> {
        VREF_SEL_W::new(self)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn vref_byp_cap_en(&mut self) -> VREF_BYP_CAP_EN_W<7> {
        VREF_BYP_CAP_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn neg_sel(&mut self) -> NEG_SEL_W<9> {
        NEG_SEL_W::new(self)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&mut self) -> SAR_HW_CTRL_NEGVREF_W<13> {
        SAR_HW_CTRL_NEGVREF_W::new(self)
    }
    #[doc = "Bits 14:15 - Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    pub fn comp_dly(&mut self) -> COMP_DLY_W<14> {
        COMP_DLY_W::new(self)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W<16> {
        SPARE_W::new(self)
    }
    #[doc = "Bit 20 - deprecated"]
    #[inline(always)]
    pub fn boostpump_en(&mut self) -> BOOSTPUMP_EN_W<20> {
        BOOSTPUMP_EN_W::new(self)
    }
    #[doc = "Bit 21 - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    pub fn refbuf_en(&mut self) -> REFBUF_EN_W<21> {
        REFBUF_EN_W::new(self)
    }
    #[doc = "Bits 24:26 - Comparator power mode. (Sample rate TBD)"]
    #[inline(always)]
    pub fn comp_pwr(&mut self) -> COMP_PWR_W<24> {
        COMP_PWR_W::new(self)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W<27> {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_config(&mut self) -> DSI_SYNC_CONFIG_W<28> {
        DSI_SYNC_CONFIG_W::new(self)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn dsi_mode(&mut self) -> DSI_MODE_W<29> {
        DSI_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn switch_disable(&mut self) -> SWITCH_DISABLE_W<30> {
        SWITCH_DISABLE_W::new(self)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
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
#[doc = "Analog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x1000_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
