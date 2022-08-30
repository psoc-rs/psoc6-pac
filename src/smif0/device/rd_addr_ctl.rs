#[doc = "Register `RD_ADDR_CTL` reader"]
pub struct R(crate::R<RD_ADDR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_ADDR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_ADDR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_ADDR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_ADDR_CTL` writer"]
pub struct W(crate::W<RD_ADDR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_ADDR_CTL_SPEC>;
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
impl From<crate::W<RD_ADDR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_ADDR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_ADDR_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W<16> {
        WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read address control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_addr_ctl](index.html) module"]
pub struct RD_ADDR_CTL_SPEC;
impl crate::RegisterSpec for RD_ADDR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_addr_ctl::R](R) reader structure"]
impl crate::Readable for RD_ADDR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_addr_ctl::W](W) writer structure"]
impl crate::Writable for RD_ADDR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_ADDR_CTL to value 0"]
impl crate::Resettable for RD_ADDR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
