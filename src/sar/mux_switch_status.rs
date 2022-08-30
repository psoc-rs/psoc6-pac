#[doc = "Register `MUX_SWITCH_STATUS` reader"]
pub struct R(crate::R<MUX_SWITCH_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MUX_FW_P0_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P0_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P1_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P1_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P2_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P2_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P3_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P3_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P4_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P5_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P6_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P6_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P7_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P7_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P0_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P0_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P1_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P1_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P2_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P2_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P3_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P3_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P4_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P5_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P6_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P6_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P7_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_P7_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_VSSA_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_VSSA_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_TEMP_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_TEMP_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_AMUXBUSA_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_AMUXBUSB_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_AMUXBUSA_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_AMUXBUSB_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_SARBUS0_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_SARBUS1_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_SARBUS0_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` reader - switch status of corresponding bit in MUX_SWITCH0"]
pub type MUX_FW_SARBUS1_VMINUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MUX_FW_P0_VPLUS_R {
        MUX_FW_P0_VPLUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MUX_FW_P1_VPLUS_R {
        MUX_FW_P1_VPLUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MUX_FW_P2_VPLUS_R {
        MUX_FW_P2_VPLUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MUX_FW_P3_VPLUS_R {
        MUX_FW_P3_VPLUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MUX_FW_P4_VPLUS_R {
        MUX_FW_P4_VPLUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MUX_FW_P5_VPLUS_R {
        MUX_FW_P5_VPLUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MUX_FW_P6_VPLUS_R {
        MUX_FW_P6_VPLUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MUX_FW_P7_VPLUS_R {
        MUX_FW_P7_VPLUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MUX_FW_P0_VMINUS_R {
        MUX_FW_P0_VMINUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MUX_FW_P1_VMINUS_R {
        MUX_FW_P1_VMINUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MUX_FW_P2_VMINUS_R {
        MUX_FW_P2_VMINUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MUX_FW_P3_VMINUS_R {
        MUX_FW_P3_VMINUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MUX_FW_P4_VMINUS_R {
        MUX_FW_P4_VMINUS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MUX_FW_P5_VMINUS_R {
        MUX_FW_P5_VMINUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MUX_FW_P6_VMINUS_R {
        MUX_FW_P6_VMINUS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MUX_FW_P7_VMINUS_R {
        MUX_FW_P7_VMINUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MUX_FW_VSSA_VMINUS_R {
        MUX_FW_VSSA_VMINUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MUX_FW_TEMP_VPLUS_R {
        MUX_FW_TEMP_VPLUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MUX_FW_AMUXBUSA_VPLUS_R {
        MUX_FW_AMUXBUSA_VPLUS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MUX_FW_AMUXBUSB_VPLUS_R {
        MUX_FW_AMUXBUSB_VPLUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MUX_FW_AMUXBUSA_VMINUS_R {
        MUX_FW_AMUXBUSA_VMINUS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MUX_FW_AMUXBUSB_VMINUS_R {
        MUX_FW_AMUXBUSB_VMINUS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MUX_FW_SARBUS0_VPLUS_R {
        MUX_FW_SARBUS0_VPLUS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MUX_FW_SARBUS1_VPLUS_R {
        MUX_FW_SARBUS1_VPLUS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MUX_FW_SARBUS0_VMINUS_R {
        MUX_FW_SARBUS0_VMINUS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - switch status of corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MUX_FW_SARBUS1_VMINUS_R {
        MUX_FW_SARBUS1_VMINUS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "SARMUX switch status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_status](index.html) module"]
pub struct MUX_SWITCH_STATUS_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch_status::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MUX_SWITCH_STATUS to value 0"]
impl crate::Resettable for MUX_SWITCH_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
