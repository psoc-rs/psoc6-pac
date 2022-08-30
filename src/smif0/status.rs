#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
