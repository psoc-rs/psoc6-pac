#[doc = "Reader of register MUX_SWITCH_DS_CTRL"]
pub type R = crate::R<u32, super::MUX_SWITCH_DS_CTRL>;
#[doc = "Writer for register MUX_SWITCH_DS_CTRL"]
pub type W = crate::W<u32, super::MUX_SWITCH_DS_CTRL>;
#[doc = "Register MUX_SWITCH_DS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX_SWITCH_DS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MUX_DS_CTRL_P0`"]
pub type MUX_DS_CTRL_P0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P0`"]
pub struct MUX_DS_CTRL_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P0_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P1`"]
pub type MUX_DS_CTRL_P1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P1`"]
pub struct MUX_DS_CTRL_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P1_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P2`"]
pub type MUX_DS_CTRL_P2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P2`"]
pub struct MUX_DS_CTRL_P2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P2_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P3`"]
pub type MUX_DS_CTRL_P3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P3`"]
pub struct MUX_DS_CTRL_P3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P3_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P4`"]
pub type MUX_DS_CTRL_P4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P4`"]
pub struct MUX_DS_CTRL_P4_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P4_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P5`"]
pub type MUX_DS_CTRL_P5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P5`"]
pub struct MUX_DS_CTRL_P5_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P5_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P6`"]
pub type MUX_DS_CTRL_P6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P6`"]
pub struct MUX_DS_CTRL_P6_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P6_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_P7`"]
pub type MUX_DS_CTRL_P7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_P7`"]
pub struct MUX_DS_CTRL_P7_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_P7_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_VSSA`"]
pub type MUX_DS_CTRL_VSSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_VSSA`"]
pub struct MUX_DS_CTRL_VSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_VSSA_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_TEMP`"]
pub type MUX_DS_CTRL_TEMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_TEMP`"]
pub struct MUX_DS_CTRL_TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_TEMP_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_AMUXBUSA`"]
pub type MUX_DS_CTRL_AMUXBUSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_AMUXBUSA`"]
pub struct MUX_DS_CTRL_AMUXBUSA_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_AMUXBUSA_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_AMUXBUSB`"]
pub type MUX_DS_CTRL_AMUXBUSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_AMUXBUSB`"]
pub struct MUX_DS_CTRL_AMUXBUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_AMUXBUSB_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_SARBUS0`"]
pub type MUX_DS_CTRL_SARBUS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_SARBUS0`"]
pub struct MUX_DS_CTRL_SARBUS0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_SARBUS0_W<'a> {
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
#[doc = "Reader of field `MUX_DS_CTRL_SARBUS1`"]
pub type MUX_DS_CTRL_SARBUS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUX_DS_CTRL_SARBUS1`"]
pub struct MUX_DS_CTRL_SARBUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_DS_CTRL_SARBUS1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p0(&self) -> MUX_DS_CTRL_P0_R {
        MUX_DS_CTRL_P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p1(&self) -> MUX_DS_CTRL_P1_R {
        MUX_DS_CTRL_P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p2(&self) -> MUX_DS_CTRL_P2_R {
        MUX_DS_CTRL_P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p3(&self) -> MUX_DS_CTRL_P3_R {
        MUX_DS_CTRL_P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p4(&self) -> MUX_DS_CTRL_P4_R {
        MUX_DS_CTRL_P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p5(&self) -> MUX_DS_CTRL_P5_R {
        MUX_DS_CTRL_P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p6(&self) -> MUX_DS_CTRL_P6_R {
        MUX_DS_CTRL_P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p7(&self) -> MUX_DS_CTRL_P7_R {
        MUX_DS_CTRL_P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_vssa(&self) -> MUX_DS_CTRL_VSSA_R {
        MUX_DS_CTRL_VSSA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_temp(&self) -> MUX_DS_CTRL_TEMP_R {
        MUX_DS_CTRL_TEMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusa(&self) -> MUX_DS_CTRL_AMUXBUSA_R {
        MUX_DS_CTRL_AMUXBUSA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusb(&self) -> MUX_DS_CTRL_AMUXBUSB_R {
        MUX_DS_CTRL_AMUXBUSB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus0(&self) -> MUX_DS_CTRL_SARBUS0_R {
        MUX_DS_CTRL_SARBUS0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus1(&self) -> MUX_DS_CTRL_SARBUS1_R {
        MUX_DS_CTRL_SARBUS1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - for P0 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p0(&mut self) -> MUX_DS_CTRL_P0_W {
        MUX_DS_CTRL_P0_W { w: self }
    }
    #[doc = "Bit 1 - for P1 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p1(&mut self) -> MUX_DS_CTRL_P1_W {
        MUX_DS_CTRL_P1_W { w: self }
    }
    #[doc = "Bit 2 - for P2 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p2(&mut self) -> MUX_DS_CTRL_P2_W {
        MUX_DS_CTRL_P2_W { w: self }
    }
    #[doc = "Bit 3 - for P3 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p3(&mut self) -> MUX_DS_CTRL_P3_W {
        MUX_DS_CTRL_P3_W { w: self }
    }
    #[doc = "Bit 4 - for P4 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p4(&mut self) -> MUX_DS_CTRL_P4_W {
        MUX_DS_CTRL_P4_W { w: self }
    }
    #[doc = "Bit 5 - for P5 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p5(&mut self) -> MUX_DS_CTRL_P5_W {
        MUX_DS_CTRL_P5_W { w: self }
    }
    #[doc = "Bit 6 - for P6 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p6(&mut self) -> MUX_DS_CTRL_P6_W {
        MUX_DS_CTRL_P6_W { w: self }
    }
    #[doc = "Bit 7 - for P7 switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_p7(&mut self) -> MUX_DS_CTRL_P7_W {
        MUX_DS_CTRL_P7_W { w: self }
    }
    #[doc = "Bit 16 - for vssa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_vssa(&mut self) -> MUX_DS_CTRL_VSSA_W {
        MUX_DS_CTRL_VSSA_W { w: self }
    }
    #[doc = "Bit 17 - for temp switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_temp(&mut self) -> MUX_DS_CTRL_TEMP_W {
        MUX_DS_CTRL_TEMP_W { w: self }
    }
    #[doc = "Bit 18 - for amuxbusa switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusa(&mut self) -> MUX_DS_CTRL_AMUXBUSA_W {
        MUX_DS_CTRL_AMUXBUSA_W { w: self }
    }
    #[doc = "Bit 19 - for amuxbusb switches"]
    #[inline(always)]
    pub fn mux_ds_ctrl_amuxbusb(&mut self) -> MUX_DS_CTRL_AMUXBUSB_W {
        MUX_DS_CTRL_AMUXBUSB_W { w: self }
    }
    #[doc = "Bit 22 - for sarbus0 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus0(&mut self) -> MUX_DS_CTRL_SARBUS0_W {
        MUX_DS_CTRL_SARBUS0_W { w: self }
    }
    #[doc = "Bit 23 - for sarbus1 switch"]
    #[inline(always)]
    pub fn mux_ds_ctrl_sarbus1(&mut self) -> MUX_DS_CTRL_SARBUS1_W {
        MUX_DS_CTRL_SARBUS1_W { w: self }
    }
}
