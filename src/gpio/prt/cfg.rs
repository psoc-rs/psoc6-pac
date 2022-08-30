#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_MODE0_A {
    #[doc = "0: Output buffer is off creating a high impedance input D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    HIGHZ = 0,
    #[doc = "1: N/A"]
    RSVD = 1,
    #[doc = "2: Resistive pull up For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Weak/resistive pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull up D_OUT = '1': Weak/resistive pull up"]
    PULLUP = 2,
    #[doc = "3: Resistive pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull down"]
    PULLDOWN = 3,
    #[doc = "4: Open drain, drives low For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': High Impedance When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OD_DRIVESLOW = 4,
    #[doc = "5: Open drain, drives high For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': High Impedance D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    OD_DRIVESHIGH = 5,
    #[doc = "6: Strong D_OUTput buffer For GPIO & UDB/DSI peripherals: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High impedance D_OUT = '1': High impedance For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': High Impedance D_OUT = '1': High Impedance"]
    STRONG = 6,
    #[doc = "7: Pull up or pull down For GPIO & UDB/DSI peripherals: When D_OUT_EN = '0': GPIO_DSI_OUT = '0': Weak/resistive pull down GPIO_DSI_OUT = '1': Weak/resistive pull up where 'GPIO_DSI_OUT' is a function of PORT_SEL, OUT & DSI_DATA_OUT. For peripherals other than GPIO & UDB/DSI: When D_OUT_EN = 1: D_OUT = '0': Strong pull down D_OUT = '1': Strong pull up When D_OUT_EN = 0: D_OUT = '0': Weak/resistive pull down D_OUT = '1': Weak/resistive pull up"]
    PULLUP_DOWN = 7,
}
impl From<DRIVE_MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_MODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRIVE_MODE0` reader - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
pub type DRIVE_MODE0_R = crate::FieldReader<u8, DRIVE_MODE0_A>;
impl DRIVE_MODE0_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `DRIVE_MODE0` writer - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
pub type DRIVE_MODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, DRIVE_MODE0_A, 3, O>;
impl<'a, const O: u8> DRIVE_MODE0_W<'a, O> {
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
}
#[doc = "Field `IN_EN0` reader - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
pub type IN_EN0_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN0` writer - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
pub type IN_EN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE1` reader - The GPIO drive mode for IO pin 1"]
pub type DRIVE_MODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE1` writer - The GPIO drive mode for IO pin 1"]
pub type DRIVE_MODE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN1` reader - Enables the input buffer for IO pin 1"]
pub type IN_EN1_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN1` writer - Enables the input buffer for IO pin 1"]
pub type IN_EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE2` reader - The GPIO drive mode for IO pin 2"]
pub type DRIVE_MODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE2` writer - The GPIO drive mode for IO pin 2"]
pub type DRIVE_MODE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN2` reader - Enables the input buffer for IO pin 2"]
pub type IN_EN2_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN2` writer - Enables the input buffer for IO pin 2"]
pub type IN_EN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE3` reader - The GPIO drive mode for IO pin 3"]
pub type DRIVE_MODE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE3` writer - The GPIO drive mode for IO pin 3"]
pub type DRIVE_MODE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN3` reader - Enables the input buffer for IO pin 3"]
pub type IN_EN3_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN3` writer - Enables the input buffer for IO pin 3"]
pub type IN_EN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE4` reader - The GPIO drive mode for IO pin4"]
pub type DRIVE_MODE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE4` writer - The GPIO drive mode for IO pin4"]
pub type DRIVE_MODE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN4` reader - Enables the input buffer for IO pin 4"]
pub type IN_EN4_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN4` writer - Enables the input buffer for IO pin 4"]
pub type IN_EN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE5` reader - The GPIO drive mode for IO pin 5"]
pub type DRIVE_MODE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE5` writer - The GPIO drive mode for IO pin 5"]
pub type DRIVE_MODE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN5` reader - Enables the input buffer for IO pin 5"]
pub type IN_EN5_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN5` writer - Enables the input buffer for IO pin 5"]
pub type IN_EN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE6` reader - The GPIO drive mode for IO pin 6"]
pub type DRIVE_MODE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE6` writer - The GPIO drive mode for IO pin 6"]
pub type DRIVE_MODE6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN6` reader - Enables the input buffer for IO pin 6"]
pub type IN_EN6_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN6` writer - Enables the input buffer for IO pin 6"]
pub type IN_EN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DRIVE_MODE7` reader - The GPIO drive mode for IO pin 7"]
pub type DRIVE_MODE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIVE_MODE7` writer - The GPIO drive mode for IO pin 7"]
pub type DRIVE_MODE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_EN7` reader - Enables the input buffer for IO pin 7"]
pub type IN_EN7_R = crate::BitReader<bool>;
#[doc = "Field `IN_EN7` writer - Enables the input buffer for IO pin 7"]
pub type IN_EN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn drive_mode0(&self) -> DRIVE_MODE0_R {
        DRIVE_MODE0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn in_en0(&self) -> IN_EN0_R {
        IN_EN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn drive_mode1(&self) -> DRIVE_MODE1_R {
        DRIVE_MODE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn in_en1(&self) -> IN_EN1_R {
        IN_EN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn drive_mode2(&self) -> DRIVE_MODE2_R {
        DRIVE_MODE2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn in_en2(&self) -> IN_EN2_R {
        IN_EN2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn drive_mode3(&self) -> DRIVE_MODE3_R {
        DRIVE_MODE3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn in_en3(&self) -> IN_EN3_R {
        IN_EN3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn drive_mode4(&self) -> DRIVE_MODE4_R {
        DRIVE_MODE4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn in_en4(&self) -> IN_EN4_R {
        IN_EN4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn drive_mode5(&self) -> DRIVE_MODE5_R {
        DRIVE_MODE5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn in_en5(&self) -> IN_EN5_R {
        IN_EN5_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn drive_mode6(&self) -> DRIVE_MODE6_R {
        DRIVE_MODE6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn in_en6(&self) -> IN_EN6_R {
        IN_EN6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn drive_mode7(&self) -> DRIVE_MODE7_R {
        DRIVE_MODE7_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn in_en7(&self) -> IN_EN7_R {
        IN_EN7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO drive mode for IO pin 0. Resistive pull-up and pull-down is selected in the drive mode. Note: when initializing IO's that are connected to a live bus (such as I2C), make sure the peripheral and HSIOM (HSIOM_PRT_SELx) is properly configured before turning the IO on here to avoid producing glitches on the bus. Note: that peripherals other than GPIO & UDB/DSI directly control both the output and output-enable of the output buffer (peripherals can drive strong 0 or strong 1 in any mode except OFF='0'). Note: D_OUT, D_OUT_EN are pins of GPIO cell."]
    #[inline(always)]
    pub fn drive_mode0(&mut self) -> DRIVE_MODE0_W<0> {
        DRIVE_MODE0_W::new(self)
    }
    #[doc = "Bit 3 - Enables the input buffer for IO pin 0. This bit should be cleared when analog signals are present on the pin to avoid crowbar currents. The output buffer can be used to drive analog signals high or low without issue. '0': Input buffer disabled '1': Input buffer enabled"]
    #[inline(always)]
    pub fn in_en0(&mut self) -> IN_EN0_W<3> {
        IN_EN0_W::new(self)
    }
    #[doc = "Bits 4:6 - The GPIO drive mode for IO pin 1"]
    #[inline(always)]
    pub fn drive_mode1(&mut self) -> DRIVE_MODE1_W<4> {
        DRIVE_MODE1_W::new(self)
    }
    #[doc = "Bit 7 - Enables the input buffer for IO pin 1"]
    #[inline(always)]
    pub fn in_en1(&mut self) -> IN_EN1_W<7> {
        IN_EN1_W::new(self)
    }
    #[doc = "Bits 8:10 - The GPIO drive mode for IO pin 2"]
    #[inline(always)]
    pub fn drive_mode2(&mut self) -> DRIVE_MODE2_W<8> {
        DRIVE_MODE2_W::new(self)
    }
    #[doc = "Bit 11 - Enables the input buffer for IO pin 2"]
    #[inline(always)]
    pub fn in_en2(&mut self) -> IN_EN2_W<11> {
        IN_EN2_W::new(self)
    }
    #[doc = "Bits 12:14 - The GPIO drive mode for IO pin 3"]
    #[inline(always)]
    pub fn drive_mode3(&mut self) -> DRIVE_MODE3_W<12> {
        DRIVE_MODE3_W::new(self)
    }
    #[doc = "Bit 15 - Enables the input buffer for IO pin 3"]
    #[inline(always)]
    pub fn in_en3(&mut self) -> IN_EN3_W<15> {
        IN_EN3_W::new(self)
    }
    #[doc = "Bits 16:18 - The GPIO drive mode for IO pin4"]
    #[inline(always)]
    pub fn drive_mode4(&mut self) -> DRIVE_MODE4_W<16> {
        DRIVE_MODE4_W::new(self)
    }
    #[doc = "Bit 19 - Enables the input buffer for IO pin 4"]
    #[inline(always)]
    pub fn in_en4(&mut self) -> IN_EN4_W<19> {
        IN_EN4_W::new(self)
    }
    #[doc = "Bits 20:22 - The GPIO drive mode for IO pin 5"]
    #[inline(always)]
    pub fn drive_mode5(&mut self) -> DRIVE_MODE5_W<20> {
        DRIVE_MODE5_W::new(self)
    }
    #[doc = "Bit 23 - Enables the input buffer for IO pin 5"]
    #[inline(always)]
    pub fn in_en5(&mut self) -> IN_EN5_W<23> {
        IN_EN5_W::new(self)
    }
    #[doc = "Bits 24:26 - The GPIO drive mode for IO pin 6"]
    #[inline(always)]
    pub fn drive_mode6(&mut self) -> DRIVE_MODE6_W<24> {
        DRIVE_MODE6_W::new(self)
    }
    #[doc = "Bit 27 - Enables the input buffer for IO pin 6"]
    #[inline(always)]
    pub fn in_en6(&mut self) -> IN_EN6_W<27> {
        IN_EN6_W::new(self)
    }
    #[doc = "Bits 28:30 - The GPIO drive mode for IO pin 7"]
    #[inline(always)]
    pub fn drive_mode7(&mut self) -> DRIVE_MODE7_W<28> {
        DRIVE_MODE7_W::new(self)
    }
    #[doc = "Bit 31 - Enables the input buffer for IO pin 7"]
    #[inline(always)]
    pub fn in_en7(&mut self) -> IN_EN7_W<31> {
        IN_EN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
