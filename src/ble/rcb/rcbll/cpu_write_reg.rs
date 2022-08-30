#[doc = "Register `CPU_WRITE_REG` reader"]
pub struct R(crate::R<CPU_WRITE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_WRITE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_WRITE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_WRITE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_WRITE_REG` writer"]
pub struct W(crate::W<CPU_WRITE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_WRITE_REG_SPEC>;
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
impl From<crate::W<CPU_WRITE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_WRITE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - N/A"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - N/A"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_WRITE_REG_SPEC, u16, u16, 16, O>;
#[doc = "Field `WRITE_DATA` reader - N/A"]
pub type WRITE_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WRITE_DATA` writer - N/A"]
pub type WRITE_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_WRITE_REG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn write_data(&self) -> WRITE_DATA_R {
        WRITE_DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - N/A"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:31 - N/A"]
    #[inline(always)]
    pub fn write_data(&mut self) -> WRITE_DATA_W<16> {
        WRITE_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "N/A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_write_reg](index.html) module"]
pub struct CPU_WRITE_REG_SPEC;
impl crate::RegisterSpec for CPU_WRITE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_write_reg::R](R) reader structure"]
impl crate::Readable for CPU_WRITE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_write_reg::W](W) writer structure"]
impl crate::Writable for CPU_WRITE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_WRITE_REG to value 0"]
impl crate::Resettable for CPU_WRITE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
