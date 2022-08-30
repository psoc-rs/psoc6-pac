#[doc = "Register `MMMS_CONFIG` reader"]
pub struct R(crate::R<MMMS_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_CONFIG` writer"]
pub struct W(crate::W<MMMS_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_CONFIG_SPEC>;
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
impl From<crate::W<MMMS_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMMS_ENABLE` reader - Configuration bit to enable MMMS functionality"]
pub type MMMS_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `MMMS_ENABLE` writer - Configuration bit to enable MMMS functionality"]
pub type MMMS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
#[doc = "Field `DISABLE_CONN_REQ_PARAM_IN_MEM` reader - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type DISABLE_CONN_REQ_PARAM_IN_MEM_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_CONN_REQ_PARAM_IN_MEM` writer - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type DISABLE_CONN_REQ_PARAM_IN_MEM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
#[doc = "Field `DISABLE_CONN_PARAM_MEM_WR` reader - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type DISABLE_CONN_PARAM_MEM_WR_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_CONN_PARAM_MEM_WR` writer - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type DISABLE_CONN_PARAM_MEM_WR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
#[doc = "Field `CONN_PARAM_FROM_REG` reader - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type CONN_PARAM_FROM_REG_R = crate::BitReader<bool>;
#[doc = "Field `CONN_PARAM_FROM_REG` writer - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
pub type CONN_PARAM_FROM_REG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
#[doc = "Field `ADV_CONN_INDEX` reader - This field specifies the connection index for which ADV is enabled"]
pub type ADV_CONN_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADV_CONN_INDEX` writer - This field specifies the connection index for which ADV is enabled"]
pub type ADV_CONN_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMMS_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `CE_LEN_IMMEDIATE_EXPIRE` reader - Enable for CE length immediate expiry"]
pub type CE_LEN_IMMEDIATE_EXPIRE_R = crate::BitReader<bool>;
#[doc = "Field `CE_LEN_IMMEDIATE_EXPIRE` writer - Enable for CE length immediate expiry"]
pub type CE_LEN_IMMEDIATE_EXPIRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
#[doc = "Field `RESET_RX_FIFO_PTR` reader - Setting this bit resets the receive FIFO pointers"]
pub type RESET_RX_FIFO_PTR_R = crate::BitReader<bool>;
#[doc = "Field `RESET_RX_FIFO_PTR` writer - Setting this bit resets the receive FIFO pointers"]
pub type RESET_RX_FIFO_PTR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMMS_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configuration bit to enable MMMS functionality"]
    #[inline(always)]
    pub fn mmms_enable(&self) -> MMMS_ENABLE_R {
        MMMS_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_req_param_in_mem(&self) -> DISABLE_CONN_REQ_PARAM_IN_MEM_R {
        DISABLE_CONN_REQ_PARAM_IN_MEM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_param_mem_wr(&self) -> DISABLE_CONN_PARAM_MEM_WR_R {
        DISABLE_CONN_PARAM_MEM_WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn conn_param_from_reg(&self) -> CONN_PARAM_FROM_REG_R {
        CONN_PARAM_FROM_REG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - This field specifies the connection index for which ADV is enabled"]
    #[inline(always)]
    pub fn adv_conn_index(&self) -> ADV_CONN_INDEX_R {
        ADV_CONN_INDEX_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Enable for CE length immediate expiry"]
    #[inline(always)]
    pub fn ce_len_immediate_expire(&self) -> CE_LEN_IMMEDIATE_EXPIRE_R {
        CE_LEN_IMMEDIATE_EXPIRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Setting this bit resets the receive FIFO pointers"]
    #[inline(always)]
    pub fn reset_rx_fifo_ptr(&self) -> RESET_RX_FIFO_PTR_R {
        RESET_RX_FIFO_PTR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration bit to enable MMMS functionality"]
    #[inline(always)]
    pub fn mmms_enable(&mut self) -> MMMS_ENABLE_W<0> {
        MMMS_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_req_param_in_mem(&mut self) -> DISABLE_CONN_REQ_PARAM_IN_MEM_W<1> {
        DISABLE_CONN_REQ_PARAM_IN_MEM_W::new(self)
    }
    #[doc = "Bit 2 - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_param_mem_wr(&mut self) -> DISABLE_CONN_PARAM_MEM_WR_W<2> {
        DISABLE_CONN_PARAM_MEM_WR_W::new(self)
    }
    #[doc = "Bit 3 - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn conn_param_from_reg(&mut self) -> CONN_PARAM_FROM_REG_W<3> {
        CONN_PARAM_FROM_REG_W::new(self)
    }
    #[doc = "Bits 4:8 - This field specifies the connection index for which ADV is enabled"]
    #[inline(always)]
    pub fn adv_conn_index(&mut self) -> ADV_CONN_INDEX_W<4> {
        ADV_CONN_INDEX_W::new(self)
    }
    #[doc = "Bit 9 - Enable for CE length immediate expiry"]
    #[inline(always)]
    pub fn ce_len_immediate_expire(&mut self) -> CE_LEN_IMMEDIATE_EXPIRE_W<9> {
        CE_LEN_IMMEDIATE_EXPIRE_W::new(self)
    }
    #[doc = "Bit 10 - Setting this bit resets the receive FIFO pointers"]
    #[inline(always)]
    pub fn reset_rx_fifo_ptr(&mut self) -> RESET_RX_FIFO_PTR_W<10> {
        RESET_RX_FIFO_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Master Multi-Slave Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_config](index.html) module"]
pub struct MMMS_CONFIG_SPEC;
impl crate::RegisterSpec for MMMS_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_config::R](R) reader structure"]
impl crate::Readable for MMMS_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_config::W](W) writer structure"]
impl crate::Writable for MMMS_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_CONFIG to value 0"]
impl crate::Resettable for MMMS_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
