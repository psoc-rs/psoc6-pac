#[doc = "Register `CONN_PARAM1` reader"]
pub struct R(crate::R<CONN_PARAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_PARAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_PARAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_PARAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_PARAM1` writer"]
pub struct W(crate::W<CONN_PARAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_PARAM1_SPEC>;
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
impl From<crate::W<CONN_PARAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_PARAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCA_PARAM` reader - Sleep Clock Accuracy"]
pub type SCA_PARAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCA_PARAM` writer - Sleep Clock Accuracy"]
pub type SCA_PARAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONN_PARAM1_SPEC, u8, u8, 3, O>;
#[doc = "Field `HOP_INCREMENT_PARAM` reader - Hop increment for connection channel."]
pub type HOP_INCREMENT_PARAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOP_INCREMENT_PARAM` writer - Hop increment for connection channel."]
pub type HOP_INCREMENT_PARAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_PARAM1_SPEC, u8, u8, 5, O>;
#[doc = "Field `CRC_INIT_L` reader - This field defines the lower byte (7:0) of the CRC initialization vector."]
pub type CRC_INIT_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC_INIT_L` writer - This field defines the lower byte (7:0) of the CRC initialization vector."]
pub type CRC_INIT_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_PARAM1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Sleep Clock Accuracy"]
    #[inline(always)]
    pub fn sca_param(&self) -> SCA_PARAM_R {
        SCA_PARAM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - Hop increment for connection channel."]
    #[inline(always)]
    pub fn hop_increment_param(&self) -> HOP_INCREMENT_PARAM_R {
        HOP_INCREMENT_PARAM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte (7:0) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_l(&self) -> CRC_INIT_L_R {
        CRC_INIT_L_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Clock Accuracy"]
    #[inline(always)]
    pub fn sca_param(&mut self) -> SCA_PARAM_W<0> {
        SCA_PARAM_W::new(self)
    }
    #[doc = "Bits 3:7 - Hop increment for connection channel."]
    #[inline(always)]
    pub fn hop_increment_param(&mut self) -> HOP_INCREMENT_PARAM_W<3> {
        HOP_INCREMENT_PARAM_W::new(self)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte (7:0) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_l(&mut self) -> CRC_INIT_L_W<8> {
        CRC_INIT_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection parameter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_param1](index.html) module"]
pub struct CONN_PARAM1_SPEC;
impl crate::RegisterSpec for CONN_PARAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_param1::R](R) reader structure"]
impl crate::Readable for CONN_PARAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_param1::W](W) writer structure"]
impl crate::Writable for CONN_PARAM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_PARAM1 to value 0"]
impl crate::Resettable for CONN_PARAM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
