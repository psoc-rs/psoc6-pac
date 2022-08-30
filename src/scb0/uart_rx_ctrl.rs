#[doc = "Register `UART_RX_CTRL` reader"]
pub struct R(crate::R<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_RX_CTRL` writer"]
pub struct W(crate::W<UART_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_RX_CTRL_SPEC>;
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
impl From<crate::W<UART_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle time between data frames and the data frame value."]
pub type STOP_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BITS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle time between data frames and the data frame value."]
pub type STOP_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_RX_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PARITY` reader - N/A"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - N/A"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `PARITY_ENABLED` reader - N/A"]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - N/A"]
pub type PARITY_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality only works for IrDA receiver functionality."]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality only works for IrDA receiver functionality."]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `DROP_ON_PARITY_ERROR` reader - N/A"]
pub type DROP_ON_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_PARITY_ERROR` writer - N/A"]
pub type DROP_ON_PARITY_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `DROP_ON_FRAME_ERROR` reader - Behaviour when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_FRAME_ERROR` writer - Behaviour when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `MP_MODE` reader - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH must be 9 bits. In multi-processor mode, the 9th received bit of a data frame seperates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data is sent to the RX FIFO. In the case of NO match, subsequent received data is dropped."]
pub type MP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MP_MODE` writer - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH must be 9 bits. In multi-processor mode, the 9th received bit of a data frame seperates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data is sent to the RX FIFO. In the case of NO match, subsequent received data is dropped."]
pub type MP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `LIN_MODE` reader - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type LIN_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LIN_MODE` writer - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
pub type LIN_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `SKIP_START` reader - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will then synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
pub type SKIP_START_R = crate::BitReader<bool>;
#[doc = "Field `SKIP_START` writer - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will then synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
pub type SKIP_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_RX_CTRL_SPEC, bool, O>;
#[doc = "Field `BREAK_WIDTH` reader - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note for LIN the break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
pub type BREAK_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREAK_WIDTH` writer - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note for LIN the break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
pub type BREAK_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_RX_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle time between data frames and the data frame value."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality only works for IrDA receiver functionality."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn drop_on_parity_error(&self) -> DROP_ON_PARITY_ERROR_R {
        DROP_ON_PARITY_ERROR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Behaviour when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_frame_error(&self) -> DROP_ON_FRAME_ERROR_R {
        DROP_ON_FRAME_ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH must be 9 bits. In multi-processor mode, the 9th received bit of a data frame seperates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data is sent to the RX FIFO. In the case of NO match, subsequent received data is dropped."]
    #[inline(always)]
    pub fn mp_mode(&self) -> MP_MODE_R {
        MP_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub fn lin_mode(&self) -> LIN_MODE_R {
        LIN_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will then synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
    #[inline(always)]
    pub fn skip_start(&self) -> SKIP_START_R {
        SKIP_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note for LIN the break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
    #[inline(always)]
    pub fn break_width(&self) -> BREAK_WIDTH_R {
        BREAK_WIDTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of half bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period. If STOP_BITS is '1', stop bits error detection is NOT performed. If STOP_BITS is in \\[2, 7\\], stop bits error detection is performed and the associated interrupt cause INTR_RX.FRAME_ERROR is set to '1' if an error is detected. In other words, the receiver supports data frames with a 1 bit period stop bit sequence, but requires at least 1.5 bit period stop bit sequences to detect errors. Note that in case of a stop bits error, the successive data frames may get lost as the receiver needs to resynchronize its start bit detection. The amount of lost data frames depends on both the amount of stop bits, the idle time between data frames and the data frame value."]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W<0> {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W<4> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W<5> {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal 'uart_rx_in'. Inversion is after local loopback. This functionality only works for IrDA receiver functionality."]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<6> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn drop_on_parity_error(&mut self) -> DROP_ON_PARITY_ERROR_W<8> {
        DROP_ON_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 9 - Behaviour when an error is detected in a start or stop period. When '0', received data is sent to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_frame_error(&mut self) -> DROP_ON_FRAME_ERROR_W<9> {
        DROP_ON_FRAME_ERROR_W::new(self)
    }
    #[doc = "Bit 10 - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH must be 9 bits. In multi-processor mode, the 9th received bit of a data frame seperates addresses (bit is '1') from data (bit is '0'). A received address is matched with RX_MATCH.DATA and RX_MATCH.MASK. In the case of a match, subsequent received data is sent to the RX FIFO. In the case of NO match, subsequent received data is dropped."]
    #[inline(always)]
    pub fn mp_mode(&mut self) -> MP_MODE_W<10> {
        MP_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data. First, break detection counts the amount of bit periods that have a line value of '0'. BREAK_WIDTH specifies the minum required amount of bit periods. Successful break detection sets the INTR_RX.BREAK_DETECT interrupt cause to '1'. Second, baud rate detection counts the amount of peripheral clock periods that are use to receive the synchronization byte (0x55; least significant bit first). The count is available through UART_RX_STATUS.BR_COUNTER. Successful baud rate detection sets the INTR_RX.BAUD_DETECT interrupt cause to '1' (BR_COUNTER is reliable). This functionality is used to synchronize/refine the receiver clock to the transmitter clock. The receiver software can use the BR_COUNTER value to set the right clk_scb to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte."]
    #[inline(always)]
    pub fn lin_mode(&mut self) -> LIN_MODE_W<12> {
        LIN_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'. This functionality is intended for wake up from DeepSleep when receiving a data frame. The transition from idle ('1') to START ('0') on the RX line is used to wake up the CPU. The transition detection (and the associated wake up functionality) is performed by the GPIO. The woken up CPU will enable the SCB's UART receiver functionality. Once enabled, it is assumed that the START bit is ongoing (the CPU wakeup and SCB enable time should be less than the START bit period). The SCB will then synchronize to a '0' to '1' transition, which indicates the first data frame bit is received (first data frame bit should be '1'). After synchronization to the first data frame bit, the SCB will resume normal UART functionality: subsequent data frames will be synchronized on the receipt of a START bit."]
    #[inline(always)]
    pub fn skip_start(&mut self) -> SKIP_START_W<13> {
        SKIP_START_W::new(self)
    }
    #[doc = "Bits 16:19 - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break. During a break the transmitted/received line value is '0'. This feature is useful for standard UART submode and LIN submode ('break field' detection). Once, the break is detected, the INTR_RX.BREAK_DETECT bit is set to '1'. Note for LIN the break detection precedes baud rate detection, which is used to synchronize/refine the receiver clock to the transmitter clock. As a result, break detection operates with an unsynchronized/unrefined receiver clock. Therefore, the receiver's definition of a bit period is imprecise and the setting of this field should take this imprecision into account. The LIN standard also accounts for this imprecision: a LIN start bit followed by 8 data bits allows for up to 9 consecutive '0' bit periods during regular transmission, whereas the LIN break detection should be at least 13 consecutive '0' bit periods. This provides for a margin of 4 bit periods. Therefore, the default value of this field is set to 10, representing a minimal break field with of 10+1 = 11 bit periods; a value in between the 9 consecutive bit periods of a regular transmission and the 13 consecutive bit periods of a break field. This provides for slight imprecisions of the receiver clock wrt. the transmitter clock. There should not be a need to program this field to any value other than its default value."]
    #[inline(always)]
    pub fn break_width(&mut self) -> BREAK_WIDTH_W<16> {
        BREAK_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART receiver control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rx_ctrl](index.html) module"]
pub struct UART_RX_CTRL_SPEC;
impl crate::RegisterSpec for UART_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UART_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UART_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_RX_CTRL to value 0x000a_0002"]
impl crate::Resettable for UART_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_0002
    }
}
