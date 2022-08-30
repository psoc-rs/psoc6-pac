#[doc = "Register `TIMEOUT_CTL` reader"]
pub struct R(crate::R<TIMEOUT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT_CTL` writer"]
pub struct W(crate::W<TIMEOUT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_CTL_SPEC>;
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
impl From<crate::W<TIMEOUT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUT` writer - This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMEOUT_CTL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field specifies a number of peripheral group (clk_group) clock cycles. If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of peripheral group clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout_ctl](index.html) module"]
pub struct TIMEOUT_CTL_SPEC;
impl crate::RegisterSpec for TIMEOUT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeout_ctl::R](R) reader structure"]
impl crate::Readable for TIMEOUT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout_ctl::W](W) writer structure"]
impl crate::Writable for TIMEOUT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEOUT_CTL to value 0xffff"]
impl crate::Resettable for TIMEOUT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
