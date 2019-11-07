#[doc = "Reader of register DDFT_CTRL"]
pub type R = crate::R<u32, super::DDFT_CTRL>;
#[doc = "Writer for register DDFT_CTRL"]
pub type W = crate::W<u32, super::DDFT_CTRL>;
#[doc = "Register DDFT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DDFT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDFT_IN0_SEL`"]
pub type DDFT_IN0_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDFT_IN0_SEL`"]
pub struct DDFT_IN0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_IN0_SEL_W<'a> {
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
#[doc = "Reader of field `DDFT_IN1_SEL`"]
pub type DDFT_IN1_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDFT_IN1_SEL`"]
pub struct DDFT_IN1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_IN1_SEL_W<'a> {
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
#[doc = "Reader of field `DDFT_OUT0_SEL`"]
pub type DDFT_OUT0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_OUT0_SEL`"]
pub struct DDFT_OUT0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_OUT0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `DDFT_OUT1_SEL`"]
pub type DDFT_OUT1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDFT_OUT1_SEL`"]
pub struct DDFT_OUT1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDFT_OUT1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies signal that is connected to 'ddft_in\\[0\\]' (digital DfT input signal 0): '0': not used '1': used as 'i2c_scl_in' in I2C mode, as 'spi_clk_in' in SPI mode"]
    #[inline(always)]
    pub fn ddft_in0_sel(&self) -> DDFT_IN0_SEL_R {
        DDFT_IN0_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Specifies signal that is connected to 'ddft_in\\[1\\]' (digital DfT input signal 0): '0': not used '1': used as 'i2c_sda_in' in I2C mode, as 'spi_mosi_in' in SPI mode"]
    #[inline(always)]
    pub fn ddft_in1_sel(&self) -> DDFT_IN1_SEL_R {
        DDFT_IN1_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Specifies signal that is connected to 'ddft_out\\[0\\]' (digital DfT output signal 0): In I2C mode (CTRL.MODE=0), '0': Constant '0'. '1': 'ec_busy_pp'. '2': 'rst_i2c_start_stop_n'. '3': 'rst_i2c_start_stop_n'. '4': 'i2c_scl_in_qual'. '5': 'i2c_sda_out_prel'. '6'-'7': Undefined. in SPI mode (CTRL.MODE=1), '0': Constant '0'. '1': 'rst_spi_n' '2': 'rst_spi_stop_n' '3'-'7': Undefined."]
    #[inline(always)]
    pub fn ddft_out0_sel(&self) -> DDFT_OUT0_SEL_R {
        DDFT_OUT0_SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Specifies signal that is connected to 'ddft_out\\[1\\]' (digital DfT output signal 1): In I2C mode (CTRL.MODE=0), '0': Constant '0'. '1': 'clk_ff_sram'. '2': 'rst_i2c_n'. '3': 'rst_i2c_stop_n'. '4': 'i2c_sda_in_qual'. '5': 'i2c_sda_out'. '6': 'event_i2c_ec_wake_up_ddft' from I2CS_IC '7': Undefined. In SPI mode (CTRL.MODE=1), '0': Constant '0'. '1': 'spi_start_detect' '2': 'spi_stop_detect' '3'-'7': Undefined."]
    #[inline(always)]
    pub fn ddft_out1_sel(&self) -> DDFT_OUT1_SEL_R {
        DDFT_OUT1_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies signal that is connected to 'ddft_in\\[0\\]' (digital DfT input signal 0): '0': not used '1': used as 'i2c_scl_in' in I2C mode, as 'spi_clk_in' in SPI mode"]
    #[inline(always)]
    pub fn ddft_in0_sel(&mut self) -> DDFT_IN0_SEL_W {
        DDFT_IN0_SEL_W { w: self }
    }
    #[doc = "Bit 4 - Specifies signal that is connected to 'ddft_in\\[1\\]' (digital DfT input signal 0): '0': not used '1': used as 'i2c_sda_in' in I2C mode, as 'spi_mosi_in' in SPI mode"]
    #[inline(always)]
    pub fn ddft_in1_sel(&mut self) -> DDFT_IN1_SEL_W {
        DDFT_IN1_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Specifies signal that is connected to 'ddft_out\\[0\\]' (digital DfT output signal 0): In I2C mode (CTRL.MODE=0), '0': Constant '0'. '1': 'ec_busy_pp'. '2': 'rst_i2c_start_stop_n'. '3': 'rst_i2c_start_stop_n'. '4': 'i2c_scl_in_qual'. '5': 'i2c_sda_out_prel'. '6'-'7': Undefined. in SPI mode (CTRL.MODE=1), '0': Constant '0'. '1': 'rst_spi_n' '2': 'rst_spi_stop_n' '3'-'7': Undefined."]
    #[inline(always)]
    pub fn ddft_out0_sel(&mut self) -> DDFT_OUT0_SEL_W {
        DDFT_OUT0_SEL_W { w: self }
    }
    #[doc = "Bits 20:22 - Specifies signal that is connected to 'ddft_out\\[1\\]' (digital DfT output signal 1): In I2C mode (CTRL.MODE=0), '0': Constant '0'. '1': 'clk_ff_sram'. '2': 'rst_i2c_n'. '3': 'rst_i2c_stop_n'. '4': 'i2c_sda_in_qual'. '5': 'i2c_sda_out'. '6': 'event_i2c_ec_wake_up_ddft' from I2CS_IC '7': Undefined. In SPI mode (CTRL.MODE=1), '0': Constant '0'. '1': 'spi_start_detect' '2': 'spi_stop_detect' '3'-'7': Undefined."]
    #[inline(always)]
    pub fn ddft_out1_sel(&mut self) -> DDFT_OUT1_SEL_W {
        DDFT_OUT1_SEL_W { w: self }
    }
}
