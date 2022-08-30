#[doc = "Register `CMD_RESP_CTRL` reader"]
pub struct R(crate::R<CMD_RESP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_RESP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_RESP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_RESP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_RESP_CTRL` writer"]
pub struct W(crate::W<CMD_RESP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_RESP_CTRL_SPEC>;
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
impl From<crate::W<CMD_RESP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_RESP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE_RD_ADDR` reader - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_RD_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE_RD_ADDR` writer - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_RD_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_RESP_CTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `BASE_WR_ADDR` reader - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_WR_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE_WR_ADDR` writer - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BASE_WR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_RESP_CTRL_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&self) -> BASE_RD_ADDR_R {
        BASE_RD_ADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&self) -> BASE_WR_ADDR_R {
        BASE_WR_ADDR_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. At the start of a read transfer this BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&mut self) -> BASE_RD_ADDR_W<0> {
        BASE_RD_ADDR_W::new(self)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. At the start of a write transfer this BASE_WR_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&mut self) -> BASE_WR_ADDR_W<16> {
        BASE_WR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command/response control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_resp_ctrl](index.html) module"]
pub struct CMD_RESP_CTRL_SPEC;
impl crate::RegisterSpec for CMD_RESP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_resp_ctrl::R](R) reader structure"]
impl crate::Readable for CMD_RESP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_resp_ctrl::W](W) writer structure"]
impl crate::Writable for CMD_RESP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_RESP_CTRL to value 0"]
impl crate::Resettable for CMD_RESP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
