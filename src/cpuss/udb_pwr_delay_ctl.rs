#[doc = "Register `UDB_PWR_DELAY_CTL` reader"]
pub struct R(crate::R<UDB_PWR_DELAY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDB_PWR_DELAY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDB_PWR_DELAY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDB_PWR_DELAY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDB_PWR_DELAY_CTL` writer"]
pub struct W(crate::W<UDB_PWR_DELAY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDB_PWR_DELAY_CTL_SPEC>;
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
impl From<crate::W<UDB_PWR_DELAY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDB_PWR_DELAY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UP` reader - Number clock cycles delay needed after power domain power up"]
pub type UP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UP` writer - Number clock cycles delay needed after power domain power up"]
pub type UP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UDB_PWR_DELAY_CTL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<0> {
        UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UDB power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udb_pwr_delay_ctl](index.html) module"]
pub struct UDB_PWR_DELAY_CTL_SPEC;
impl crate::RegisterSpec for UDB_PWR_DELAY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udb_pwr_delay_ctl::R](R) reader structure"]
impl crate::Readable for UDB_PWR_DELAY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udb_pwr_delay_ctl::W](W) writer structure"]
impl crate::Writable for UDB_PWR_DELAY_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDB_PWR_DELAY_CTL to value 0x012c"]
impl crate::Resettable for UDB_PWR_DELAY_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x012c
    }
}
