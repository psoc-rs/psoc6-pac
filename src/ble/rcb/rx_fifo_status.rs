#[doc = "Register `RX_FIFO_STATUS` reader"]
pub struct R(crate::R<RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USED` reader - Amount of enties in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SR_VALID` reader - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
pub type SR_VALID_R = crate::BitReader<bool>;
#[doc = "Field `RD_PTR` reader - FIFO read pointer: FIFO location from which a data frame is read."]
pub type RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_PTR` reader - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
pub type WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Amount of enties in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - FIFO read pointer: FIFO location from which a data frame is read."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Receiver FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_status](index.html) module"]
pub struct RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_status::R](R) reader structure"]
impl crate::Readable for RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_FIFO_STATUS to value 0"]
impl crate::Resettable for RX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
