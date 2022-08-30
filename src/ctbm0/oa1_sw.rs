#[doc = "Register `OA1_SW` reader"]
pub struct R(crate::R<OA1_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA1_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA1_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA1_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA1_SW` writer"]
pub struct W(crate::W<OA1_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA1_SW_SPEC>;
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
impl From<crate::W<OA1_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA1_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA1P_A03` reader - Opamp1 positive terminal amuxbusb"]
pub type OA1P_A03_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A03` writer - Opamp1 positive terminal amuxbusb"]
pub type OA1P_A03_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1P_A13` reader - Opamp1 positive terminal P5"]
pub type OA1P_A13_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A13` writer - Opamp1 positive terminal P5"]
pub type OA1P_A13_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1P_A43` reader - Opamp1 positive terminal ctbbus1"]
pub type OA1P_A43_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A43` writer - Opamp1 positive terminal ctbbus1"]
pub type OA1P_A43_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1P_A73` reader - Opamp1 positive terminal to vref1"]
pub type OA1P_A73_R = crate::BitReader<bool>;
#[doc = "Field `OA1P_A73` writer - Opamp1 positive terminal to vref1"]
pub type OA1P_A73_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1M_A22` reader - Opamp1 negative terminal P4"]
pub type OA1M_A22_R = crate::BitReader<bool>;
#[doc = "Field `OA1M_A22` writer - Opamp1 negative terminal P4"]
pub type OA1M_A22_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1M_A82` reader - Opamp1 negative terminal Opamp1 output"]
pub type OA1M_A82_R = crate::BitReader<bool>;
#[doc = "Field `OA1M_A82` writer - Opamp1 negative terminal Opamp1 output"]
pub type OA1M_A82_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1O_D52` reader - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
pub type OA1O_D52_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D52` writer - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
pub type OA1O_D52_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1O_D62` reader - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
pub type OA1O_D62_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D62` writer - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
pub type OA1O_D62_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
#[doc = "Field `OA1O_D82` reader - Opamp1 output switch to short 1x with 10x drive"]
pub type OA1O_D82_R = crate::BitReader<bool>;
#[doc = "Field `OA1O_D82` writer - Opamp1 output switch to short 1x with 10x drive"]
pub type OA1O_D82_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA1_SW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub fn oa1p_a03(&self) -> OA1P_A03_R {
        OA1P_A03_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Opamp1 positive terminal P5"]
    #[inline(always)]
    pub fn oa1p_a13(&self) -> OA1P_A13_R {
        OA1P_A13_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub fn oa1p_a43(&self) -> OA1P_A43_R {
        OA1P_A43_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Opamp1 positive terminal to vref1"]
    #[inline(always)]
    pub fn oa1p_a73(&self) -> OA1P_A73_R {
        OA1P_A73_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Opamp1 negative terminal P4"]
    #[inline(always)]
    pub fn oa1m_a22(&self) -> OA1M_A22_R {
        OA1M_A22_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub fn oa1m_a82(&self) -> OA1M_A82_R {
        OA1M_A82_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d52(&self) -> OA1O_D52_R {
        OA1O_D52_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d62(&self) -> OA1O_D62_R {
        OA1O_D62_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Opamp1 output switch to short 1x with 10x drive"]
    #[inline(always)]
    pub fn oa1o_d82(&self) -> OA1O_D82_R {
        OA1O_D82_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Opamp1 positive terminal amuxbusb"]
    #[inline(always)]
    pub fn oa1p_a03(&mut self) -> OA1P_A03_W<0> {
        OA1P_A03_W::new(self)
    }
    #[doc = "Bit 1 - Opamp1 positive terminal P5"]
    #[inline(always)]
    pub fn oa1p_a13(&mut self) -> OA1P_A13_W<1> {
        OA1P_A13_W::new(self)
    }
    #[doc = "Bit 4 - Opamp1 positive terminal ctbbus1"]
    #[inline(always)]
    pub fn oa1p_a43(&mut self) -> OA1P_A43_W<4> {
        OA1P_A43_W::new(self)
    }
    #[doc = "Bit 7 - Opamp1 positive terminal to vref1"]
    #[inline(always)]
    pub fn oa1p_a73(&mut self) -> OA1P_A73_W<7> {
        OA1P_A73_W::new(self)
    }
    #[doc = "Bit 8 - Opamp1 negative terminal P4"]
    #[inline(always)]
    pub fn oa1m_a22(&mut self) -> OA1M_A22_W<8> {
        OA1M_A22_W::new(self)
    }
    #[doc = "Bit 14 - Opamp1 negative terminal Opamp1 output"]
    #[inline(always)]
    pub fn oa1m_a82(&mut self) -> OA1M_A82_W<14> {
        OA1M_A82_W::new(self)
    }
    #[doc = "Bit 18 - Opamp1 output sarbus0 (ctbbus2 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d52(&mut self) -> OA1O_D52_W<18> {
        OA1O_D52_W::new(self)
    }
    #[doc = "Bit 19 - Opamp1 output sarbus1 (ctbbus3 in CTB)"]
    #[inline(always)]
    pub fn oa1o_d62(&mut self) -> OA1O_D62_W<19> {
        OA1O_D62_W::new(self)
    }
    #[doc = "Bit 21 - Opamp1 output switch to short 1x with 10x drive"]
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
#[doc = "Opamp1 switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa1_sw](index.html) module"]
pub struct OA1_SW_SPEC;
impl crate::RegisterSpec for OA1_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa1_sw::R](R) reader structure"]
impl crate::Readable for OA1_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa1_sw::W](W) writer structure"]
impl crate::Writable for OA1_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA1_SW to value 0"]
impl crate::Resettable for OA1_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
