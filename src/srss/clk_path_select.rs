#[doc = "Register `CLK_PATH_SELECT[%s]` reader"]
pub struct R(crate::R<CLK_PATH_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PATH_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PATH_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PATH_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PATH_SELECT[%s]` writer"]
pub struct W(crate::W<CLK_PATH_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PATH_SELECT_SPEC>;
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
impl From<crate::W<CLK_PATH_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PATH_SELECT_SPEC>) -> Self {
        W(writer)
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
    #[doc = "4: DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    DSI_MUX = 4,
}
impl From<PATH_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: PATH_MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PATH_MUX` reader - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PATH_MUX_R = crate::FieldReader<u8, PATH_MUX_A>;
impl PATH_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PATH_MUX_A> {
        match self.bits {
            0 => Some(PATH_MUX_A::IMO),
            1 => Some(PATH_MUX_A::EXTCLK),
            2 => Some(PATH_MUX_A::ECO),
            3 => Some(PATH_MUX_A::ALTHF),
            4 => Some(PATH_MUX_A::DSI_MUX),
            _ => None,
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
#[doc = "Field `PATH_MUX` writer - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PATH_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_PATH_SELECT_SPEC, u8, PATH_MUX_A, 3, O>;
impl<'a, const O: u8> PATH_MUX_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:2 - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&self) -> PATH_MUX_R {
        PATH_MUX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a source for clock PATH<i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&mut self) -> PATH_MUX_W<0> {
        PATH_MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Path Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_path_select](index.html) module"]
pub struct CLK_PATH_SELECT_SPEC;
impl crate::RegisterSpec for CLK_PATH_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_path_select::R](R) reader structure"]
impl crate::Readable for CLK_PATH_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_path_select::W](W) writer structure"]
impl crate::Writable for CLK_PATH_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PATH_SELECT[%s]
to value 0"]
impl crate::Resettable for CLK_PATH_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
