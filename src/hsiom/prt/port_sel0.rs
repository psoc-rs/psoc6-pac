#[doc = "Reader of register PORT_SEL0"]
pub type R = crate::R<u32, super::PORT_SEL0>;
#[doc = "Writer for register PORT_SEL0"]
pub type W = crate::W<u32, super::PORT_SEL0>;
#[doc = "Register PORT_SEL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PORT_SEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects connection for IO pin 0 route.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "0: GPIO controls 'out'"]
    GPIO = 0,
    #[doc = "1: GPIO controls 'out', DSI controls 'output enable'"]
    GPIO_DSI = 1,
    #[doc = "2: DSI controls 'out' and 'output enable'"]
    DSI_DSI = 2,
    #[doc = "3: DSI controls 'out', GPIO controls 'output enable'"]
    DSI_GPIO = 3,
    #[doc = "4: Analog mux bus A"]
    AMUXA = 4,
    #[doc = "5: Analog mux bus B"]
    AMUXB = 5,
    #[doc = "6: Analog mux bus A, DSI control"]
    AMUXA_DSI = 6,
    #[doc = "7: Analog mux bus B, DSI control"]
    AMUXB_DSI = 7,
    #[doc = "8: Active functionality 0"]
    ACT_0 = 8,
    #[doc = "9: Active functionality 1"]
    ACT_1 = 9,
    #[doc = "10: Active functionality 2"]
    ACT_2 = 10,
    #[doc = "11: Active functionality 3"]
    ACT_3 = 11,
    #[doc = "12: DeepSleep functionality 0"]
    DS_0 = 12,
    #[doc = "13: DeepSleep functionality 1"]
    DS_1 = 13,
    #[doc = "14: DeepSleep functionality 2"]
    DS_2 = 14,
    #[doc = "15: DeepSleep functionality 3"]
    DS_3 = 15,
    #[doc = "16: Active functionality 4"]
    ACT_4 = 16,
    #[doc = "17: Active functionality 5"]
    ACT_5 = 17,
    #[doc = "18: Active functionality 6"]
    ACT_6 = 18,
    #[doc = "19: Active functionality 7"]
    ACT_7 = 19,
    #[doc = "20: Active functionality 8"]
    ACT_8 = 20,
    #[doc = "21: Active functionality 9"]
    ACT_9 = 21,
    #[doc = "22: Active functionality 10"]
    ACT_10 = 22,
    #[doc = "23: Active functionality 11"]
    ACT_11 = 23,
    #[doc = "24: Active functionality 12"]
    ACT_12 = 24,
    #[doc = "25: Active functionality 13"]
    ACT_13 = 25,
    #[doc = "26: Active functionality 14"]
    ACT_14 = 26,
    #[doc = "27: Active functionality 15"]
    ACT_15 = 27,
    #[doc = "28: DeepSleep functionality 4"]
    DS_4 = 28,
    #[doc = "29: DeepSleep functionality 5"]
    DS_5 = 29,
    #[doc = "30: DeepSleep functionality 6"]
    DS_6 = 30,
    #[doc = "31: DeepSleep functionality 7"]
    DS_7 = 31,
}
impl From<IO0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IO0_SEL`"]
pub type IO0_SEL_R = crate::R<u8, IO0_SEL_A>;
impl IO0_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_SEL_A {
        match self.bits {
            0 => IO0_SEL_A::GPIO,
            1 => IO0_SEL_A::GPIO_DSI,
            2 => IO0_SEL_A::DSI_DSI,
            3 => IO0_SEL_A::DSI_GPIO,
            4 => IO0_SEL_A::AMUXA,
            5 => IO0_SEL_A::AMUXB,
            6 => IO0_SEL_A::AMUXA_DSI,
            7 => IO0_SEL_A::AMUXB_DSI,
            8 => IO0_SEL_A::ACT_0,
            9 => IO0_SEL_A::ACT_1,
            10 => IO0_SEL_A::ACT_2,
            11 => IO0_SEL_A::ACT_3,
            12 => IO0_SEL_A::DS_0,
            13 => IO0_SEL_A::DS_1,
            14 => IO0_SEL_A::DS_2,
            15 => IO0_SEL_A::DS_3,
            16 => IO0_SEL_A::ACT_4,
            17 => IO0_SEL_A::ACT_5,
            18 => IO0_SEL_A::ACT_6,
            19 => IO0_SEL_A::ACT_7,
            20 => IO0_SEL_A::ACT_8,
            21 => IO0_SEL_A::ACT_9,
            22 => IO0_SEL_A::ACT_10,
            23 => IO0_SEL_A::ACT_11,
            24 => IO0_SEL_A::ACT_12,
            25 => IO0_SEL_A::ACT_13,
            26 => IO0_SEL_A::ACT_14,
            27 => IO0_SEL_A::ACT_15,
            28 => IO0_SEL_A::DS_4,
            29 => IO0_SEL_A::DS_5,
            30 => IO0_SEL_A::DS_6,
            31 => IO0_SEL_A::DS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == IO0_SEL_A::GPIO
    }
    #[doc = "Checks if the value of the field is `GPIO_DSI`"]
    #[inline(always)]
    pub fn is_gpio_dsi(&self) -> bool {
        *self == IO0_SEL_A::GPIO_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_DSI`"]
    #[inline(always)]
    pub fn is_dsi_dsi(&self) -> bool {
        *self == IO0_SEL_A::DSI_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_GPIO`"]
    #[inline(always)]
    pub fn is_dsi_gpio(&self) -> bool {
        *self == IO0_SEL_A::DSI_GPIO
    }
    #[doc = "Checks if the value of the field is `AMUXA`"]
    #[inline(always)]
    pub fn is_amuxa(&self) -> bool {
        *self == IO0_SEL_A::AMUXA
    }
    #[doc = "Checks if the value of the field is `AMUXB`"]
    #[inline(always)]
    pub fn is_amuxb(&self) -> bool {
        *self == IO0_SEL_A::AMUXB
    }
    #[doc = "Checks if the value of the field is `AMUXA_DSI`"]
    #[inline(always)]
    pub fn is_amuxa_dsi(&self) -> bool {
        *self == IO0_SEL_A::AMUXA_DSI
    }
    #[doc = "Checks if the value of the field is `AMUXB_DSI`"]
    #[inline(always)]
    pub fn is_amuxb_dsi(&self) -> bool {
        *self == IO0_SEL_A::AMUXB_DSI
    }
    #[doc = "Checks if the value of the field is `ACT_0`"]
    #[inline(always)]
    pub fn is_act_0(&self) -> bool {
        *self == IO0_SEL_A::ACT_0
    }
    #[doc = "Checks if the value of the field is `ACT_1`"]
    #[inline(always)]
    pub fn is_act_1(&self) -> bool {
        *self == IO0_SEL_A::ACT_1
    }
    #[doc = "Checks if the value of the field is `ACT_2`"]
    #[inline(always)]
    pub fn is_act_2(&self) -> bool {
        *self == IO0_SEL_A::ACT_2
    }
    #[doc = "Checks if the value of the field is `ACT_3`"]
    #[inline(always)]
    pub fn is_act_3(&self) -> bool {
        *self == IO0_SEL_A::ACT_3
    }
    #[doc = "Checks if the value of the field is `DS_0`"]
    #[inline(always)]
    pub fn is_ds_0(&self) -> bool {
        *self == IO0_SEL_A::DS_0
    }
    #[doc = "Checks if the value of the field is `DS_1`"]
    #[inline(always)]
    pub fn is_ds_1(&self) -> bool {
        *self == IO0_SEL_A::DS_1
    }
    #[doc = "Checks if the value of the field is `DS_2`"]
    #[inline(always)]
    pub fn is_ds_2(&self) -> bool {
        *self == IO0_SEL_A::DS_2
    }
    #[doc = "Checks if the value of the field is `DS_3`"]
    #[inline(always)]
    pub fn is_ds_3(&self) -> bool {
        *self == IO0_SEL_A::DS_3
    }
    #[doc = "Checks if the value of the field is `ACT_4`"]
    #[inline(always)]
    pub fn is_act_4(&self) -> bool {
        *self == IO0_SEL_A::ACT_4
    }
    #[doc = "Checks if the value of the field is `ACT_5`"]
    #[inline(always)]
    pub fn is_act_5(&self) -> bool {
        *self == IO0_SEL_A::ACT_5
    }
    #[doc = "Checks if the value of the field is `ACT_6`"]
    #[inline(always)]
    pub fn is_act_6(&self) -> bool {
        *self == IO0_SEL_A::ACT_6
    }
    #[doc = "Checks if the value of the field is `ACT_7`"]
    #[inline(always)]
    pub fn is_act_7(&self) -> bool {
        *self == IO0_SEL_A::ACT_7
    }
    #[doc = "Checks if the value of the field is `ACT_8`"]
    #[inline(always)]
    pub fn is_act_8(&self) -> bool {
        *self == IO0_SEL_A::ACT_8
    }
    #[doc = "Checks if the value of the field is `ACT_9`"]
    #[inline(always)]
    pub fn is_act_9(&self) -> bool {
        *self == IO0_SEL_A::ACT_9
    }
    #[doc = "Checks if the value of the field is `ACT_10`"]
    #[inline(always)]
    pub fn is_act_10(&self) -> bool {
        *self == IO0_SEL_A::ACT_10
    }
    #[doc = "Checks if the value of the field is `ACT_11`"]
    #[inline(always)]
    pub fn is_act_11(&self) -> bool {
        *self == IO0_SEL_A::ACT_11
    }
    #[doc = "Checks if the value of the field is `ACT_12`"]
    #[inline(always)]
    pub fn is_act_12(&self) -> bool {
        *self == IO0_SEL_A::ACT_12
    }
    #[doc = "Checks if the value of the field is `ACT_13`"]
    #[inline(always)]
    pub fn is_act_13(&self) -> bool {
        *self == IO0_SEL_A::ACT_13
    }
    #[doc = "Checks if the value of the field is `ACT_14`"]
    #[inline(always)]
    pub fn is_act_14(&self) -> bool {
        *self == IO0_SEL_A::ACT_14
    }
    #[doc = "Checks if the value of the field is `ACT_15`"]
    #[inline(always)]
    pub fn is_act_15(&self) -> bool {
        *self == IO0_SEL_A::ACT_15
    }
    #[doc = "Checks if the value of the field is `DS_4`"]
    #[inline(always)]
    pub fn is_ds_4(&self) -> bool {
        *self == IO0_SEL_A::DS_4
    }
    #[doc = "Checks if the value of the field is `DS_5`"]
    #[inline(always)]
    pub fn is_ds_5(&self) -> bool {
        *self == IO0_SEL_A::DS_5
    }
    #[doc = "Checks if the value of the field is `DS_6`"]
    #[inline(always)]
    pub fn is_ds_6(&self) -> bool {
        *self == IO0_SEL_A::DS_6
    }
    #[doc = "Checks if the value of the field is `DS_7`"]
    #[inline(always)]
    pub fn is_ds_7(&self) -> bool {
        *self == IO0_SEL_A::DS_7
    }
}
#[doc = "Write proxy for field `IO0_SEL`"]
pub struct IO0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO0_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GPIO controls 'out'"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO)
    }
    #[doc = "GPIO controls 'out', DSI controls 'output enable'"]
    #[inline(always)]
    pub fn gpio_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO_DSI)
    }
    #[doc = "DSI controls 'out' and 'output enable'"]
    #[inline(always)]
    pub fn dsi_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_DSI)
    }
    #[doc = "DSI controls 'out', GPIO controls 'output enable'"]
    #[inline(always)]
    pub fn dsi_gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_GPIO)
    }
    #[doc = "Analog mux bus A"]
    #[inline(always)]
    pub fn amuxa(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA)
    }
    #[doc = "Analog mux bus B"]
    #[inline(always)]
    pub fn amuxb(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB)
    }
    #[doc = "Analog mux bus A, DSI control"]
    #[inline(always)]
    pub fn amuxa_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA_DSI)
    }
    #[doc = "Analog mux bus B, DSI control"]
    #[inline(always)]
    pub fn amuxb_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB_DSI)
    }
    #[doc = "Active functionality 0"]
    #[inline(always)]
    pub fn act_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_0)
    }
    #[doc = "Active functionality 1"]
    #[inline(always)]
    pub fn act_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_1)
    }
    #[doc = "Active functionality 2"]
    #[inline(always)]
    pub fn act_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_2)
    }
    #[doc = "Active functionality 3"]
    #[inline(always)]
    pub fn act_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_3)
    }
    #[doc = "DeepSleep functionality 0"]
    #[inline(always)]
    pub fn ds_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_0)
    }
    #[doc = "DeepSleep functionality 1"]
    #[inline(always)]
    pub fn ds_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_1)
    }
    #[doc = "DeepSleep functionality 2"]
    #[inline(always)]
    pub fn ds_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_2)
    }
    #[doc = "DeepSleep functionality 3"]
    #[inline(always)]
    pub fn ds_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_3)
    }
    #[doc = "Active functionality 4"]
    #[inline(always)]
    pub fn act_4(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_4)
    }
    #[doc = "Active functionality 5"]
    #[inline(always)]
    pub fn act_5(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_5)
    }
    #[doc = "Active functionality 6"]
    #[inline(always)]
    pub fn act_6(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_6)
    }
    #[doc = "Active functionality 7"]
    #[inline(always)]
    pub fn act_7(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_7)
    }
    #[doc = "Active functionality 8"]
    #[inline(always)]
    pub fn act_8(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_8)
    }
    #[doc = "Active functionality 9"]
    #[inline(always)]
    pub fn act_9(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_9)
    }
    #[doc = "Active functionality 10"]
    #[inline(always)]
    pub fn act_10(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_10)
    }
    #[doc = "Active functionality 11"]
    #[inline(always)]
    pub fn act_11(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_11)
    }
    #[doc = "Active functionality 12"]
    #[inline(always)]
    pub fn act_12(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_12)
    }
    #[doc = "Active functionality 13"]
    #[inline(always)]
    pub fn act_13(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_13)
    }
    #[doc = "Active functionality 14"]
    #[inline(always)]
    pub fn act_14(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_14)
    }
    #[doc = "Active functionality 15"]
    #[inline(always)]
    pub fn act_15(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_15)
    }
    #[doc = "DeepSleep functionality 4"]
    #[inline(always)]
    pub fn ds_4(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_4)
    }
    #[doc = "DeepSleep functionality 5"]
    #[inline(always)]
    pub fn ds_5(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_5)
    }
    #[doc = "DeepSleep functionality 6"]
    #[inline(always)]
    pub fn ds_6(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_6)
    }
    #[doc = "DeepSleep functionality 7"]
    #[inline(always)]
    pub fn ds_7(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `IO1_SEL`"]
pub type IO1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO1_SEL`"]
pub struct IO1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `IO2_SEL`"]
pub type IO2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO2_SEL`"]
pub struct IO2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IO3_SEL`"]
pub type IO3_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO3_SEL`"]
pub struct IO3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects connection for IO pin 0 route."]
    #[inline(always)]
    pub fn io0_sel(&self) -> IO0_SEL_R {
        IO0_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 1 route."]
    #[inline(always)]
    pub fn io1_sel(&self) -> IO1_SEL_R {
        IO1_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 2 route."]
    #[inline(always)]
    pub fn io2_sel(&self) -> IO2_SEL_R {
        IO2_SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 3 route."]
    #[inline(always)]
    pub fn io3_sel(&self) -> IO3_SEL_R {
        IO3_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects connection for IO pin 0 route."]
    #[inline(always)]
    pub fn io0_sel(&mut self) -> IO0_SEL_W {
        IO0_SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 1 route."]
    #[inline(always)]
    pub fn io1_sel(&mut self) -> IO1_SEL_W {
        IO1_SEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 2 route."]
    #[inline(always)]
    pub fn io2_sel(&mut self) -> IO2_SEL_W {
        IO2_SEL_W { w: self }
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 3 route."]
    #[inline(always)]
    pub fn io3_sel(&mut self) -> IO3_SEL_W {
        IO3_SEL_W { w: self }
    }
}
