#[doc = "Register `CM0_VECTOR_TABLE_BASE` reader"]
pub struct R(crate::R<CM0_VECTOR_TABLE_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM0_VECTOR_TABLE_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM0_VECTOR_TABLE_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM0_VECTOR_TABLE_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM0_VECTOR_TABLE_BASE` writer"]
pub struct W(crate::W<CM0_VECTOR_TABLE_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM0_VECTOR_TABLE_BASE_SPEC>;
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
impl From<crate::W<CM0_VECTOR_TABLE_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM0_VECTOR_TABLE_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR24` reader - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type ADDR24_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR24` writer - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type ADDR24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CM0_VECTOR_TABLE_BASE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&self) -> ADDR24_R {
        ADDR24_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&mut self) -> ADDR24_W<8> {
        ADDR24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM0+ vector table base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm0_vector_table_base](index.html) module"]
pub struct CM0_VECTOR_TABLE_BASE_SPEC;
impl crate::RegisterSpec for CM0_VECTOR_TABLE_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm0_vector_table_base::R](R) reader structure"]
impl crate::Readable for CM0_VECTOR_TABLE_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm0_vector_table_base::W](W) writer structure"]
impl crate::Writable for CM0_VECTOR_TABLE_BASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM0_VECTOR_TABLE_BASE to value 0"]
impl crate::Resettable for CM0_VECTOR_TABLE_BASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
