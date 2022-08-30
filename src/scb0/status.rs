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
#[doc = "Field `EC_BUSY` reader - Inidicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ and CMD_RESP mode). This bit can be used by SW to determine whether it is safe for the CPU to access the EZ memory (without bus wait states (a blocked CPU access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether CPU access was actually blocked by externally clocked logic."]
pub type EC_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Inidicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ and CMD_RESP mode). This bit can be used by SW to determine whether it is safe for the CPU to access the EZ memory (without bus wait states (a blocked CPU access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether CPU access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub fn ec_busy(&self) -> EC_BUSY_R {
        EC_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Generic status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
