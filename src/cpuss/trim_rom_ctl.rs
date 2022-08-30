#[doc = "Register `TRIM_ROM_CTL` reader"]
pub struct R(crate::R<TRIM_ROM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_ROM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_ROM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_ROM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_ROM_CTL` writer"]
pub struct W(crate::W<TRIM_ROM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_ROM_CTL_SPEC>;
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
impl From<crate::W<TRIM_ROM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_ROM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RM` reader - N/A"]
pub type RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RM` writer - N/A"]
pub type RM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_ROM_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RME` reader - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
pub type RME_R = crate::BitReader<bool>;
#[doc = "Field `RME` writer - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
pub type RME_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRIM_ROM_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
    #[inline(always)]
    pub fn rme(&self) -> RME_R {
        RME_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W<0> {
        RM_W::new(self)
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external pin Read-Write margin setting."]
    #[inline(always)]
    pub fn rme(&mut self) -> RME_W<4> {
        RME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_rom_ctl](index.html) module"]
pub struct TRIM_ROM_CTL_SPEC;
impl crate::RegisterSpec for TRIM_ROM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_rom_ctl::R](R) reader structure"]
impl crate::Readable for TRIM_ROM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_rom_ctl::W](W) writer structure"]
impl crate::Writable for TRIM_ROM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_ROM_CTL to value 0x02"]
impl crate::Resettable for TRIM_ROM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
