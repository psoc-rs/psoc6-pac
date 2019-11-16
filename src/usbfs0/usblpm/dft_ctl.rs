#[doc = "Reader of register DFT_CTL"]
pub type R = crate::R<u32, super::DFT_CTL>;
#[doc = "Writer for register DFT_CTL"]
pub type W = crate::W<u32, super::DFT_CTL>;
#[doc = "Register DFT_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DFT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DDFT output select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DDFT_OUT_SEL_A {
    #[doc = "0: Nothing connected, output 0"]
    OFF = 0,
    #[doc = "1: Single Ended output of DP"]
    DP_SE = 1,
    #[doc = "2: Single Ended output of DM"]
    DM_SE = 2,
    #[doc = "3: Output Enable"]
    TXOE = 3,
    #[doc = "4: Differential Receiver output"]
    RCV_DF = 4,
    #[doc = "5: GPIO output of DP"]
    GPIO_DP_OUT = 5,
    #[doc = "6: GPIO output of DM"]
    GPIO_DM_OUT = 6,
}
impl From<DDFT_OUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT_OUT_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DDFT_OUT_SEL`"]
pub type DDFT_OUT_SEL_R = crate::R<u8, DDFT_OUT_SEL_A>;
impl DDFT_OUT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DDFT_OUT_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DDFT_OUT_SEL_A::OFF),
            1 => Val(DDFT_OUT_SEL_A::DP_SE),
            2 => Val(DDFT_OUT_SEL_A::DM_SE),
            3 => Val(DDFT_OUT_SEL_A::TXOE),
            4 => Val(DDFT_OUT_SEL_A::RCV_DF),
            5 => Val(DDFT_OUT_SEL_A::GPIO_DP_OUT),
            6 => Val(DDFT_OUT_SEL_A::GPIO_DM_OUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DDFT_OUT_SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `DP_SE`"]
    #[inline(always)]
    pub fn is_dp_se(&self) -> bool {
        *self == DDFT_OUT_SEL_A::DP_SE
    }
    #[doc = "Checks if the value of the field is `DM_SE`"]
    #[inline(always)]
    pub fn is_dm_se(&self) -> bool {
        *self == DDFT_OUT_SEL_A::DM_SE
    }
    #[doc = "Checks if the value of the field is `TXOE`"]
    #[inline(always)]
    pub fn is_txoe(&self) -> bool {
        *self == DDFT_OUT_SEL_A::TXOE
    }
    #[doc = "Checks if the value of the field is `RCV_DF`"]
    #[inline(always)]
    pub fn is_rcv_df(&self) -> bool {
        *self == DDFT_OUT_SEL_A::RCV_DF
    }
    #[doc = "Checks if the value of the field is `GPIO_DP_OUT`"]
    #[inline(always)]
    pub fn is_gpio_dp_out(&self) -> bool {
        *self == DDFT_OUT_SEL_A::GPIO_DP_OUT
    }
    #[doc = "Checks if the value of the field is `GPIO_DM_OUT`"]
    #[inline(always)]
    pub fn is_gpio_dm_out(&self) -> bool {
        *self == DDFT_OUT_SEL_A::GPIO_DM_OUT
    }
}
#[doc = "Write proxy for field `DDFT_OUT_SEL`"]
pub struct DDFT_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_OUT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDFT_OUT_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::OFF)
    }
    #[doc = "Single Ended output of DP"]
    #[inline(always)]
    pub fn dp_se(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::DP_SE)
    }
    #[doc = "Single Ended output of DM"]
    #[inline(always)]
    pub fn dm_se(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::DM_SE)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub fn txoe(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::TXOE)
    }
    #[doc = "Differential Receiver output"]
    #[inline(always)]
    pub fn rcv_df(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::RCV_DF)
    }
    #[doc = "GPIO output of DP"]
    #[inline(always)]
    pub fn gpio_dp_out(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::GPIO_DP_OUT)
    }
    #[doc = "GPIO output of DM"]
    #[inline(always)]
    pub fn gpio_dm_out(self) -> &'a mut W {
        self.variant(DDFT_OUT_SEL_A::GPIO_DM_OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "DDFT input select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DDFT_IN_SEL_A {
    #[doc = "0: Nothing connected, output 0"]
    OFF = 0,
    #[doc = "1: GPIO input of DP"]
    GPIO_DP_IN = 1,
    #[doc = "2: GPIO input of DM"]
    GPIO_DM_IN = 2,
}
impl From<DDFT_IN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT_IN_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DDFT_IN_SEL`"]
pub type DDFT_IN_SEL_R = crate::R<u8, DDFT_IN_SEL_A>;
impl DDFT_IN_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DDFT_IN_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DDFT_IN_SEL_A::OFF),
            1 => Val(DDFT_IN_SEL_A::GPIO_DP_IN),
            2 => Val(DDFT_IN_SEL_A::GPIO_DM_IN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DDFT_IN_SEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `GPIO_DP_IN`"]
    #[inline(always)]
    pub fn is_gpio_dp_in(&self) -> bool {
        *self == DDFT_IN_SEL_A::GPIO_DP_IN
    }
    #[doc = "Checks if the value of the field is `GPIO_DM_IN`"]
    #[inline(always)]
    pub fn is_gpio_dm_in(&self) -> bool {
        *self == DDFT_IN_SEL_A::GPIO_DM_IN
    }
}
#[doc = "Write proxy for field `DDFT_IN_SEL`"]
pub struct DDFT_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_IN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDFT_IN_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DDFT_IN_SEL_A::OFF)
    }
    #[doc = "GPIO input of DP"]
    #[inline(always)]
    pub fn gpio_dp_in(self) -> &'a mut W {
        self.variant(DDFT_IN_SEL_A::GPIO_DP_IN)
    }
    #[doc = "GPIO input of DM"]
    #[inline(always)]
    pub fn gpio_dm_in(self) -> &'a mut W {
        self.variant(DDFT_IN_SEL_A::GPIO_DM_IN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    pub fn ddft_out_sel(&self) -> DDFT_OUT_SEL_R {
        DDFT_OUT_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    pub fn ddft_in_sel(&self) -> DDFT_IN_SEL_R {
        DDFT_IN_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    pub fn ddft_out_sel(&mut self) -> DDFT_OUT_SEL_W {
        DDFT_OUT_SEL_W { w: self }
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    pub fn ddft_in_sel(&mut self) -> DDFT_IN_SEL_W {
        DDFT_IN_SEL_W { w: self }
    }
}
