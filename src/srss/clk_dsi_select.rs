#[doc = "Reader of register CLK_DSI_SELECT[%s]"]
pub type R = crate::R<u32, super::CLK_DSI_SELECT>;
#[doc = "Writer for register CLK_DSI_SELECT[%s]"]
pub type W = crate::W<u32, super::CLK_DSI_SELECT>;
#[doc = "Register CLK_DSI_SELECT[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_DSI_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH<i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_MUX_A {
    #[doc = "0: DSI0 - dsi_out\\[0\\]"]
    DSI_OUT0,
    #[doc = "1: DSI1 - dsi_out\\[1\\]"]
    DSI_OUT1,
    #[doc = "2: DSI2 - dsi_out\\[2\\]"]
    DSI_OUT2,
    #[doc = "3: DSI3 - dsi_out\\[3\\]"]
    DSI_OUT3,
    #[doc = "4: DSI4 - dsi_out\\[4\\]"]
    DSI_OUT4,
    #[doc = "5: DSI5 - dsi_out\\[5\\]"]
    DSI_OUT5,
    #[doc = "6: DSI6 - dsi_out\\[6\\]"]
    DSI_OUT6,
    #[doc = "7: DSI7 - dsi_out\\[7\\]"]
    DSI_OUT7,
    #[doc = "8: DSI8 - dsi_out\\[8\\]"]
    DSI_OUT8,
    #[doc = "9: DSI9 - dsi_out\\[9\\]"]
    DSI_OUT9,
    #[doc = "10: DSI10 - dsi_out\\[10\\]"]
    DSI_OUT10,
    #[doc = "11: DSI11 - dsi_out\\[11\\]"]
    DSI_OUT11,
    #[doc = "12: DSI12 - dsi_out\\[12\\]"]
    DSI_OUT12,
    #[doc = "13: DSI13 - dsi_out\\[13\\]"]
    DSI_OUT13,
    #[doc = "14: DSI14 - dsi_out\\[14\\]"]
    DSI_OUT14,
    #[doc = "15: DSI15 - dsi_out\\[15\\]"]
    DSI_OUT15,
    #[doc = "16: ILO - Internal Low-speed Oscillator"]
    ILO,
    #[doc = "17: WCO - Watch-Crystal Oscillator"]
    WCO,
    #[doc = "18: ALTLF - Alternate Low-Frequency Clock"]
    ALTLF,
    #[doc = "19: PILO - Precision Internal Low-speed Oscillator"]
    PILO,
}
impl From<DSI_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: DSI_MUX_A) -> Self {
        match variant {
            DSI_MUX_A::DSI_OUT0 => 0,
            DSI_MUX_A::DSI_OUT1 => 1,
            DSI_MUX_A::DSI_OUT2 => 2,
            DSI_MUX_A::DSI_OUT3 => 3,
            DSI_MUX_A::DSI_OUT4 => 4,
            DSI_MUX_A::DSI_OUT5 => 5,
            DSI_MUX_A::DSI_OUT6 => 6,
            DSI_MUX_A::DSI_OUT7 => 7,
            DSI_MUX_A::DSI_OUT8 => 8,
            DSI_MUX_A::DSI_OUT9 => 9,
            DSI_MUX_A::DSI_OUT10 => 10,
            DSI_MUX_A::DSI_OUT11 => 11,
            DSI_MUX_A::DSI_OUT12 => 12,
            DSI_MUX_A::DSI_OUT13 => 13,
            DSI_MUX_A::DSI_OUT14 => 14,
            DSI_MUX_A::DSI_OUT15 => 15,
            DSI_MUX_A::ILO => 16,
            DSI_MUX_A::WCO => 17,
            DSI_MUX_A::ALTLF => 18,
            DSI_MUX_A::PILO => 19,
        }
    }
}
#[doc = "Reader of field `DSI_MUX`"]
pub type DSI_MUX_R = crate::R<u8, DSI_MUX_A>;
impl DSI_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSI_MUX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSI_MUX_A::DSI_OUT0),
            1 => Val(DSI_MUX_A::DSI_OUT1),
            2 => Val(DSI_MUX_A::DSI_OUT2),
            3 => Val(DSI_MUX_A::DSI_OUT3),
            4 => Val(DSI_MUX_A::DSI_OUT4),
            5 => Val(DSI_MUX_A::DSI_OUT5),
            6 => Val(DSI_MUX_A::DSI_OUT6),
            7 => Val(DSI_MUX_A::DSI_OUT7),
            8 => Val(DSI_MUX_A::DSI_OUT8),
            9 => Val(DSI_MUX_A::DSI_OUT9),
            10 => Val(DSI_MUX_A::DSI_OUT10),
            11 => Val(DSI_MUX_A::DSI_OUT11),
            12 => Val(DSI_MUX_A::DSI_OUT12),
            13 => Val(DSI_MUX_A::DSI_OUT13),
            14 => Val(DSI_MUX_A::DSI_OUT14),
            15 => Val(DSI_MUX_A::DSI_OUT15),
            16 => Val(DSI_MUX_A::ILO),
            17 => Val(DSI_MUX_A::WCO),
            18 => Val(DSI_MUX_A::ALTLF),
            19 => Val(DSI_MUX_A::PILO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DSI_OUT0`"]
    #[inline(always)]
    pub fn is_dsi_out0(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT0
    }
    #[doc = "Checks if the value of the field is `DSI_OUT1`"]
    #[inline(always)]
    pub fn is_dsi_out1(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT1
    }
    #[doc = "Checks if the value of the field is `DSI_OUT2`"]
    #[inline(always)]
    pub fn is_dsi_out2(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT2
    }
    #[doc = "Checks if the value of the field is `DSI_OUT3`"]
    #[inline(always)]
    pub fn is_dsi_out3(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT3
    }
    #[doc = "Checks if the value of the field is `DSI_OUT4`"]
    #[inline(always)]
    pub fn is_dsi_out4(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT4
    }
    #[doc = "Checks if the value of the field is `DSI_OUT5`"]
    #[inline(always)]
    pub fn is_dsi_out5(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT5
    }
    #[doc = "Checks if the value of the field is `DSI_OUT6`"]
    #[inline(always)]
    pub fn is_dsi_out6(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT6
    }
    #[doc = "Checks if the value of the field is `DSI_OUT7`"]
    #[inline(always)]
    pub fn is_dsi_out7(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT7
    }
    #[doc = "Checks if the value of the field is `DSI_OUT8`"]
    #[inline(always)]
    pub fn is_dsi_out8(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT8
    }
    #[doc = "Checks if the value of the field is `DSI_OUT9`"]
    #[inline(always)]
    pub fn is_dsi_out9(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT9
    }
    #[doc = "Checks if the value of the field is `DSI_OUT10`"]
    #[inline(always)]
    pub fn is_dsi_out10(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT10
    }
    #[doc = "Checks if the value of the field is `DSI_OUT11`"]
    #[inline(always)]
    pub fn is_dsi_out11(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT11
    }
    #[doc = "Checks if the value of the field is `DSI_OUT12`"]
    #[inline(always)]
    pub fn is_dsi_out12(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT12
    }
    #[doc = "Checks if the value of the field is `DSI_OUT13`"]
    #[inline(always)]
    pub fn is_dsi_out13(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT13
    }
    #[doc = "Checks if the value of the field is `DSI_OUT14`"]
    #[inline(always)]
    pub fn is_dsi_out14(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT14
    }
    #[doc = "Checks if the value of the field is `DSI_OUT15`"]
    #[inline(always)]
    pub fn is_dsi_out15(&self) -> bool {
        *self == DSI_MUX_A::DSI_OUT15
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == DSI_MUX_A::ILO
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == DSI_MUX_A::WCO
    }
    #[doc = "Checks if the value of the field is `ALTLF`"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == DSI_MUX_A::ALTLF
    }
    #[doc = "Checks if the value of the field is `PILO`"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == DSI_MUX_A::PILO
    }
}
#[doc = "Write proxy for field `DSI_MUX`"]
pub struct DSI_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSI_MUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    #[inline(always)]
    pub fn dsi_out0(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT0)
    }
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    #[inline(always)]
    pub fn dsi_out1(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT1)
    }
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    #[inline(always)]
    pub fn dsi_out2(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT2)
    }
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    #[inline(always)]
    pub fn dsi_out3(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT3)
    }
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    #[inline(always)]
    pub fn dsi_out4(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT4)
    }
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    #[inline(always)]
    pub fn dsi_out5(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT5)
    }
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    #[inline(always)]
    pub fn dsi_out6(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT6)
    }
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    #[inline(always)]
    pub fn dsi_out7(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT7)
    }
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    #[inline(always)]
    pub fn dsi_out8(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT8)
    }
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    #[inline(always)]
    pub fn dsi_out9(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT9)
    }
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    #[inline(always)]
    pub fn dsi_out10(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT10)
    }
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    #[inline(always)]
    pub fn dsi_out11(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT11)
    }
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    #[inline(always)]
    pub fn dsi_out12(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT12)
    }
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    #[inline(always)]
    pub fn dsi_out13(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT13)
    }
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    #[inline(always)]
    pub fn dsi_out14(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT14)
    }
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    #[inline(always)]
    pub fn dsi_out15(self) -> &'a mut W {
        self.variant(DSI_MUX_A::DSI_OUT15)
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(DSI_MUX_A::ILO)
    }
    #[doc = "WCO - Watch-Crystal Oscillator"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(DSI_MUX_A::WCO)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut W {
        self.variant(DSI_MUX_A::ALTLF)
    }
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut W {
        self.variant(DSI_MUX_A::PILO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH<i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub fn dsi_mux(&self) -> DSI_MUX_R {
        DSI_MUX_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH<i> using CLK_SELECT_PATH register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub fn dsi_mux(&mut self) -> DSI_MUX_W {
        DSI_MUX_W { w: self }
    }
}
