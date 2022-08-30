#[doc = "Register `CMD_RESP_STATUS` reader"]
pub struct R(crate::R<CMD_RESP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RESP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RESP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RESP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURR_RD_ADDR` reader - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable when there is no bus transfer. This field is potentially unreliable when there is a ongoing bus transfer, i.e. when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub type CURR_RD_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CURR_WR_ADDR` reader - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a write access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable when there is no bus transfer. This field is potentially unreliable when there is a ongoing bus transfer, i.e when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub type CURR_WR_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMD_RESP_EC_BUS_BUSY` reader - Indicates whether there is an ongoing bus transfer to the SCB. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when slave mode is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
pub type CMD_RESP_EC_BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `CMD_RESP_EC_BUSY` reader - N/A"]
pub type CMD_RESP_EC_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable when there is no bus transfer. This field is potentially unreliable when there is a ongoing bus transfer, i.e. when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_rd_addr(&self) -> CURR_RD_ADDR_R {
        CURR_RD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a write access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximim memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable when there is no bus transfer. This field is potentially unreliable when there is a ongoing bus transfer, i.e when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_wr_addr(&self) -> CURR_WR_ADDR_R {
        CURR_WR_ADDR_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Indicates whether there is an ongoing bus transfer to the SCB. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when slave mode is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
    #[inline(always)]
    pub fn cmd_resp_ec_bus_busy(&self) -> CMD_RESP_EC_BUS_BUSY_R {
        CMD_RESP_EC_BUS_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn cmd_resp_ec_busy(&self) -> CMD_RESP_EC_BUSY_R {
        CMD_RESP_EC_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Command/response status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_resp_status](index.html) module"]
pub struct CMD_RESP_STATUS_SPEC;
impl crate::RegisterSpec for CMD_RESP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_resp_status::R](R) reader structure"]
impl crate::Readable for CMD_RESP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_RESP_STATUS to value 0"]
impl crate::Resettable for CMD_RESP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
