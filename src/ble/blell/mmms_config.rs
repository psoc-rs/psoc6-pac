#[doc = "Reader of register MMMS_CONFIG"]
pub type R = crate::R<u32, super::MMMS_CONFIG>;
#[doc = "Writer for register MMMS_CONFIG"]
pub type W = crate::W<u32, super::MMMS_CONFIG>;
#[doc = "Register MMMS_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::MMMS_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MMMS_ENABLE`"]
pub type MMMS_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMMS_ENABLE`"]
pub struct MMMS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MMMS_ENABLE_W<'a> {
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
#[doc = "Reader of field `DISABLE_CONN_REQ_PARAM_IN_MEM`"]
pub type DISABLE_CONN_REQ_PARAM_IN_MEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_CONN_REQ_PARAM_IN_MEM`"]
pub struct DISABLE_CONN_REQ_PARAM_IN_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_CONN_REQ_PARAM_IN_MEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DISABLE_CONN_PARAM_MEM_WR`"]
pub type DISABLE_CONN_PARAM_MEM_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_CONN_PARAM_MEM_WR`"]
pub struct DISABLE_CONN_PARAM_MEM_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_CONN_PARAM_MEM_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CONN_PARAM_FROM_REG`"]
pub type CONN_PARAM_FROM_REG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONN_PARAM_FROM_REG`"]
pub struct CONN_PARAM_FROM_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_PARAM_FROM_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADV_CONN_INDEX`"]
pub type ADV_CONN_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADV_CONN_INDEX`"]
pub struct ADV_CONN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_CONN_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CE_LEN_IMMEDIATE_EXPIRE`"]
pub type CE_LEN_IMMEDIATE_EXPIRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE_LEN_IMMEDIATE_EXPIRE`"]
pub struct CE_LEN_IMMEDIATE_EXPIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_LEN_IMMEDIATE_EXPIRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RESET_RX_FIFO_PTR`"]
pub type RESET_RX_FIFO_PTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_RX_FIFO_PTR`"]
pub struct RESET_RX_FIFO_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_RX_FIFO_PTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Configuration bit to enable MMMS functionality"]
    #[inline(always)]
    pub fn mmms_enable(&self) -> MMMS_ENABLE_R {
        MMMS_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_req_param_in_mem(&self) -> DISABLE_CONN_REQ_PARAM_IN_MEM_R {
        DISABLE_CONN_REQ_PARAM_IN_MEM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_param_mem_wr(&self) -> DISABLE_CONN_PARAM_MEM_WR_R {
        DISABLE_CONN_PARAM_MEM_WR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn conn_param_from_reg(&self) -> CONN_PARAM_FROM_REG_R {
        CONN_PARAM_FROM_REG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8 - This field specifies the connection index for which ADV is enabled"]
    #[inline(always)]
    pub fn adv_conn_index(&self) -> ADV_CONN_INDEX_R {
        ADV_CONN_INDEX_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Enable for CE length immediate expiry"]
    #[inline(always)]
    pub fn ce_len_immediate_expire(&self) -> CE_LEN_IMMEDIATE_EXPIRE_R {
        CE_LEN_IMMEDIATE_EXPIRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Setting this bit resets the receive FIFO pointers"]
    #[inline(always)]
    pub fn reset_rx_fifo_ptr(&self) -> RESET_RX_FIFO_PTR_R {
        RESET_RX_FIFO_PTR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration bit to enable MMMS functionality"]
    #[inline(always)]
    pub fn mmms_enable(&mut self) -> MMMS_ENABLE_W {
        MMMS_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1'b1 and MMMS enabled, then the parameters received in connection request are not stored in CONN_REQ_PARAM memory. By default this bit is 1'b0 and the connection request parameters are stored in connection memory. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_req_param_in_mem(&mut self) -> DISABLE_CONN_REQ_PARAM_IN_MEM_W {
        DISABLE_CONN_REQ_PARAM_IN_MEM_W { w: self }
    }
    #[doc = "Bit 2 - By default on end_ce, the connection parameters memory is loaded with the updated connection parameters. Setting this bit prevent's this update. This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn disable_conn_param_mem_wr(&mut self) -> DISABLE_CONN_PARAM_MEM_WR_W {
        DISABLE_CONN_PARAM_MEM_WR_W { w: self }
    }
    #[doc = "Bit 3 - By default the parameters for the connection are picked up from the connection parameters memory. Setting this bit disables this and the parameters are picked up from registers 0 - HW loads the parameters from connection memory 1 - Firmware should program the paramters for the connection event This bit is intended as a fail-safe. Should not be changed dynamically during runtime"]
    #[inline(always)]
    pub fn conn_param_from_reg(&mut self) -> CONN_PARAM_FROM_REG_W {
        CONN_PARAM_FROM_REG_W { w: self }
    }
    #[doc = "Bits 4:8 - This field specifies the connection index for which ADV is enabled"]
    #[inline(always)]
    pub fn adv_conn_index(&mut self) -> ADV_CONN_INDEX_W {
        ADV_CONN_INDEX_W { w: self }
    }
    #[doc = "Bit 9 - Enable for CE length immediate expiry"]
    #[inline(always)]
    pub fn ce_len_immediate_expire(&mut self) -> CE_LEN_IMMEDIATE_EXPIRE_W {
        CE_LEN_IMMEDIATE_EXPIRE_W { w: self }
    }
    #[doc = "Bit 10 - Setting this bit resets the receive FIFO pointers"]
    #[inline(always)]
    pub fn reset_rx_fifo_ptr(&mut self) -> RESET_RX_FIFO_PTR_W {
        RESET_RX_FIFO_PTR_W { w: self }
    }
}
