#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_MODE0_A {
    #[doc = "0: Output buffer is off creating a high impedance input\nD_OUT = '0': High Impedance\nD_OUT = '1': High Impedance"]
    HIGHZ = 0,
    #[doc = "1: N/A"]
    RSVD = 1,
    #[doc = "2: Resistive pull up\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Weak/resistive pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High impedance\n   D_OUT = '1': High impedance\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': Weak/resistive pull up\n   D_OUT = '1': Weak/resistive pull up"]
    PULLUP = 2,
    #[doc = "3: Resistive pull down\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Weak/resistive pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High impedance\n   D_OUT = '1': High impedance\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': Weak/resistive pull down\n   D_OUT = '1': Weak/resistive pull down"]
    PULLDOWN = 3,
    #[doc = "4: Open drain, drives low\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': High Impedance\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High impedance\n   D_OUT = '1': High impedance\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High Impedance\n   D_OUT = '1': High Impedance"]
    OD_DRIVESLOW = 4,
    #[doc = "5: Open drain, drives high\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': High Impedance\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High impedance\n   D_OUT = '1': High impedance\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High Impedance\n   D_OUT = '1': High Impedance"]
    OD_DRIVESHIGH = 5,
    #[doc = "6: Strong D_OUTput buffer\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High impedance\n   D_OUT = '1': High impedance\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n   D_OUT = '0': High Impedance\n   D_OUT = '1': High Impedance"]
    STRONG = 6,
    #[doc = "7: Pull up or pull down\n\nFor GPIO & UDB/DSI peripherals:\nWhen D_OUT_EN = '0':\n    GPIO_DSI_OUT = '0': Weak/resistive pull down\n    GPIO_DSI_OUT = '1': Weak/resistive pull up\nwhere 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT & DSI_DATA_OUT.\n\nFor peripherals other than GPIO & UDB/DSI:\nWhen D_OUT_EN = 1:\n   D_OUT = '0': Strong pull down\n   D_OUT = '1': Strong pull up\nWhen D_OUT_EN = 0:\n    D_OUT = '0': Weak/resistive pull down\n    D_OUT = '1': Weak/resistive pull up"]
    PULLUP_DOWN = 7,
}
impl From<DRIVE_MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_MODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVE_MODE0`"]
pub type DRIVE_MODE0_R = crate::R<u8, DRIVE_MODE0_A>;
impl DRIVE_MODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_MODE0_A {
        match self.bits {
            0 => DRIVE_MODE0_A::HIGHZ,
            1 => DRIVE_MODE0_A::RSVD,
            2 => DRIVE_MODE0_A::PULLUP,
            3 => DRIVE_MODE0_A::PULLDOWN,
            4 => DRIVE_MODE0_A::OD_DRIVESLOW,
            5 => DRIVE_MODE0_A::OD_DRIVESHIGH,
            6 => DRIVE_MODE0_A::STRONG,
            7 => DRIVE_MODE0_A::PULLUP_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HIGHZ`"]
    #[inline(always)]
    pub fn is_highz(&self) -> bool {
        *self == DRIVE_MODE0_A::HIGHZ
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == DRIVE_MODE0_A::RSVD
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == DRIVE_MODE0_A::PULLUP
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == DRIVE_MODE0_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `OD_DRIVESLOW`"]
    #[inline(always)]
    pub fn is_od_driveslow(&self) -> bool {
        *self == DRIVE_MODE0_A::OD_DRIVESLOW
    }
    #[doc = "Checks if the value of the field is `OD_DRIVESHIGH`"]
    #[inline(always)]
    pub fn is_od_driveshigh(&self) -> bool {
        *self == DRIVE_MODE0_A::OD_DRIVESHIGH
    }
    #[doc = "Checks if the value of the field is `STRONG`"]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == DRIVE_MODE0_A::STRONG
    }
    #[doc = "Checks if the value of the field is `PULLUP_DOWN`"]
    #[inline(always)]
    pub fn is_pullup_down(&self) -> bool {
        *self == DRIVE_MODE0_A::PULLUP_DOWN
    }
}
#[doc = "Write proxy for field `DRIVE_MODE0`"]
pub struct DRIVE_MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_MODE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn highz(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::HIGHZ)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::RSVD)
    }
    #[doc = "Resistive pull up For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::PULLUP)
    }
    #[doc = "Resistive pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::PULLDOWN)
    }
    #[doc = "Open drain, drives low For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn od_driveslow(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::OD_DRIVESLOW)
    }
    #[doc = "Open drain, drives high For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn od_driveshigh(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::OD_DRIVESHIGH)
    }
    #[doc = "Strong D_OUTput buffer For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    #[inline(always)]
    pub fn strong(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::STRONG)
    }
    #[doc = "Pull up or pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT & DSI_DATA_OUT. For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    #[inline(always)]
    pub fn pullup_down(self) -> &'a mut W {
        self.variant(DRIVE_MODE0_A::PULLUP_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `IN_EN0`"]
pub type IN_EN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN0`"]
pub struct IN_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE1`"]
pub type DRIVE_MODE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE1`"]
pub struct DRIVE_MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `IN_EN1`"]
pub type IN_EN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN1`"]
pub struct IN_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE2`"]
pub type DRIVE_MODE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE2`"]
pub struct DRIVE_MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `IN_EN2`"]
pub type IN_EN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN2`"]
pub struct IN_EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE3`"]
pub type DRIVE_MODE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE3`"]
pub struct DRIVE_MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `IN_EN3`"]
pub type IN_EN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN3`"]
pub struct IN_EN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE4`"]
pub type DRIVE_MODE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE4`"]
pub struct DRIVE_MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `IN_EN4`"]
pub type IN_EN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN4`"]
pub struct IN_EN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE5`"]
pub type DRIVE_MODE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE5`"]
pub struct DRIVE_MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `IN_EN5`"]
pub type IN_EN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN5`"]
pub struct IN_EN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE6`"]
pub type DRIVE_MODE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE6`"]
pub struct DRIVE_MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `IN_EN6`"]
pub type IN_EN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN6`"]
pub struct IN_EN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DRIVE_MODE7`"]
pub type DRIVE_MODE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRIVE_MODE7`"]
pub struct DRIVE_MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_MODE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `IN_EN7`"]
pub type IN_EN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_EN7`"]
pub struct IN_EN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_EN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn drive_mode0(&self) -> DRIVE_MODE0_R {
        DRIVE_MODE0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn in_en0(&self) -> IN_EN0_R {
        IN_EN0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn drive_mode1(&self) -> DRIVE_MODE1_R {
        DRIVE_MODE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn in_en1(&self) -> IN_EN1_R {
        IN_EN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn drive_mode2(&self) -> DRIVE_MODE2_R {
        DRIVE_MODE2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn in_en2(&self) -> IN_EN2_R {
        IN_EN2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn drive_mode3(&self) -> DRIVE_MODE3_R {
        DRIVE_MODE3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn in_en3(&self) -> IN_EN3_R {
        IN_EN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn drive_mode4(&self) -> DRIVE_MODE4_R {
        DRIVE_MODE4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn in_en4(&self) -> IN_EN4_R {
        IN_EN4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn drive_mode5(&self) -> DRIVE_MODE5_R {
        DRIVE_MODE5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn in_en5(&self) -> IN_EN5_R {
        IN_EN5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn drive_mode6(&self) -> DRIVE_MODE6_R {
        DRIVE_MODE6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn in_en6(&self) -> IN_EN6_R {
        IN_EN6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn drive_mode7(&self) -> DRIVE_MODE7_R {
        DRIVE_MODE7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn in_en7(&self) -> IN_EN7_R {
        IN_EN7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn drive_mode0(&mut self) -> DRIVE_MODE0_W {
        DRIVE_MODE0_W { w: self }
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn in_en0(&mut self) -> IN_EN0_W {
        IN_EN0_W { w: self }
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn drive_mode1(&mut self) -> DRIVE_MODE1_W {
        DRIVE_MODE1_W { w: self }
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn in_en1(&mut self) -> IN_EN1_W {
        IN_EN1_W { w: self }
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn drive_mode2(&mut self) -> DRIVE_MODE2_W {
        DRIVE_MODE2_W { w: self }
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn in_en2(&mut self) -> IN_EN2_W {
        IN_EN2_W { w: self }
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn drive_mode3(&mut self) -> DRIVE_MODE3_W {
        DRIVE_MODE3_W { w: self }
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn in_en3(&mut self) -> IN_EN3_W {
        IN_EN3_W { w: self }
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn drive_mode4(&mut self) -> DRIVE_MODE4_W {
        DRIVE_MODE4_W { w: self }
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn in_en4(&mut self) -> IN_EN4_W {
        IN_EN4_W { w: self }
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn drive_mode5(&mut self) -> DRIVE_MODE5_W {
        DRIVE_MODE5_W { w: self }
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn in_en5(&mut self) -> IN_EN5_W {
        IN_EN5_W { w: self }
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn drive_mode6(&mut self) -> DRIVE_MODE6_W {
        DRIVE_MODE6_W { w: self }
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn in_en6(&mut self) -> IN_EN6_W {
        IN_EN6_W { w: self }
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn drive_mode7(&mut self) -> DRIVE_MODE7_W {
        DRIVE_MODE7_W { w: self }
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn in_en7(&mut self) -> IN_EN7_W {
        IN_EN7_W { w: self }
    }
}
