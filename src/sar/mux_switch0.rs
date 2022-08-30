#[doc = "Register `MUX_SWITCH0` reader"]
pub struct R(crate::R<MUX_SWITCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH0` writer"]
pub struct W(crate::W<MUX_SWITCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH0_SPEC>;
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
impl From<crate::W<MUX_SWITCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_FW_P0_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P0_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P0_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P0_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P1_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P1_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P1_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P1_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P2_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P2_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P2_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P2_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P3_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P3_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P3_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P3_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P4_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P4_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P4_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P5_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P5_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P5_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P6_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P6_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P6_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P6_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P7_VPLUS` reader - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P7_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P7_VPLUS` writer - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_P7_VPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P0_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P0_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P0_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P0_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P1_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P1_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P1_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P1_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P2_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P2_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P2_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P2_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P3_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P3_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P3_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P3_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P4_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P4_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P4_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P5_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P5_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P5_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P6_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P6_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P6_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P6_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P7_VMINUS` reader - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P7_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P7_VMINUS` writer - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_P7_VMINUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_VSSA_VMINUS` reader - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_VSSA_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_VSSA_VMINUS` writer - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_VSSA_VMINUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_TEMP_VPLUS` reader - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
pub type MUX_FW_TEMP_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_TEMP_VPLUS` writer - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
pub type MUX_FW_TEMP_VPLUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` reader - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSA_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` writer - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSA_VPLUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` reader - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSB_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` writer - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSB_VPLUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` reader - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSA_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` writer - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSA_VMINUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` reader - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSB_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` writer - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_AMUXBUSB_VMINUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` reader - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS0_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` writer - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS0_VPLUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` reader - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS1_VPLUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` writer - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS1_VPLUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` reader - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS0_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` writer - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS0_VMINUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` reader - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS1_VMINUS_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` writer - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
pub type MUX_FW_SARBUS1_VMINUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P4_COREIO0` reader - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
pub type MUX_FW_P4_COREIO0_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_COREIO0` writer - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
pub type MUX_FW_P4_COREIO0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P5_COREIO1` reader - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
pub type MUX_FW_P5_COREIO1_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_COREIO1` writer - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
pub type MUX_FW_P5_COREIO1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P6_COREIO2` reader - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
pub type MUX_FW_P6_COREIO2_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P6_COREIO2` writer - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
pub type MUX_FW_P6_COREIO2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
#[doc = "Field `MUX_FW_P7_COREIO3` reader - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
pub type MUX_FW_P7_COREIO3_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P7_COREIO3` writer - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
pub type MUX_FW_P7_COREIO3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MUX_SWITCH0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MUX_FW_P0_VPLUS_R {
        MUX_FW_P0_VPLUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MUX_FW_P1_VPLUS_R {
        MUX_FW_P1_VPLUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MUX_FW_P2_VPLUS_R {
        MUX_FW_P2_VPLUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MUX_FW_P3_VPLUS_R {
        MUX_FW_P3_VPLUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MUX_FW_P4_VPLUS_R {
        MUX_FW_P4_VPLUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MUX_FW_P5_VPLUS_R {
        MUX_FW_P5_VPLUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MUX_FW_P6_VPLUS_R {
        MUX_FW_P6_VPLUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MUX_FW_P7_VPLUS_R {
        MUX_FW_P7_VPLUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MUX_FW_P0_VMINUS_R {
        MUX_FW_P0_VMINUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MUX_FW_P1_VMINUS_R {
        MUX_FW_P1_VMINUS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MUX_FW_P2_VMINUS_R {
        MUX_FW_P2_VMINUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MUX_FW_P3_VMINUS_R {
        MUX_FW_P3_VMINUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MUX_FW_P4_VMINUS_R {
        MUX_FW_P4_VMINUS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MUX_FW_P5_VMINUS_R {
        MUX_FW_P5_VMINUS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MUX_FW_P6_VMINUS_R {
        MUX_FW_P6_VMINUS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MUX_FW_P7_VMINUS_R {
        MUX_FW_P7_VMINUS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MUX_FW_VSSA_VMINUS_R {
        MUX_FW_VSSA_VMINUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MUX_FW_TEMP_VPLUS_R {
        MUX_FW_TEMP_VPLUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MUX_FW_AMUXBUSA_VPLUS_R {
        MUX_FW_AMUXBUSA_VPLUS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MUX_FW_AMUXBUSB_VPLUS_R {
        MUX_FW_AMUXBUSB_VPLUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MUX_FW_AMUXBUSA_VMINUS_R {
        MUX_FW_AMUXBUSA_VMINUS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MUX_FW_AMUXBUSB_VMINUS_R {
        MUX_FW_AMUXBUSB_VMINUS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MUX_FW_SARBUS0_VPLUS_R {
        MUX_FW_SARBUS0_VPLUS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MUX_FW_SARBUS1_VPLUS_R {
        MUX_FW_SARBUS1_VPLUS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MUX_FW_SARBUS0_VMINUS_R {
        MUX_FW_SARBUS0_VMINUS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MUX_FW_SARBUS1_VMINUS_R {
        MUX_FW_SARBUS1_VMINUS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&self) -> MUX_FW_P4_COREIO0_R {
        MUX_FW_P4_COREIO0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&self) -> MUX_FW_P5_COREIO1_R {
        MUX_FW_P5_COREIO1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&self) -> MUX_FW_P6_COREIO2_R {
        MUX_FW_P6_COREIO2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&self) -> MUX_FW_P7_COREIO3_R {
        MUX_FW_P7_COREIO3_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between pin P0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&mut self) -> MUX_FW_P0_VPLUS_W<0> {
        MUX_FW_P0_VPLUS_W::new(self)
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between pin P1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&mut self) -> MUX_FW_P1_VPLUS_W<1> {
        MUX_FW_P1_VPLUS_W::new(self)
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between pin P2 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&mut self) -> MUX_FW_P2_VPLUS_W<2> {
        MUX_FW_P2_VPLUS_W::new(self)
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between pin P3 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&mut self) -> MUX_FW_P3_VPLUS_W<3> {
        MUX_FW_P3_VPLUS_W::new(self)
    }
    #[doc = "Bit 4 - Firmware control: 0=open, 1=close switch between pin P4 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&mut self) -> MUX_FW_P4_VPLUS_W<4> {
        MUX_FW_P4_VPLUS_W::new(self)
    }
    #[doc = "Bit 5 - Firmware control: 0=open, 1=close switch between pin P5 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&mut self) -> MUX_FW_P5_VPLUS_W<5> {
        MUX_FW_P5_VPLUS_W::new(self)
    }
    #[doc = "Bit 6 - Firmware control: 0=open, 1=close switch between pin P6 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&mut self) -> MUX_FW_P6_VPLUS_W<6> {
        MUX_FW_P6_VPLUS_W::new(self)
    }
    #[doc = "Bit 7 - Firmware control: 0=open, 1=close switch between pin P7 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&mut self) -> MUX_FW_P7_VPLUS_W<7> {
        MUX_FW_P7_VPLUS_W::new(self)
    }
    #[doc = "Bit 8 - Firmware control: 0=open, 1=close switch between pin P0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&mut self) -> MUX_FW_P0_VMINUS_W<8> {
        MUX_FW_P0_VMINUS_W::new(self)
    }
    #[doc = "Bit 9 - Firmware control: 0=open, 1=close switch between pin P1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&mut self) -> MUX_FW_P1_VMINUS_W<9> {
        MUX_FW_P1_VMINUS_W::new(self)
    }
    #[doc = "Bit 10 - Firmware control: 0=open, 1=close switch between pin P2 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&mut self) -> MUX_FW_P2_VMINUS_W<10> {
        MUX_FW_P2_VMINUS_W::new(self)
    }
    #[doc = "Bit 11 - Firmware control: 0=open, 1=close switch between pin P3 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&mut self) -> MUX_FW_P3_VMINUS_W<11> {
        MUX_FW_P3_VMINUS_W::new(self)
    }
    #[doc = "Bit 12 - Firmware control: 0=open, 1=close switch between pin P4 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&mut self) -> MUX_FW_P4_VMINUS_W<12> {
        MUX_FW_P4_VMINUS_W::new(self)
    }
    #[doc = "Bit 13 - Firmware control: 0=open, 1=close switch between pin P5 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&mut self) -> MUX_FW_P5_VMINUS_W<13> {
        MUX_FW_P5_VMINUS_W::new(self)
    }
    #[doc = "Bit 14 - Firmware control: 0=open, 1=close switch between pin P6 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&mut self) -> MUX_FW_P6_VMINUS_W<14> {
        MUX_FW_P6_VMINUS_W::new(self)
    }
    #[doc = "Bit 15 - Firmware control: 0=open, 1=close switch between pin P7 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&mut self) -> MUX_FW_P7_VMINUS_W<15> {
        MUX_FW_P7_VMINUS_W::new(self)
    }
    #[doc = "Bit 16 - Firmware control: 0=open, 1=close switch between vssa_kelvin and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&mut self) -> MUX_FW_VSSA_VMINUS_W<16> {
        MUX_FW_VSSA_VMINUS_W::new(self)
    }
    #[doc = "Bit 17 - Firmware control: 0=open, 1=close switch between temperature sensor and vplus signal, also powers on the temperature sensor. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&mut self) -> MUX_FW_TEMP_VPLUS_W<17> {
        MUX_FW_TEMP_VPLUS_W::new(self)
    }
    #[doc = "Bit 18 - Firmware control: 0=open, 1=close switch between amuxbusa and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&mut self) -> MUX_FW_AMUXBUSA_VPLUS_W<18> {
        MUX_FW_AMUXBUSA_VPLUS_W::new(self)
    }
    #[doc = "Bit 19 - Firmware control: 0=open, 1=close switch between amuxbusb and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&mut self) -> MUX_FW_AMUXBUSB_VPLUS_W<19> {
        MUX_FW_AMUXBUSB_VPLUS_W::new(self)
    }
    #[doc = "Bit 20 - Firmware control: 0=open, 1=close switch between amuxbusa and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&mut self) -> MUX_FW_AMUXBUSA_VMINUS_W<20> {
        MUX_FW_AMUXBUSA_VMINUS_W::new(self)
    }
    #[doc = "Bit 21 - Firmware control: 0=open, 1=close switch between amuxbusb and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&mut self) -> MUX_FW_AMUXBUSB_VMINUS_W<21> {
        MUX_FW_AMUXBUSB_VMINUS_W::new(self)
    }
    #[doc = "Bit 22 - Firmware control: 0=open, 1=close switch between sarbus0 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&mut self) -> MUX_FW_SARBUS0_VPLUS_W<22> {
        MUX_FW_SARBUS0_VPLUS_W::new(self)
    }
    #[doc = "Bit 23 - Firmware control: 0=open, 1=close switch between sarbus1 and vplus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&mut self) -> MUX_FW_SARBUS1_VPLUS_W<23> {
        MUX_FW_SARBUS1_VPLUS_W::new(self)
    }
    #[doc = "Bit 24 - Firmware control: 0=open, 1=close switch between sarbus0 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&mut self) -> MUX_FW_SARBUS0_VMINUS_W<24> {
        MUX_FW_SARBUS0_VMINUS_W::new(self)
    }
    #[doc = "Bit 25 - Firmware control: 0=open, 1=close switch between sarbus1 and vminus signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&mut self) -> MUX_FW_SARBUS1_VMINUS_W<25> {
        MUX_FW_SARBUS1_VMINUS_W::new(self)
    }
    #[doc = "Bit 26 - Firmware control: 0=open, 1=close switch between P4 and coreio0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&mut self) -> MUX_FW_P4_COREIO0_W<26> {
        MUX_FW_P4_COREIO0_W::new(self)
    }
    #[doc = "Bit 27 - Firmware control: 0=open, 1=close switch between P5 and coreio1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&mut self) -> MUX_FW_P5_COREIO1_W<27> {
        MUX_FW_P5_COREIO1_W::new(self)
    }
    #[doc = "Bit 28 - Firmware control: 0=open, 1=close switch between P6 and coreio2 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&mut self) -> MUX_FW_P6_COREIO2_W<28> {
        MUX_FW_P6_COREIO2_W::new(self)
    }
    #[doc = "Bit 29 - Firmware control: 0=open, 1=close switch between P7 and coreio3 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&mut self) -> MUX_FW_P7_COREIO3_W<29> {
        MUX_FW_P7_COREIO3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SARMUX Firmware switch controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch0](index.html) module"]
pub struct MUX_SWITCH0_SPEC;
impl crate::RegisterSpec for MUX_SWITCH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch0::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch0::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH0 to value 0"]
impl crate::Resettable for MUX_SWITCH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
