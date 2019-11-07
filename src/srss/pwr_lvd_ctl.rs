#[doc = "Reader of register PWR_LVD_CTL"]
pub type R = crate::R<u32, super::PWR_LVD_CTL>;
#[doc = "Writer for register PWR_LVD_CTL"]
pub type W = crate::W<u32, super::PWR_LVD_CTL>;
#[doc = "Register PWR_LVD_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_LVD_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HVLVD1_TRIPSEL`"]
pub type HVLVD1_TRIPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVLVD1_TRIPSEL`"]
pub struct HVLVD1_TRIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_TRIPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Source selection for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HVLVD1_SRCSEL_A {
    #[doc = "0: Select VDDD"]
    VDDD,
    #[doc = "1: Select AMUXBUSA (VDDD branch)"]
    AMUXBUSA,
    #[doc = "2: N/A"]
    RSVD,
    #[doc = "3: N/A"]
    VDDIO,
    #[doc = "4: Select AMUXBUSB (VDDD branch)"]
    AMUXBUSB,
}
impl From<HVLVD1_SRCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVLVD1_SRCSEL_A) -> Self {
        match variant {
            HVLVD1_SRCSEL_A::VDDD => 0,
            HVLVD1_SRCSEL_A::AMUXBUSA => 1,
            HVLVD1_SRCSEL_A::RSVD => 2,
            HVLVD1_SRCSEL_A::VDDIO => 3,
            HVLVD1_SRCSEL_A::AMUXBUSB => 4,
        }
    }
}
#[doc = "Reader of field `HVLVD1_SRCSEL`"]
pub type HVLVD1_SRCSEL_R = crate::R<u8, HVLVD1_SRCSEL_A>;
impl HVLVD1_SRCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HVLVD1_SRCSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HVLVD1_SRCSEL_A::VDDD),
            1 => Val(HVLVD1_SRCSEL_A::AMUXBUSA),
            2 => Val(HVLVD1_SRCSEL_A::RSVD),
            3 => Val(HVLVD1_SRCSEL_A::VDDIO),
            4 => Val(HVLVD1_SRCSEL_A::AMUXBUSB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VDDD`"]
    #[inline(always)]
    pub fn is_vddd(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::VDDD
    }
    #[doc = "Checks if the value of the field is `AMUXBUSA`"]
    #[inline(always)]
    pub fn is_amuxbusa(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::AMUXBUSA
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::RSVD
    }
    #[doc = "Checks if the value of the field is `VDDIO`"]
    #[inline(always)]
    pub fn is_vddio(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::VDDIO
    }
    #[doc = "Checks if the value of the field is `AMUXBUSB`"]
    #[inline(always)]
    pub fn is_amuxbusb(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::AMUXBUSB
    }
}
#[doc = "Write proxy for field `HVLVD1_SRCSEL`"]
pub struct HVLVD1_SRCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_SRCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HVLVD1_SRCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select VDDD"]
    #[inline(always)]
    pub fn vddd(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::VDDD)
    }
    #[doc = "Select AMUXBUSA (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusa(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::AMUXBUSA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::RSVD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vddio(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::VDDIO)
    }
    #[doc = "Select AMUXBUSB (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusb(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::AMUXBUSB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `HVLVD1_EN`"]
pub type HVLVD1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HVLVD1_EN`"]
pub struct HVLVD1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn hvlvd1_tripsel(&self) -> HVLVD1_TRIPSEL_R {
        HVLVD1_TRIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_srcsel(&self) -> HVLVD1_SRCSEL_R {
        HVLVD1_SRCSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it further recommended to read the realted PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn hvlvd1_en(&self) -> HVLVD1_EN_R {
        HVLVD1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn hvlvd1_tripsel(&mut self) -> HVLVD1_TRIPSEL_W {
        HVLVD1_TRIPSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_srcsel(&mut self) -> HVLVD1_SRCSEL_W {
        HVLVD1_SRCSEL_W { w: self }
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it further recommended to read the realted PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn hvlvd1_en(&mut self) -> HVLVD1_EN_W {
        HVLVD1_EN_W { w: self }
    }
}
