#[doc = "Register `CONN_REQ_WORD2` reader"]
pub struct R(crate::R<CONN_REQ_WORD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_REQ_WORD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_REQ_WORD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_REQ_WORD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_REQ_WORD2` writer"]
pub struct W(crate::W<CONN_REQ_WORD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_REQ_WORD2_SPEC>;
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
impl From<crate::W<CONN_REQ_WORD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_REQ_WORD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WINDOW_SIZE_VAL` reader - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub type TX_WINDOW_SIZE_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_WINDOW_SIZE_VAL` writer - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub type TX_WINDOW_SIZE_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD2_SPEC, u8, u8, 8, O>;
#[doc = "Field `CRC_INIT_LOWER` reader - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
pub type CRC_INIT_LOWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_INIT_LOWER` writer - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
pub type CRC_INIT_LOWER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_REQ_WORD2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&self) -> TX_WINDOW_SIZE_VAL_R {
        TX_WINDOW_SIZE_VAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&self) -> CRC_INIT_LOWER_R {
        CRC_INIT_LOWER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size_val(&mut self) -> TX_WINDOW_SIZE_VAL_W<0> {
        TX_WINDOW_SIZE_VAL_W::new(self)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte \\[7:0\\]
of the CRC initialization value."]
    #[inline(always)]
    pub fn crc_init_lower(&mut self) -> CRC_INIT_LOWER_W<8> {
        CRC_INIT_LOWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection request address word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_req_word2](index.html) module"]
pub struct CONN_REQ_WORD2_SPEC;
impl crate::RegisterSpec for CONN_REQ_WORD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_req_word2::R](R) reader structure"]
impl crate::Readable for CONN_REQ_WORD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_req_word2::W](W) writer structure"]
impl crate::Writable for CONN_REQ_WORD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_REQ_WORD2 to value 0"]
impl crate::Resettable for CONN_REQ_WORD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
