#[doc = "Register `FLOW_CTL` reader"]
pub struct R(crate::R<FLOW_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW_CTL` writer"]
pub struct W(crate::W<FLOW_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_CTL_SPEC>;
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
impl From<crate::W<FLOW_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_ERR_RESP` reader - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type EP1_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP1_ERR_RESP` writer - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub type EP1_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP2_ERR_RESP` reader - End Point 2 error response"]
pub type EP2_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP2_ERR_RESP` writer - End Point 2 error response"]
pub type EP2_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP3_ERR_RESP` reader - End Point 3 error response"]
pub type EP3_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP3_ERR_RESP` writer - End Point 3 error response"]
pub type EP3_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP4_ERR_RESP` reader - End Point 4 error response"]
pub type EP4_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP4_ERR_RESP` writer - End Point 4 error response"]
pub type EP4_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP5_ERR_RESP` reader - End Point 5 error response"]
pub type EP5_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP5_ERR_RESP` writer - End Point 5 error response"]
pub type EP5_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP6_ERR_RESP` reader - End Point 6 error response"]
pub type EP6_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP6_ERR_RESP` writer - End Point 6 error response"]
pub type EP6_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP7_ERR_RESP` reader - End Point 7 error response"]
pub type EP7_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP7_ERR_RESP` writer - End Point 7 error response"]
pub type EP7_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
#[doc = "Field `EP8_ERR_RESP` reader - End Point 8 error response"]
pub type EP8_ERR_RESP_R = crate::BitReader<bool>;
#[doc = "Field `EP8_ERR_RESP` writer - End Point 8 error response"]
pub type EP8_ERR_RESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOW_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&self) -> EP1_ERR_RESP_R {
        EP1_ERR_RESP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&self) -> EP2_ERR_RESP_R {
        EP2_ERR_RESP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&self) -> EP3_ERR_RESP_R {
        EP3_ERR_RESP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&self) -> EP4_ERR_RESP_R {
        EP4_ERR_RESP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&self) -> EP5_ERR_RESP_R {
        EP5_ERR_RESP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&self) -> EP6_ERR_RESP_R {
        EP6_ERR_RESP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&self) -> EP7_ERR_RESP_R {
        EP7_ERR_RESP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&self) -> EP8_ERR_RESP_R {
        EP8_ERR_RESP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&mut self) -> EP1_ERR_RESP_W<0> {
        EP1_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&mut self) -> EP2_ERR_RESP_W<1> {
        EP2_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&mut self) -> EP3_ERR_RESP_W<2> {
        EP3_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&mut self) -> EP4_ERR_RESP_W<3> {
        EP4_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&mut self) -> EP5_ERR_RESP_W<4> {
        EP5_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&mut self) -> EP6_ERR_RESP_W<5> {
        EP6_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&mut self) -> EP7_ERR_RESP_W<6> {
        EP7_ERR_RESP_W::new(self)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&mut self) -> EP8_ERR_RESP_W<7> {
        EP8_ERR_RESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_ctl](index.html) module"]
pub struct FLOW_CTL_SPEC;
impl crate::RegisterSpec for FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow_ctl::R](R) reader structure"]
impl crate::Readable for FLOW_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow_ctl::W](W) writer structure"]
impl crate::Writable for FLOW_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOW_CTL to value 0"]
impl crate::Resettable for FLOW_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
