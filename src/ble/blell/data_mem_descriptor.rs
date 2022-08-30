#[doc = "Register `DATA_MEM_DESCRIPTOR[%s]` reader"]
pub struct R(crate::R<DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_MEM_DESCRIPTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_MEM_DESCRIPTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_MEM_DESCRIPTOR[%s]` writer"]
pub struct W(crate::W<DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_MEM_DESCRIPTOR_SPEC>;
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
impl From<crate::W<DATA_MEM_DESCRIPTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLID` reader - N/A"]
pub type LLID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LLID` writer - N/A"]
pub type LLID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_MEM_DESCRIPTOR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATA_LENGTH` reader - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub type DATA_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_LENGTH` writer - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub type DATA_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_MEM_DESCRIPTOR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid(&self) -> LLID_R {
        LLID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length(&self) -> DATA_LENGTH_R {
        DATA_LENGTH_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid(&mut self) -> LLID_W<0> {
        LLID_W::new(self)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length(&mut self) -> DATA_LENGTH_W<2> {
        DATA_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer descriptor 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_mem_descriptor](index.html) module"]
pub struct DATA_MEM_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for DATA_MEM_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_mem_descriptor::R](R) reader structure"]
impl crate::Readable for DATA_MEM_DESCRIPTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_mem_descriptor::W](W) writer structure"]
impl crate::Writable for DATA_MEM_DESCRIPTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_MEM_DESCRIPTOR[%s]
to value 0"]
impl crate::Resettable for DATA_MEM_DESCRIPTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
