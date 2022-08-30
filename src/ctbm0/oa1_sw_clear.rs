#[doc = "Register `OA1_SW_CLEAR` reader"]
pub struct R(crate::R<OA1_SW_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA1_SW_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA1_SW_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA1_SW_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA1_SW_CLEAR` writer"]
pub struct W(crate::W<OA1_SW_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA1_SW_CLEAR_SPEC>;
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
impl From<crate::W<OA1_SW_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA1_SW_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1P_A03` reader - see corresponding bit in OA1_SW"]
pub type OA1P_A03_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A03` writer - see corresponding bit in OA1_SW"]
pub type OA1P_A03_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1P_A13` reader - see corresponding bit in OA1_SW"]
pub type OA1P_A13_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A13` writer - see corresponding bit in OA1_SW"]
pub type OA1P_A13_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1P_A43` reader - see corresponding bit in OA1_SW"]
pub type OA1P_A43_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A43` writer - see corresponding bit in OA1_SW"]
pub type OA1P_A43_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1P_A73` reader - see corresponding bit in OA1_SW"]
pub type OA1P_A73_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A73` writer - see corresponding bit in OA1_SW"]
pub type OA1P_A73_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1M_A22` reader - see corresponding bit in OA1_SW"]
pub type OA1M_A22_R = crate::BitReader<bool>;
#[doc = "Field `OA1M_A22` writer - see corresponding bit in OA1_SW"]
pub type OA1M_A22_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1M_A82` reader - see corresponding bit in OA1_SW"]
pub type OA1M_A82_R = crate::BitReader<bool>;
#[doc = "Field `OA1M_A82` writer - see corresponding bit in OA1_SW"]
pub type OA1M_A82_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1O_D52` reader - see corresponding bit in OA1_SW"]
pub type OA1O_D52_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D52` writer - see corresponding bit in OA1_SW"]
pub type OA1O_D52_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1O_D62` reader - see corresponding bit in OA1_SW"]
pub type OA1O_D62_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D62` writer - see corresponding bit in OA1_SW"]
pub type OA1O_D62_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA1O_D82` reader - see corresponding bit in OA1_SW"]
pub type OA1O_D82_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D82` writer - see corresponding bit in OA1_SW"]
pub type OA1O_D82_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_CLEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a03(&self) -> OA1P_A03_R {
        OA1P_A03_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a13(&self) -> OA1P_A13_R {
        OA1P_A13_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a43(&self) -> OA1P_A43_R {
        OA1P_A43_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a73(&self) -> OA1P_A73_R {
        OA1P_A73_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a22(&self) -> OA1M_A22_R {
        OA1M_A22_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a82(&self) -> OA1M_A82_R {
        OA1M_A82_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52(&self) -> OA1O_D52_R {
        OA1O_D52_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62(&self) -> OA1O_D62_R {
        OA1O_D62_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d82(&self) -> OA1O_D82_R {
        OA1O_D82_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a03(&mut self) -> OA1P_A03_W<0> {
        OA1P_A03_W::new(self)
    }
    #[doc = "Bit 1 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a13(&mut self) -> OA1P_A13_W<1> {
        OA1P_A13_W::new(self)
    }
    #[doc = "Bit 4 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a43(&mut self) -> OA1P_A43_W<4> {
        OA1P_A43_W::new(self)
    }
    #[doc = "Bit 7 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1p_a73(&mut self) -> OA1P_A73_W<7> {
        OA1P_A73_W::new(self)
    }
    #[doc = "Bit 8 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a22(&mut self) -> OA1M_A22_W<8> {
        OA1M_A22_W::new(self)
    }
    #[doc = "Bit 14 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1m_a82(&mut self) -> OA1M_A82_W<14> {
        OA1M_A82_W::new(self)
    }
    #[doc = "Bit 18 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d52(&mut self) -> OA1O_D52_W<18> {
        OA1O_D52_W::new(self)
    }
    #[doc = "Bit 19 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d62(&mut self) -> OA1O_D62_W<19> {
        OA1O_D62_W::new(self)
    }
    #[doc = "Bit 21 - see corresponding bit in OA1_SW"]
    #[inline(always)]
    pub fn oa1o_d82(&mut self) -> OA1O_D82_W<21> {
        OA1O_D82_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp1 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_sw_clear](index.html) module"]
pub struct OA1_SW_CLEAR_SPEC;
impl crate::RegisterSpec for OA1_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa1_sw_clear::R](R) reader structure"]
impl crate::Readable for OA1_SW_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa1_sw_clear::W](W) writer structure"]
impl crate::Writable for OA1_SW_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA1_SW_CLEAR to value 0"]
impl crate::Resettable for OA1_SW_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
