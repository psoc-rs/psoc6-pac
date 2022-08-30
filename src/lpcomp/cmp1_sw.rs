#[doc = "Register `CMP1_SW` reader"]
pub struct R(crate::R<CMP1_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP1_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP1_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP1_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP1_SW` writer"]
pub struct W(crate::W<CMP1_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP1_SW_SPEC>;
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
impl From<crate::W<CMP1_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP1_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1_IP1` reader - Comparator 1 positive terminal isolation switch to GPIO"]
pub type CMP1_IP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_IP1` writer - Comparator 1 positive terminal isolation switch to GPIO"]
pub type CMP1_IP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_AP1` reader - Comparator 1 positive terminal switch to amuxbusA"]
pub type CMP1_AP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_AP1` writer - Comparator 1 positive terminal switch to amuxbusA"]
pub type CMP1_AP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_BP1` reader - Comparator 1 positive terminal switch to amuxbusB"]
pub type CMP1_BP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_BP1` writer - Comparator 1 positive terminal switch to amuxbusB"]
pub type CMP1_BP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_IN1` reader - Comparator 1 negative terminal isolation switch to GPIO"]
pub type CMP1_IN1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_IN1` writer - Comparator 1 negative terminal isolation switch to GPIO"]
pub type CMP1_IN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_AN1` reader - Comparator 1 negative terminal switch to amuxbusA"]
pub type CMP1_AN1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_AN1` writer - Comparator 1 negative terminal switch to amuxbusA"]
pub type CMP1_AN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_BN1` reader - Comparator 1 negative terminal switch to amuxbusB"]
pub type CMP1_BN1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_BN1` writer - Comparator 1 negative terminal switch to amuxbusB"]
pub type CMP1_BN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
#[doc = "Field `CMP1_VN1` reader - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type CMP1_VN1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1_VN1` writer - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type CMP1_VN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP1_SW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> CMP1_IP1_R {
        CMP1_IP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> CMP1_AP1_R {
        CMP1_AP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> CMP1_BP1_R {
        CMP1_BP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> CMP1_IN1_R {
        CMP1_IN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> CMP1_AN1_R {
        CMP1_AN1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> CMP1_BN1_R {
        CMP1_BN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> CMP1_VN1_R {
        CMP1_VN1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&mut self) -> CMP1_IP1_W<0> {
        CMP1_IP1_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&mut self) -> CMP1_AP1_W<1> {
        CMP1_AP1_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&mut self) -> CMP1_BP1_W<2> {
        CMP1_BP1_W::new(self)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&mut self) -> CMP1_IN1_W<4> {
        CMP1_IN1_W::new(self)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&mut self) -> CMP1_AN1_W<5> {
        CMP1_AN1_W::new(self)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&mut self) -> CMP1_BN1_W<6> {
        CMP1_BN1_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&mut self) -> CMP1_VN1_W<7> {
        CMP1_VN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_sw](index.html) module"]
pub struct CMP1_SW_SPEC;
impl crate::RegisterSpec for CMP1_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp1_sw::R](R) reader structure"]
impl crate::Readable for CMP1_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp1_sw::W](W) writer structure"]
impl crate::Writable for CMP1_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP1_SW to value 0"]
impl crate::Resettable for CMP1_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
