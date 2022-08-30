#[doc = "Register `RD_DUMMY_CTL` reader"]
pub struct R(crate::R<RD_DUMMY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_DUMMY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_DUMMY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_DUMMY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_DUMMY_CTL` writer"]
pub struct W(crate::W<RD_DUMMY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_DUMMY_CTL_SPEC>;
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
impl From<crate::W<RD_DUMMY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_DUMMY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE5` reader - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type SIZE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE5` writer - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type SIZE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_DUMMY_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRESENT` reader - Presence of dummy cycles: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `PRESENT` writer - Presence of dummy cycles: '0': not present '1': present"]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD_DUMMY_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn size5(&mut self) -> SIZE5_W<0> {
        SIZE5_W::new(self)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&mut self) -> PRESENT_W<31> {
        PRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read dummy control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_dummy_ctl](index.html) module"]
pub struct RD_DUMMY_CTL_SPEC;
impl crate::RegisterSpec for RD_DUMMY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_dummy_ctl::R](R) reader structure"]
impl crate::Readable for RD_DUMMY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_dummy_ctl::W](W) writer structure"]
impl crate::Writable for RD_DUMMY_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_DUMMY_CTL to value 0"]
impl crate::Resettable for RD_DUMMY_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
