#[doc = "Register `SLOW_CA_CTL` reader"]
pub struct R(crate::R<SLOW_CA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_CA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_CA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_CA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_CA_CTL` writer"]
pub struct W(crate::W<SLOW_CA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_CA_CTL_SPEC>;
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
impl From<crate::W<SLOW_CA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_CA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAY` reader - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type WAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAY` writer - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type WAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOW_CA_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SET_ADDR` reader - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type SET_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SET_ADDR` writer - this is for debug purpose only, and should be hidden to customers in technical document"]
pub type SET_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOW_CA_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PREF_EN` reader - N/A"]
pub type PREF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREF_EN` writer - N/A"]
pub type PREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLOW_CA_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - N/A"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - N/A"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLOW_CA_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:17 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn way(&self) -> WAY_R {
        WAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn set_addr(&self) -> SET_ADDR_R {
        SET_ADDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn way(&mut self) -> WAY_W<16> {
        WAY_W::new(self)
    }
    #[doc = "Bits 24:25 - this is for debug purpose only, and should be hidden to customers in technical document"]
    #[inline(always)]
    pub fn set_addr(&mut self) -> SET_ADDR_W<24> {
        SET_ADDR_W::new(self)
    }
    #[doc = "Bit 30 - N/A"]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W<30> {
        PREF_EN_W::new(self)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow cache control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_ca_ctl](index.html) module"]
pub struct SLOW_CA_CTL_SPEC;
impl crate::RegisterSpec for SLOW_CA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_ca_ctl::R](R) reader structure"]
impl crate::Readable for SLOW_CA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_ca_ctl::W](W) writer structure"]
impl crate::Writable for SLOW_CA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLOW_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for SLOW_CA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}
