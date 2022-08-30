#[doc = "Register `US_OFFSET_STATUS` reader"]
pub struct R(crate::R<US_OFFSET_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_OFFSET_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_OFFSET_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_OFFSET_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `US_OFFSET` reader - During slave connection event, HW updates this register with the calculated us_offset at anchor point, granularity is 1us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0x00D5."]
pub type US_OFFSET_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - During slave connection event, HW updates this register with the calculated us_offset at anchor point, granularity is 1us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0x00D5."]
    #[inline(always)]
    pub fn us_offset(&self) -> US_OFFSET_R {
        US_OFFSET_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Micro-second Offset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_offset_status](index.html) module"]
pub struct US_OFFSET_STATUS_SPEC;
impl crate::RegisterSpec for US_OFFSET_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_offset_status::R](R) reader structure"]
impl crate::Readable for US_OFFSET_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_OFFSET_STATUS to value 0xd5"]
impl crate::Resettable for US_OFFSET_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd5
    }
}
