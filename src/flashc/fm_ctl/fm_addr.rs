#[doc = "Register `FM_ADDR` reader"]
pub struct R(crate::R<FM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FM_ADDR` writer"]
pub struct W(crate::W<FM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FM_ADDR_SPEC>;
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
impl From<crate::W<FM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Row address."]
pub type RA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RA` writer - Row address."]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FM_ADDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `BA` reader - Bank address."]
pub type BA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BA` writer - Bank address."]
pub type BA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FM_ADDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `AXA` reader - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
pub type AXA_R = crate::BitReader<bool>;
#[doc = "Field `AXA` writer - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
pub type AXA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FM_ADDR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&self) -> AXA_R {
        AXA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Row address."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<0> {
        RA_W::new(self)
    }
    #[doc = "Bits 16:23 - Bank address."]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W<16> {
        BA_W::new(self)
    }
    #[doc = "Bit 24 - Auxiliary address field: '0': regular flash memory. '1': supervisory flash memory."]
    #[inline(always)]
    pub fn axa(&mut self) -> AXA_W<24> {
        AXA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash macro address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fm_addr](index.html) module"]
pub struct FM_ADDR_SPEC;
impl crate::RegisterSpec for FM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fm_addr::R](R) reader structure"]
impl crate::Readable for FM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fm_addr::W](W) writer structure"]
impl crate::Writable for FM_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FM_ADDR to value 0"]
impl crate::Resettable for FM_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
