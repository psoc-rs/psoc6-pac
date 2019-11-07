#[doc = "Reader of register MUX_SWITCH0"]
pub type R = crate::R<u32, super::MUX_SWITCH0>;
#[doc = "Writer for register MUX_SWITCH0"]
pub type W = crate::W<u32, super::MUX_SWITCH0>;
#[doc = "Register MUX_SWITCH0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX_SWITCH0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUX_FW_P0_VPLUS`"]
pub type MUX_FW_P0_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P0_VPLUS`"]
pub struct MUX_FW_P0_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P0_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P1_VPLUS`"]
pub type MUX_FW_P1_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P1_VPLUS`"]
pub struct MUX_FW_P1_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P1_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P2_VPLUS`"]
pub type MUX_FW_P2_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P2_VPLUS`"]
pub struct MUX_FW_P2_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P2_VPLUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P3_VPLUS`"]
pub type MUX_FW_P3_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P3_VPLUS`"]
pub struct MUX_FW_P3_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P3_VPLUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P4_VPLUS`"]
pub type MUX_FW_P4_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P4_VPLUS`"]
pub struct MUX_FW_P4_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P5_VPLUS`"]
pub type MUX_FW_P5_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P5_VPLUS`"]
pub struct MUX_FW_P5_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P6_VPLUS`"]
pub type MUX_FW_P6_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P6_VPLUS`"]
pub struct MUX_FW_P6_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P7_VPLUS`"]
pub type MUX_FW_P7_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P7_VPLUS`"]
pub struct MUX_FW_P7_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_VPLUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P0_VMINUS`"]
pub type MUX_FW_P0_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P0_VMINUS`"]
pub struct MUX_FW_P0_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P0_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P1_VMINUS`"]
pub type MUX_FW_P1_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P1_VMINUS`"]
pub struct MUX_FW_P1_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P1_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P2_VMINUS`"]
pub type MUX_FW_P2_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P2_VMINUS`"]
pub struct MUX_FW_P2_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P2_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P3_VMINUS`"]
pub type MUX_FW_P3_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P3_VMINUS`"]
pub struct MUX_FW_P3_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P3_VMINUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P4_VMINUS`"]
pub type MUX_FW_P4_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P4_VMINUS`"]
pub struct MUX_FW_P4_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P5_VMINUS`"]
pub type MUX_FW_P5_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P5_VMINUS`"]
pub struct MUX_FW_P5_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P6_VMINUS`"]
pub type MUX_FW_P6_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P6_VMINUS`"]
pub struct MUX_FW_P6_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P7_VMINUS`"]
pub type MUX_FW_P7_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P7_VMINUS`"]
pub struct MUX_FW_P7_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_VMINUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_VSSA_VMINUS`"]
pub type MUX_FW_VSSA_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_VSSA_VMINUS`"]
pub struct MUX_FW_VSSA_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_VSSA_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_TEMP_VPLUS`"]
pub type MUX_FW_TEMP_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_TEMP_VPLUS`"]
pub struct MUX_FW_TEMP_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_TEMP_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_AMUXBUSA_VPLUS`"]
pub type MUX_FW_AMUXBUSA_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_AMUXBUSA_VPLUS`"]
pub struct MUX_FW_AMUXBUSA_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSA_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_AMUXBUSB_VPLUS`"]
pub type MUX_FW_AMUXBUSB_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_AMUXBUSB_VPLUS`"]
pub struct MUX_FW_AMUXBUSB_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSB_VPLUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_AMUXBUSA_VMINUS`"]
pub type MUX_FW_AMUXBUSA_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_AMUXBUSA_VMINUS`"]
pub struct MUX_FW_AMUXBUSA_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSA_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_AMUXBUSB_VMINUS`"]
pub type MUX_FW_AMUXBUSB_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_AMUXBUSB_VMINUS`"]
pub struct MUX_FW_AMUXBUSB_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSB_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_SARBUS0_VPLUS`"]
pub type MUX_FW_SARBUS0_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_SARBUS0_VPLUS`"]
pub struct MUX_FW_SARBUS0_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS0_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_SARBUS1_VPLUS`"]
pub type MUX_FW_SARBUS1_VPLUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_SARBUS1_VPLUS`"]
pub struct MUX_FW_SARBUS1_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS1_VPLUS_W<'a> {
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
#[doc = "Reader of field `MUX_FW_SARBUS0_VMINUS`"]
pub type MUX_FW_SARBUS0_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_SARBUS0_VMINUS`"]
pub struct MUX_FW_SARBUS0_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS0_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_SARBUS1_VMINUS`"]
pub type MUX_FW_SARBUS1_VMINUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_SARBUS1_VMINUS`"]
pub struct MUX_FW_SARBUS1_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS1_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P4_COREIO0`"]
pub type MUX_FW_P4_COREIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P4_COREIO0`"]
pub struct MUX_FW_P4_COREIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_COREIO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MUX_FW_P5_COREIO1`"]
pub type MUX_FW_P5_COREIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P5_COREIO1`"]
pub struct MUX_FW_P5_COREIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_COREIO1_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P6_COREIO2`"]
pub type MUX_FW_P6_COREIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P6_COREIO2`"]
pub struct MUX_FW_P6_COREIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_COREIO2_W<'a> {
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
#[doc = "Reader of field `MUX_FW_P7_COREIO3`"]
pub type MUX_FW_P7_COREIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_FW_P7_COREIO3`"]
pub struct MUX_FW_P7_COREIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_COREIO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MUX_FW_P0_VPLUS_R {
        MUX_FW_P0_VPLUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MUX_FW_P1_VPLUS_R {
        MUX_FW_P1_VPLUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MUX_FW_P2_VPLUS_R {
        MUX_FW_P2_VPLUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MUX_FW_P3_VPLUS_R {
        MUX_FW_P3_VPLUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MUX_FW_P4_VPLUS_R {
        MUX_FW_P4_VPLUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MUX_FW_P5_VPLUS_R {
        MUX_FW_P5_VPLUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MUX_FW_P6_VPLUS_R {
        MUX_FW_P6_VPLUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MUX_FW_P7_VPLUS_R {
        MUX_FW_P7_VPLUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MUX_FW_P0_VMINUS_R {
        MUX_FW_P0_VMINUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MUX_FW_P1_VMINUS_R {
        MUX_FW_P1_VMINUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MUX_FW_P2_VMINUS_R {
        MUX_FW_P2_VMINUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MUX_FW_P3_VMINUS_R {
        MUX_FW_P3_VMINUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MUX_FW_P4_VMINUS_R {
        MUX_FW_P4_VMINUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MUX_FW_P5_VMINUS_R {
        MUX_FW_P5_VMINUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MUX_FW_P6_VMINUS_R {
        MUX_FW_P6_VMINUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MUX_FW_P7_VMINUS_R {
        MUX_FW_P7_VMINUS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MUX_FW_VSSA_VMINUS_R {
        MUX_FW_VSSA_VMINUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MUX_FW_TEMP_VPLUS_R {
        MUX_FW_TEMP_VPLUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MUX_FW_AMUXBUSA_VPLUS_R {
        MUX_FW_AMUXBUSA_VPLUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MUX_FW_AMUXBUSB_VPLUS_R {
        MUX_FW_AMUXBUSB_VPLUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MUX_FW_AMUXBUSA_VMINUS_R {
        MUX_FW_AMUXBUSA_VMINUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MUX_FW_AMUXBUSB_VMINUS_R {
        MUX_FW_AMUXBUSB_VMINUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MUX_FW_SARBUS0_VPLUS_R {
        MUX_FW_SARBUS0_VPLUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MUX_FW_SARBUS1_VPLUS_R {
        MUX_FW_SARBUS1_VPLUS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MUX_FW_SARBUS0_VMINUS_R {
        MUX_FW_SARBUS0_VMINUS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MUX_FW_SARBUS1_VMINUS_R {
        MUX_FW_SARBUS1_VMINUS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&self) -> MUX_FW_P4_COREIO0_R {
        MUX_FW_P4_COREIO0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&self) -> MUX_FW_P5_COREIO1_R {
        MUX_FW_P5_COREIO1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&self) -> MUX_FW_P6_COREIO2_R {
        MUX_FW_P6_COREIO2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&self) -> MUX_FW_P7_COREIO3_R {
        MUX_FW_P7_COREIO3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&mut self) -> MUX_FW_P0_VPLUS_W {
        MUX_FW_P0_VPLUS_W { w: self }
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&mut self) -> MUX_FW_P1_VPLUS_W {
        MUX_FW_P1_VPLUS_W { w: self }
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&mut self) -> MUX_FW_P2_VPLUS_W {
        MUX_FW_P2_VPLUS_W { w: self }
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&mut self) -> MUX_FW_P3_VPLUS_W {
        MUX_FW_P3_VPLUS_W { w: self }
    }
    #[doc = "Bit 4 - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&mut self) -> MUX_FW_P4_VPLUS_W {
        MUX_FW_P4_VPLUS_W { w: self }
    }
    #[doc = "Bit 5 - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&mut self) -> MUX_FW_P5_VPLUS_W {
        MUX_FW_P5_VPLUS_W { w: self }
    }
    #[doc = "Bit 6 - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&mut self) -> MUX_FW_P6_VPLUS_W {
        MUX_FW_P6_VPLUS_W { w: self }
    }
    #[doc = "Bit 7 - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&mut self) -> MUX_FW_P7_VPLUS_W {
        MUX_FW_P7_VPLUS_W { w: self }
    }
    #[doc = "Bit 8 - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&mut self) -> MUX_FW_P0_VMINUS_W {
        MUX_FW_P0_VMINUS_W { w: self }
    }
    #[doc = "Bit 9 - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&mut self) -> MUX_FW_P1_VMINUS_W {
        MUX_FW_P1_VMINUS_W { w: self }
    }
    #[doc = "Bit 10 - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&mut self) -> MUX_FW_P2_VMINUS_W {
        MUX_FW_P2_VMINUS_W { w: self }
    }
    #[doc = "Bit 11 - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&mut self) -> MUX_FW_P3_VMINUS_W {
        MUX_FW_P3_VMINUS_W { w: self }
    }
    #[doc = "Bit 12 - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&mut self) -> MUX_FW_P4_VMINUS_W {
        MUX_FW_P4_VMINUS_W { w: self }
    }
    #[doc = "Bit 13 - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&mut self) -> MUX_FW_P5_VMINUS_W {
        MUX_FW_P5_VMINUS_W { w: self }
    }
    #[doc = "Bit 14 - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&mut self) -> MUX_FW_P6_VMINUS_W {
        MUX_FW_P6_VMINUS_W { w: self }
    }
    #[doc = "Bit 15 - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&mut self) -> MUX_FW_P7_VMINUS_W {
        MUX_FW_P7_VMINUS_W { w: self }
    }
    #[doc = "Bit 16 - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&mut self) -> MUX_FW_VSSA_VMINUS_W {
        MUX_FW_VSSA_VMINUS_W { w: self }
    }
    #[doc = "Bit 17 - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&mut self) -> MUX_FW_TEMP_VPLUS_W {
        MUX_FW_TEMP_VPLUS_W { w: self }
    }
    #[doc = "Bit 18 - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&mut self) -> MUX_FW_AMUXBUSA_VPLUS_W {
        MUX_FW_AMUXBUSA_VPLUS_W { w: self }
    }
    #[doc = "Bit 19 - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&mut self) -> MUX_FW_AMUXBUSB_VPLUS_W {
        MUX_FW_AMUXBUSB_VPLUS_W { w: self }
    }
    #[doc = "Bit 20 - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&mut self) -> MUX_FW_AMUXBUSA_VMINUS_W {
        MUX_FW_AMUXBUSA_VMINUS_W { w: self }
    }
    #[doc = "Bit 21 - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&mut self) -> MUX_FW_AMUXBUSB_VMINUS_W {
        MUX_FW_AMUXBUSB_VMINUS_W { w: self }
    }
    #[doc = "Bit 22 - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&mut self) -> MUX_FW_SARBUS0_VPLUS_W {
        MUX_FW_SARBUS0_VPLUS_W { w: self }
    }
    #[doc = "Bit 23 - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&mut self) -> MUX_FW_SARBUS1_VPLUS_W {
        MUX_FW_SARBUS1_VPLUS_W { w: self }
    }
    #[doc = "Bit 24 - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&mut self) -> MUX_FW_SARBUS0_VMINUS_W {
        MUX_FW_SARBUS0_VMINUS_W { w: self }
    }
    #[doc = "Bit 25 - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&mut self) -> MUX_FW_SARBUS1_VMINUS_W {
        MUX_FW_SARBUS1_VMINUS_W { w: self }
    }
    #[doc = "Bit 26 - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&mut self) -> MUX_FW_P4_COREIO0_W {
        MUX_FW_P4_COREIO0_W { w: self }
    }
    #[doc = "Bit 27 - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&mut self) -> MUX_FW_P5_COREIO1_W {
        MUX_FW_P5_COREIO1_W { w: self }
    }
    #[doc = "Bit 28 - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&mut self) -> MUX_FW_P6_COREIO2_W {
        MUX_FW_P6_COREIO2_W { w: self }
    }
    #[doc = "Bit 29 - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&mut self) -> MUX_FW_P7_COREIO3_W {
        MUX_FW_P7_COREIO3_W { w: self }
    }
}
