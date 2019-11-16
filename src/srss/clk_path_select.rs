#[doc = "Reader of register CLK_PATH_SELECT[%s]"]
pub type R = crate::R<u32, super::CLK_PATH_SELECT>;
#[doc = "Writer for register CLK_PATH_SELECT[%s]"]
pub type W = crate::W<u32, super::CLK_PATH_SELECT>;
#[doc = "Register CLK_PATH_SELECT[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_PATH_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PATH_MUX_A {
    #[doc = "0: IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "1: EXTCLK - External Clock Pin"]
    EXTCLK = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    ECO = 2,
    #[doc = "3: ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    ALTHF = 3,
    #[doc = "4: DSI_MUX - Output of DSI mux for this path.  Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    DSI_MUX = 4,
}
impl From<PATH_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: PATH_MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PATH_MUX`"]
pub type PATH_MUX_R = crate::R<u8, PATH_MUX_A>;
impl PATH_MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PATH_MUX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PATH_MUX_A::IMO),
            1 => Val(PATH_MUX_A::EXTCLK),
            2 => Val(PATH_MUX_A::ECO),
            3 => Val(PATH_MUX_A::ALTHF),
            4 => Val(PATH_MUX_A::DSI_MUX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == PATH_MUX_A::IMO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == PATH_MUX_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == PATH_MUX_A::ECO
    }
    #[doc = "Checks if the value of the field is `ALTHF`"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == PATH_MUX_A::ALTHF
    }
    #[doc = "Checks if the value of the field is `DSI_MUX`"]
    #[inline(always)]
    pub fn is_dsi_mux(&self) -> bool {
        *self == PATH_MUX_A::DSI_MUX
    }
}
#[doc = "Write proxy for field `PATH_MUX`"]
pub struct PATH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PATH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PATH_MUX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(PATH_MUX_A::IMO)
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(PATH_MUX_A::EXTCLK)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(PATH_MUX_A::ECO)
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut W {
        self.variant(PATH_MUX_A::ALTHF)
    }
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    #[inline(always)]
    pub fn dsi_mux(self) -> &'a mut W {
        self.variant(PATH_MUX_A::DSI_MUX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&self) -> PATH_MUX_R {
        PATH_MUX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&mut self) -> PATH_MUX_W {
        PATH_MUX_W { w: self }
    }
}
