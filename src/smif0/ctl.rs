#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x3000"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000
    }
}
#[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIP_MODE_A {
    #[doc = "0: '0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    MMIO_MODE = 0,
    #[doc = "1: 1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    XIP_MODE = 1,
}
impl From<XIP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XIP_MODE`"]
pub type XIP_MODE_R = crate::R<bool, XIP_MODE_A>;
impl XIP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_MODE_A {
        match self.bits {
            false => XIP_MODE_A::MMIO_MODE,
            true => XIP_MODE_A::XIP_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MMIO_MODE`"]
    #[inline(always)]
    pub fn is_mmio_mode(&self) -> bool {
        *self == XIP_MODE_A::MMIO_MODE
    }
    #[doc = "Checks if the value of the field is `XIP_MODE`"]
    #[inline(always)]
    pub fn is_xip_mode(&self) -> bool {
        *self == XIP_MODE_A::XIP_MODE
    }
}
#[doc = "Write proxy for field `XIP_MODE`"]
pub struct XIP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn mmio_mode(self) -> &'a mut W {
        self.variant(XIP_MODE_A::MMIO_MODE)
    }
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn xip_mode(self) -> &'a mut W {
        self.variant(XIP_MODE_A::XIP_MODE)
    }
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
#[doc = "Reader of field `CLOCK_IF_RX_SEL`"]
pub type CLOCK_IF_RX_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLOCK_IF_RX_SEL`"]
pub struct CLOCK_IF_RX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_IF_RX_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DESELECT_DELAY`"]
pub type DESELECT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DESELECT_DELAY`"]
pub struct DESELECT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DESELECT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_A {
    #[doc = "0: 0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BUS_ERROR = 0,
    #[doc = "1: 1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    WAIT_STATES = 1,
}
impl From<BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLOCK`"]
pub type BLOCK_R = crate::R<bool, BLOCK_A>;
impl BLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_A {
        match self.bits {
            false => BLOCK_A::BUS_ERROR,
            true => BLOCK_A::WAIT_STATES,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline(always)]
    pub fn is_bus_error(&self) -> bool {
        *self == BLOCK_A::BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `WAIT_STATES`"]
    #[inline(always)]
    pub fn is_wait_states(&self) -> bool {
        *self == BLOCK_A::WAIT_STATES
    }
}
#[doc = "Write proxy for field `BLOCK`"]
pub struct BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn bus_error(self) -> &'a mut W {
        self.variant(BLOCK_A::BUS_ERROR)
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn wait_states(self) -> &'a mut W {
        self.variant(BLOCK_A::WAIT_STATES)
    }
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
#[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLED_A {
    #[doc = "0: N/A"]
    DISABLED = 0,
    #[doc = "1: N/A"]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, ENABLED_A>;
impl ENABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
    }
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
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn xip_mode(&self) -> XIP_MODE_R {
        XIP_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub fn clock_if_rx_sel(&self) -> CLOCK_IF_RX_SEL_R {
        CLOCK_IF_RX_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&self) -> DESELECT_DELAY_R {
        DESELECT_DELAY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn xip_mode(&mut self) -> XIP_MODE_W {
        XIP_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub fn clock_if_rx_sel(&mut self) -> CLOCK_IF_RX_SEL_W {
        CLOCK_IF_RX_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&mut self) -> DESELECT_DELAY_W {
        DESELECT_DELAY_W { w: self }
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W { w: self }
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
