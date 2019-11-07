#[doc = "Reader of register MUX_SWITCH_STATUS"]
pub type R = crate::R<u32, super::MUX_SWITCH_STATUS>;
#[doc = "Reader of field `MUX_FW_P0_VPLUS`"]
pub type MUX_FW_P0_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P1_VPLUS`"]
pub type MUX_FW_P1_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P2_VPLUS`"]
pub type MUX_FW_P2_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P3_VPLUS`"]
pub type MUX_FW_P3_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P4_VPLUS`"]
pub type MUX_FW_P4_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P5_VPLUS`"]
pub type MUX_FW_P5_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P6_VPLUS`"]
pub type MUX_FW_P6_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P7_VPLUS`"]
pub type MUX_FW_P7_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P0_VMINUS`"]
pub type MUX_FW_P0_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P1_VMINUS`"]
pub type MUX_FW_P1_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P2_VMINUS`"]
pub type MUX_FW_P2_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P3_VMINUS`"]
pub type MUX_FW_P3_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P4_VMINUS`"]
pub type MUX_FW_P4_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P5_VMINUS`"]
pub type MUX_FW_P5_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P6_VMINUS`"]
pub type MUX_FW_P6_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_P7_VMINUS`"]
pub type MUX_FW_P7_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_VSSA_VMINUS`"]
pub type MUX_FW_VSSA_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_TEMP_VPLUS`"]
pub type MUX_FW_TEMP_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_AMUXBUSA_VPLUS`"]
pub type MUX_FW_AMUXBUSA_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_AMUXBUSB_VPLUS`"]
pub type MUX_FW_AMUXBUSB_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_AMUXBUSA_VMINUS`"]
pub type MUX_FW_AMUXBUSA_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_AMUXBUSB_VMINUS`"]
pub type MUX_FW_AMUXBUSB_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_SARBUS0_VPLUS`"]
pub type MUX_FW_SARBUS0_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_SARBUS1_VPLUS`"]
pub type MUX_FW_SARBUS1_VPLUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_SARBUS0_VMINUS`"]
pub type MUX_FW_SARBUS0_VMINUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MUX_FW_SARBUS1_VMINUS`"]
pub type MUX_FW_SARBUS1_VMINUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MUX_FW_P0_VPLUS_R {
        MUX_FW_P0_VPLUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MUX_FW_P1_VPLUS_R {
        MUX_FW_P1_VPLUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MUX_FW_P2_VPLUS_R {
        MUX_FW_P2_VPLUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MUX_FW_P3_VPLUS_R {
        MUX_FW_P3_VPLUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MUX_FW_P4_VPLUS_R {
        MUX_FW_P4_VPLUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MUX_FW_P5_VPLUS_R {
        MUX_FW_P5_VPLUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MUX_FW_P6_VPLUS_R {
        MUX_FW_P6_VPLUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MUX_FW_P7_VPLUS_R {
        MUX_FW_P7_VPLUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MUX_FW_P0_VMINUS_R {
        MUX_FW_P0_VMINUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MUX_FW_P1_VMINUS_R {
        MUX_FW_P1_VMINUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MUX_FW_P2_VMINUS_R {
        MUX_FW_P2_VMINUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MUX_FW_P3_VMINUS_R {
        MUX_FW_P3_VMINUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MUX_FW_P4_VMINUS_R {
        MUX_FW_P4_VMINUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MUX_FW_P5_VMINUS_R {
        MUX_FW_P5_VMINUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MUX_FW_P6_VMINUS_R {
        MUX_FW_P6_VMINUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MUX_FW_P7_VMINUS_R {
        MUX_FW_P7_VMINUS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MUX_FW_VSSA_VMINUS_R {
        MUX_FW_VSSA_VMINUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MUX_FW_TEMP_VPLUS_R {
        MUX_FW_TEMP_VPLUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MUX_FW_AMUXBUSA_VPLUS_R {
        MUX_FW_AMUXBUSA_VPLUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MUX_FW_AMUXBUSB_VPLUS_R {
        MUX_FW_AMUXBUSB_VPLUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MUX_FW_AMUXBUSA_VMINUS_R {
        MUX_FW_AMUXBUSA_VMINUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MUX_FW_AMUXBUSB_VMINUS_R {
        MUX_FW_AMUXBUSB_VMINUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MUX_FW_SARBUS0_VPLUS_R {
        MUX_FW_SARBUS0_VPLUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MUX_FW_SARBUS1_VPLUS_R {
        MUX_FW_SARBUS1_VPLUS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MUX_FW_SARBUS0_VMINUS_R {
        MUX_FW_SARBUS0_VMINUS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MUX_FW_SARBUS1_VMINUS_R {
        MUX_FW_SARBUS1_VMINUS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
