#[doc = "Reader of register PWR_CTL"]
pub type R = crate::R<u32, super::PWR_CTL>;
#[doc = "Writer for register PWR_CTL"]
pub type W = crate::W<u32, super::PWR_CTL>;
#[doc = "Register PWR_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current power mode of the device. LPACTIVE/LPSLEEP are implemented as firmware configuration of multiple registers and are reported here as ACTIVE/SLEEP, respectively. Note that this field cannot be read in all power modes on actual silicon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POWER_MODE_A {
    #[doc = "0: System is resetting."]
    RESET = 0,
    #[doc = "1: At least one CPU is running."]
    ACTIVE = 1,
    #[doc = "2: No CPUs are running.  Peripherals may be running."]
    SLEEP = 2,
    #[doc = "3: Main high-frequency clock is off; low speed clocks are available.  Communication interface clocks may be present."]
    DEEPSLEEP = 3,
}
impl From<POWER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: POWER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `POWER_MODE`"]
pub type POWER_MODE_R = crate::R<u8, POWER_MODE_A>;
impl POWER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_MODE_A {
        match self.bits {
            0 => POWER_MODE_A::RESET,
            1 => POWER_MODE_A::ACTIVE,
            2 => POWER_MODE_A::SLEEP,
            3 => POWER_MODE_A::DEEPSLEEP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == POWER_MODE_A::RESET
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == POWER_MODE_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == POWER_MODE_A::SLEEP
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == POWER_MODE_A::DEEPSLEEP
    }
}
#[doc = "Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_SESSION_A {
    #[doc = "0: No debug session active"]
    NO_SESSION = 0,
    #[doc = "1: Debug session is active.  Power modes behave differently to keep the debug session active."]
    SESSION_ACTIVE = 1,
}
impl From<DEBUG_SESSION_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUG_SESSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEBUG_SESSION`"]
pub type DEBUG_SESSION_R = crate::R<bool, DEBUG_SESSION_A>;
impl DEBUG_SESSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUG_SESSION_A {
        match self.bits {
            false => DEBUG_SESSION_A::NO_SESSION,
            true => DEBUG_SESSION_A::SESSION_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SESSION`"]
    #[inline(always)]
    pub fn is_no_session(&self) -> bool {
        *self == DEBUG_SESSION_A::NO_SESSION
    }
    #[doc = "Checks if the value of the field is `SESSION_ACTIVE`"]
    #[inline(always)]
    pub fn is_session_active(&self) -> bool {
        *self == DEBUG_SESSION_A::SESSION_ACTIVE
    }
}
#[doc = "Reader of field `LPM_READY`"]
pub type LPM_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `IREF_LPMODE`"]
pub type IREF_LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IREF_LPMODE`"]
pub struct IREF_LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IREF_LPMODE_W<'a> {
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
#[doc = "Reader of field `VREFBUF_OK`"]
pub type VREFBUF_OK_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPSLP_REG_DIS`"]
pub type DPSLP_REG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPSLP_REG_DIS`"]
pub struct DPSLP_REG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REG_DIS_W<'a> {
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
#[doc = "Reader of field `RET_REG_DIS`"]
pub type RET_REG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RET_REG_DIS`"]
pub struct RET_REG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_REG_DIS_W<'a> {
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
#[doc = "Reader of field `NWELL_REG_DIS`"]
pub type NWELL_REG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NWELL_REG_DIS`"]
pub struct NWELL_REG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NWELL_REG_DIS_W<'a> {
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
#[doc = "Reader of field `LINREG_DIS`"]
pub type LINREG_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINREG_DIS`"]
pub struct LINREG_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LINREG_DIS_W<'a> {
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
#[doc = "Reader of field `LINREG_LPMODE`"]
pub type LINREG_LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINREG_LPMODE`"]
pub struct LINREG_LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINREG_LPMODE_W<'a> {
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
#[doc = "Reader of field `PORBOD_LPMODE`"]
pub type PORBOD_LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORBOD_LPMODE`"]
pub struct PORBOD_LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORBOD_LPMODE_W<'a> {
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
#[doc = "Reader of field `BGREF_LPMODE`"]
pub type BGREF_LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BGREF_LPMODE`"]
pub struct BGREF_LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BGREF_LPMODE_W<'a> {
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
#[doc = "Reader of field `PLL_LS_BYPASS`"]
pub type PLL_LS_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL_LS_BYPASS`"]
pub struct PLL_LS_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LS_BYPASS_W<'a> {
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
#[doc = "Reader of field `VREFBUF_LPMODE`"]
pub type VREFBUF_LPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFBUF_LPMODE`"]
pub struct VREFBUF_LPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUF_LPMODE_W<'a> {
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
#[doc = "Reader of field `VREFBUF_DIS`"]
pub type VREFBUF_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREFBUF_DIS`"]
pub struct VREFBUF_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFBUF_DIS_W<'a> {
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
#[doc = "Reader of field `ACT_REF_DIS`"]
pub type ACT_REF_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACT_REF_DIS`"]
pub struct ACT_REF_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ACT_REF_OK`"]
pub type ACT_REF_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Current power mode of the device. LPACTIVE/LPSLEEP are implemented as firmware configuration of multiple registers and are reported here as ACTIVE/SLEEP, respectively. Note that this field cannot be read in all power modes on actual silicon."]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Indicates whether a debug session is active (CDBGPWRUPREQ signal is 1)"]
    #[inline(always)]
    pub fn debug_session(&self) -> DEBUG_SESSION_R {
        DEBUG_SESSION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether certain low power functions are ready. The low current circuits take longer to startup after POR/XRES/BOD/HIBERNATE wakeup than the normal mode circuits. HIBERNATE mode may be entered regardless of this bit. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: If a low power circuit operation is requested, it will stay in its normal operating mode until it is ready. If DEEPSLEEP is requested by all processors WFI/WFE, the device will instead enter SLEEP. When low power circuits are ready, device will automatically enter the originally requested mode. 1: Normal operation. DEEPSLEEP and low power circuits operate as requested in other registers."]
    #[inline(always)]
    pub fn lpm_ready(&self) -> LPM_READY_R {
        LPM_READY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn iref_lpmode(&self) -> IREF_LPMODE_R {
        IREF_LPMODE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Indicates that the voltage reference buffer is ready. Due to synchronization delays, it may take two IMO clock cycles for hardware to clear this bit after asserting VREFBUF_DIS=1."]
    #[inline(always)]
    pub fn vrefbuf_ok(&self) -> VREFBUF_OK_R {
        VREFBUF_OK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub fn dpslp_reg_dis(&self) -> DPSLP_REG_DIS_R {
        DPSLP_REG_DIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable the Retention regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn ret_reg_dis(&self) -> RET_REG_DIS_R {
        RET_REG_DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn nwell_reg_dis(&self) -> NWELL_REG_DIS_R {
        NWELL_REG_DIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub fn linreg_dis(&self) -> LINREG_DIS_R {
        LINREG_DIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Control the power mode of the ULP Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: ULP Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn linreg_lpmode(&self) -> LINREG_LPMODE_R {
        LINREG_LPMODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Control the power mode of the ULP POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: ULP POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn porbod_lpmode(&self) -> PORBOD_LPMODE_R {
        PORBOD_LPMODE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Control the power mode of the ULP Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: ULP Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub fn bgref_lpmode(&self) -> BGREF_LPMODE_R {
        BGREF_LPMODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn pll_ls_bypass(&self) -> PLL_LS_BYPASS_R {
        PLL_LS_BYPASS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: ULP Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 1: ULP Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn vrefbuf_lpmode(&self) -> VREFBUF_LPMODE_R {
        VREFBUF_LPMODE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn vrefbuf_dis(&self) -> VREFBUF_DIS_R {
        VREFBUF_DIS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub fn act_ref_dis(&self) -> ACT_REF_DIS_R {
        ACT_REF_DIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicates that the normal mode of the Active Reference is ready."]
    #[inline(always)]
    pub fn act_ref_ok(&self) -> ACT_REF_OK_R {
        ACT_REF_OK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Control the power mode of the reference current generator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Current reference generator operates in normal mode. It works for vddd ramp rates of 100mV/us or less. 1: Current reference generator operates in low power mode. Response time is reduced to save current, and it works for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn iref_lpmode(&mut self) -> IREF_LPMODE_W {
        IREF_LPMODE_W { w: self }
    }
    #[doc = "Bit 20 - Disable the DeepSleep regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: DeepSleep Regulator is on. 1: DeepSleep Regulator is off."]
    #[inline(always)]
    pub fn dpslp_reg_dis(&mut self) -> DPSLP_REG_DIS_W {
        DPSLP_REG_DIS_W { w: self }
    }
    #[doc = "Bit 21 - Disable the Retention regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Retention Regulator is on. 1: Retention Regulator is off."]
    #[inline(always)]
    pub fn ret_reg_dis(&mut self) -> RET_REG_DIS_W {
        RET_REG_DIS_W { w: self }
    }
    #[doc = "Bit 22 - Disable the Nwell regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Nwell Regulator is on. 1: Nwell Regulator is off."]
    #[inline(always)]
    pub fn nwell_reg_dis(&mut self) -> NWELL_REG_DIS_W {
        NWELL_REG_DIS_W { w: self }
    }
    #[doc = "Bit 23 - Disable the linear Core Regulator. For ULP products, this is only legal when the ULP SISO-LC/SIMO-LC Buck supplies vccd, but there is no hardware protection for this case. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Linear regulator is on. 1: Linear regulator is off."]
    #[inline(always)]
    pub fn linreg_dis(&mut self) -> LINREG_DIS_W {
        LINREG_DIS_W { w: self }
    }
    #[doc = "Bit 24 - Control the power mode of the ULP Linear Regulator. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP Linear Regulator operates in normal mode. Internal current consumption is 50uA and load current capability is 50mA to 300mA, depending on the number of regulator modules present in the product. 1: ULP Linear Regulator operates in low power mode. Internal current consumption is 5uA and load current capability is 25mA. Firmware must ensure the current is kept within the limit."]
    #[inline(always)]
    pub fn linreg_lpmode(&mut self) -> LINREG_LPMODE_W {
        LINREG_LPMODE_W { w: self }
    }
    #[doc = "Bit 25 - Control the power mode of the ULP POR/BOD circuits. The value in this register is ignored and normal mode is used until LPM_READY==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP POR/BOD circuits operate in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: ULP POR/BOD circuits operate in low power mode. Response time is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn porbod_lpmode(&mut self) -> PORBOD_LPMODE_W {
        PORBOD_LPMODE_W { w: self }
    }
    #[doc = "Bit 26 - Control the power mode of the ULP Bandgap Voltage and Current References. This applies to voltage and current generation and is different than the reference voltage buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. When lower power mode is used, the Active Reference circuit can be disabled to reduce current. Firmware is responsible to ensure ACT_REF_OK==1 before changing back to normal mode. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: ULP Active Bandgap Voltage and Current Reference operates in normal mode. They work for vddd ramp rates of 100mV/us or less. 1: ULP Active Bandgap Voltage and Current Reference operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less. The Active Reference may be disabled using ACT_REF_DIS=0."]
    #[inline(always)]
    pub fn bgref_lpmode(&mut self) -> BGREF_LPMODE_W {
        BGREF_LPMODE_W { w: self }
    }
    #[doc = "Bit 27 - Bypass level shifter inside the PLL. 0: Do not bypass the level shifter. This setting is ok for all operational modes and vccd target voltage. 1: Bypass the level shifter. This may reduce jitter on the PLL output clock, but can only be used when vccd is targeted to 1.1V nominal. Otherwise, it can result in clock degradation and static current."]
    #[inline(always)]
    pub fn pll_ls_bypass(&mut self) -> PLL_LS_BYPASS_W {
        PLL_LS_BYPASS_W { w: self }
    }
    #[doc = "Bit 28 - Control the power mode of the 800mV voltage reference buffer. The value in this register is ignored and normal mode is used until LPM_READY==1. 0: ULP Voltage Reference Buffer operates in normal mode. They work for vddd ramp rates of 100mV/us or less. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 1: ULP Voltage Reference Buffer operates in low power mode. Power supply rejection is reduced to save current, and they work for vddd ramp rates of 10mV/us or less."]
    #[inline(always)]
    pub fn vrefbuf_lpmode(&mut self) -> VREFBUF_LPMODE_W {
        VREFBUF_LPMODE_W { w: self }
    }
    #[doc = "Bit 29 - Disable the 800mV voltage reference buffer. Firmware should only disable the buffer when there is no connected circuit that is using it. SRSS circuits that require it are the PLL and ECO. A particular product may have circuits outside the SRSS that use the buffer. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn vrefbuf_dis(&mut self) -> VREFBUF_DIS_W {
        VREFBUF_DIS_W { w: self }
    }
    #[doc = "Bit 30 - Disables the Active Reference. Firmware must ensure that LPM_READY==1 and BGREF_LPMODE==1 for at least 1us before disabling the Active Reference. When enabling the Active Reference, use ACT_REF_OK indicator to know when it is ready. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: Active Reference is enabled 1: Active Reference is disabled"]
    #[inline(always)]
    pub fn act_ref_dis(&mut self) -> ACT_REF_DIS_W {
        ACT_REF_DIS_W { w: self }
    }
}
