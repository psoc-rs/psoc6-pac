#[doc = "Reader of register CONN_RXMEM_BASE_ADDR_DLE"]
pub type R = crate::R<u32, super::CONN_RXMEM_BASE_ADDR_DLE>;
#[doc = "Writer for register CONN_RXMEM_BASE_ADDR_DLE"]
pub type W = crate::W<u32, super::CONN_RXMEM_BASE_ADDR_DLE>;
#[doc = "Register CONN_RXMEM_BASE_ADDR_DLE `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_RXMEM_BASE_ADDR_DLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_RX_MEM_BASE_ADDR_DLE`"]
pub type CONN_RX_MEM_BASE_ADDR_DLE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CONN_RX_MEM_BASE_ADDR_DLE`"]
pub struct CONN_RX_MEM_BASE_ADDR_DLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_RX_MEM_BASE_ADDR_DLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data from Rx memory are read as 32-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn conn_rx_mem_base_addr_dle(&self) -> CONN_RX_MEM_BASE_ADDR_DLE_R {
        CONN_RX_MEM_BASE_ADDR_DLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data from Rx memory are read as 32-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn conn_rx_mem_base_addr_dle(&mut self) -> CONN_RX_MEM_BASE_ADDR_DLE_W {
        CONN_RX_MEM_BASE_ADDR_DLE_W { w: self }
    }
}
