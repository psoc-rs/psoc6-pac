#[doc = "Register `EFUSE_WDATA_H` reader"]
pub struct R(crate::R<EFUSE_WDATA_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_WDATA_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_WDATA_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_WDATA_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE_WDATA_H` writer"]
pub struct W(crate::W<EFUSE_WDATA_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_WDATA_H_SPEC>;
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
impl From<crate::W<EFUSE_WDATA_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_WDATA_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - This register has the write value to the Efuse macro, fuse bits\\[63:32\\]"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - This register has the write value to the Efuse macro, fuse bits\\[63:32\\]"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EFUSE_WDATA_H_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register has the write value to the Efuse macro, fuse bits\\[63:32\\]"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register has the write value to the Efuse macro, fuse bits\\[63:32\\]"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE higher write word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse_wdata_h](index.html) module"]
pub struct EFUSE_WDATA_H_SPEC;
impl crate::RegisterSpec for EFUSE_WDATA_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse_wdata_h::R](R) reader structure"]
impl crate::Readable for EFUSE_WDATA_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse_wdata_h::W](W) writer structure"]
impl crate::Writable for EFUSE_WDATA_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE_WDATA_H to value 0"]
impl crate::Resettable for EFUSE_WDATA_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
