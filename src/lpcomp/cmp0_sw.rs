#[doc = "Register `CMP0_SW` reader"]
pub struct R(crate::R<CMP0_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP0_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP0_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP0_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP0_SW` writer"]
pub struct W(crate::W<CMP0_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP0_SW_SPEC>;
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
impl From<crate::W<CMP0_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP0_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0_IP0` reader - Comparator 0 positive terminal isolation switch to GPIO"]
pub type CMP0_IP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_IP0` writer - Comparator 0 positive terminal isolation switch to GPIO"]
pub type CMP0_IP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_AP0` reader - Comparator 0 positive terminal switch to amuxbusA"]
pub type CMP0_AP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_AP0` writer - Comparator 0 positive terminal switch to amuxbusA"]
pub type CMP0_AP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_BP0` reader - Comparator 0 positive terminal switch to amuxbusB"]
pub type CMP0_BP0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_BP0` writer - Comparator 0 positive terminal switch to amuxbusB"]
pub type CMP0_BP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_IN0` reader - Comparator 0 negative terminal isolation switch to GPIO"]
pub type CMP0_IN0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_IN0` writer - Comparator 0 negative terminal isolation switch to GPIO"]
pub type CMP0_IN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_AN0` reader - Comparator 0 negative terminal switch to amuxbusA"]
pub type CMP0_AN0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_AN0` writer - Comparator 0 negative terminal switch to amuxbusA"]
pub type CMP0_AN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_BN0` reader - Comparator 0 negative terminal switch to amuxbusB"]
pub type CMP0_BN0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_BN0` writer - Comparator 0 negative terminal switch to amuxbusB"]
pub type CMP0_BN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
#[doc = "Field `CMP0_VN0` reader - Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type CMP0_VN0_R = crate::BitReader<bool>;
#[doc = "Field `CMP0_VN0` writer - Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type CMP0_VN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP0_SW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 0 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp0_ip0(&self) -> CMP0_IP0_R {
        CMP0_IP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 0 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp0_ap0(&self) -> CMP0_AP0_R {
        CMP0_AP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 0 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp0_bp0(&self) -> CMP0_BP0_R {
        CMP0_BP0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator 0 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp0_in0(&self) -> CMP0_IN0_R {
        CMP0_IN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator 0 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp0_an0(&self) -> CMP0_AN0_R {
        CMP0_AN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator 0 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp0_bn0(&self) -> CMP0_BN0_R {
        CMP0_BN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp0_vn0(&self) -> CMP0_VN0_R {
        CMP0_VN0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp0_ip0(&mut self) -> CMP0_IP0_W<0> {
        CMP0_IP0_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 0 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp0_ap0(&mut self) -> CMP0_AP0_W<1> {
        CMP0_AP0_W::new(self)
    }
    #[doc = "Bit 2 - Comparator 0 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp0_bp0(&mut self) -> CMP0_BP0_W<2> {
        CMP0_BP0_W::new(self)
    }
    #[doc = "Bit 4 - Comparator 0 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp0_in0(&mut self) -> CMP0_IN0_W<4> {
        CMP0_IN0_W::new(self)
    }
    #[doc = "Bit 5 - Comparator 0 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp0_an0(&mut self) -> CMP0_AN0_W<5> {
        CMP0_AN0_W::new(self)
    }
    #[doc = "Bit 6 - Comparator 0 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp0_bn0(&mut self) -> CMP0_BN0_W<6> {
        CMP0_BN0_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 0 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp0_vn0(&mut self) -> CMP0_VN0_W<7> {
        CMP0_VN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 0 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp0_sw](index.html) module"]
pub struct CMP0_SW_SPEC;
impl crate::RegisterSpec for CMP0_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp0_sw::R](R) reader structure"]
impl crate::Readable for CMP0_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp0_sw::W](W) writer structure"]
impl crate::Writable for CMP0_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP0_SW to value 0"]
impl crate::Resettable for CMP0_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
