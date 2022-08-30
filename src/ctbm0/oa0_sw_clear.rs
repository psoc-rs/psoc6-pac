#[doc = "Register `OA0_SW_CLEAR` reader"]
pub struct R(crate::R<OA0_SW_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA0_SW_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA0_SW_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA0_SW_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA0_SW_CLEAR` writer"]
pub struct W(crate::W<OA0_SW_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA0_SW_CLEAR_SPEC>;
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
impl From<crate::W<OA0_SW_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA0_SW_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0P_A00` reader - see corresponding bit in OA0_SW"]
pub type OA0P_A00_R = crate::BitReader<bool>;
#[doc = "Field `OA0P_A00` writer - see corresponding bit in OA0_SW"]
pub type OA0P_A00_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0P_A20` reader - see corresponding bit in OA0_SW"]
pub type OA0P_A20_R = crate::BitReader<bool>;
#[doc = "Field `OA0P_A20` writer - see corresponding bit in OA0_SW"]
pub type OA0P_A20_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0P_A30` reader - see corresponding bit in OA0_SW"]
pub type OA0P_A30_R = crate::BitReader<bool>;
#[doc = "Field `OA0P_A30` writer - see corresponding bit in OA0_SW"]
pub type OA0P_A30_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0M_A11` reader - see corresponding bit in OA0_SW"]
pub type OA0M_A11_R = crate::BitReader<bool>;
#[doc = "Field `OA0M_A11` writer - see corresponding bit in OA0_SW"]
pub type OA0M_A11_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0M_A81` reader - see corresponding bit in OA0_SW"]
pub type OA0M_A81_R = crate::BitReader<bool>;
#[doc = "Field `OA0M_A81` writer - see corresponding bit in OA0_SW"]
pub type OA0M_A81_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0O_D51` reader - see corresponding bit in OA0_SW"]
pub type OA0O_D51_R = crate::BitReader<bool>;
#[doc = "Field `OA0O_D51` writer - see corresponding bit in OA0_SW"]
pub type OA0O_D51_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `OA0O_D81` reader - see corresponding bit in OA0_SW"]
pub type OA0O_D81_R = crate::BitReader<bool>;
#[doc = "Field `OA0O_D81` writer - see corresponding bit in OA0_SW"]
pub type OA0O_D81_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA0_SW_CLEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&self) -> OA0P_A00_R {
        OA0P_A00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&self) -> OA0P_A20_R {
        OA0P_A20_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&self) -> OA0P_A30_R {
        OA0P_A30_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&self) -> OA0M_A11_R {
        OA0M_A11_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&self) -> OA0M_A81_R {
        OA0M_A81_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&self) -> OA0O_D51_R {
        OA0O_D51_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&self) -> OA0O_D81_R {
        OA0O_D81_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a00(&mut self) -> OA0P_A00_W<0> {
        OA0P_A00_W::new(self)
    }
    #[doc = "Bit 2 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a20(&mut self) -> OA0P_A20_W<2> {
        OA0P_A20_W::new(self)
    }
    #[doc = "Bit 3 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0p_a30(&mut self) -> OA0P_A30_W<3> {
        OA0P_A30_W::new(self)
    }
    #[doc = "Bit 8 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a11(&mut self) -> OA0M_A11_W<8> {
        OA0M_A11_W::new(self)
    }
    #[doc = "Bit 14 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0m_a81(&mut self) -> OA0M_A81_W<14> {
        OA0M_A81_W::new(self)
    }
    #[doc = "Bit 18 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d51(&mut self) -> OA0O_D51_W<18> {
        OA0O_D51_W::new(self)
    }
    #[doc = "Bit 21 - see corresponding bit in OA0_SW"]
    #[inline(always)]
    pub fn oa0o_d81(&mut self) -> OA0O_D81_W<21> {
        OA0O_D81_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp0 switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_sw_clear](index.html) module"]
pub struct OA0_SW_CLEAR_SPEC;
impl crate::RegisterSpec for OA0_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa0_sw_clear::R](R) reader structure"]
impl crate::Readable for OA0_SW_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa0_sw_clear::W](W) writer structure"]
impl crate::Writable for OA0_SW_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA0_SW_CLEAR to value 0"]
impl crate::Resettable for OA0_SW_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
