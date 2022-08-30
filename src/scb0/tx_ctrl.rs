#[doc = "Register `TX_CTRL` reader"]
pub struct R(crate::R<TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CTRL` writer"]
pub struct W(crate::W<TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CTRL_SPEC>;
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
impl From<crate::W<TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
pub type MSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTRL_SPEC, bool, O>;
#[doc = "Field `OPEN_DRAIN` reader - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
pub type OPEN_DRAIN_R = crate::BitReader<bool>;
#[doc = "Field `OPEN_DRAIN` writer - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
pub type OPEN_DRAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
    #[inline(always)]
    pub fn open_drain(&self) -> OPEN_DRAIN_R {
        OPEN_DRAIN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ and CMD_RESP mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<0> {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'. For EZ and CMD_RESP this field must be set to '1'"]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<8> {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated SCB output signals 'xxx_out_en' and 'xxx_out'. This field determines how the SCB controls those two signals. Consult the GPIO chapter in the architecture TRM to understand how the pin drive modes behave when connected to SCBs. '0': Normal operation mode. In this operation mode 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. In this operation mode 'xxx_out_en' output controls the outputted value. Typically the 'xxx_out' signal is a constant '0'. Thus when 'xxx_out_en' is '1' the line is driven low, but when 'xxx_out_en' is '0' the output is not driven. This requires that the line is driven high by an external device or pull-up resistor The open drain mode is supported for: - I2C mode this field must be set. - UART mode use this mode when a pull-up resistor is used on the TX line. - SPI mode this field must be set if there are multiple slaves driving MISO."]
    #[inline(always)]
    pub fn open_drain(&mut self) -> OPEN_DRAIN_W<16> {
        OPEN_DRAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ctrl](index.html) module"]
pub struct TX_CTRL_SPEC;
impl crate::RegisterSpec for TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ctrl::R](R) reader structure"]
impl crate::Readable for TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ctrl::W](W) writer structure"]
impl crate::Writable for TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x0107"]
impl crate::Resettable for TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
