#[doc = "Register `MMMS_DATA_MEM_DESCRIPTOR[%s]` reader"]
pub struct R(crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_DATA_MEM_DESCRIPTOR[%s]` writer"]
pub struct W(crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>;
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
impl From<crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLID_C1` reader - N/A"]
pub type LLID_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LLID_C1` writer - N/A"]
pub type LLID_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMMS_DATA_MEM_DESCRIPTOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATA_LENGTH_C1` reader - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub type DATA_LENGTH_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_LENGTH_C1` writer - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub type DATA_LENGTH_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMMS_DATA_MEM_DESCRIPTOR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&self) -> LLID_C1_R {
        LLID_C1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&self) -> DATA_LENGTH_C1_R {
        DATA_LENGTH_C1_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&mut self) -> LLID_C1_W<0> {
        LLID_C1_W::new(self)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&mut self) -> DATA_LENGTH_C1_W<2> {
        DATA_LENGTH_C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer descriptor 0 to 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_data_mem_descriptor](index.html) module"]
pub struct MMMS_DATA_MEM_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_data_mem_descriptor::R](R) reader structure"]
impl crate::Readable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_data_mem_descriptor::W](W) writer structure"]
impl crate::Writable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_DATA_MEM_DESCRIPTOR[%s]
to value 0"]
impl crate::Resettable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
