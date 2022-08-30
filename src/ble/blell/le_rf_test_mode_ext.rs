#[doc = "Register `LE_RF_TEST_MODE_EXT` reader"]
pub struct R(crate::R<LE_RF_TEST_MODE_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_RF_TEST_MODE_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_RF_TEST_MODE_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_RF_TEST_MODE_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LE_RF_TEST_MODE_EXT` writer"]
pub struct W(crate::W<LE_RF_TEST_MODE_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LE_RF_TEST_MODE_EXT_SPEC>;
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
impl From<crate::W<LE_RF_TEST_MODE_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LE_RF_TEST_MODE_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTM_PACKET_LENGTH` reader - DTM TX packet length. Bits \\[7:6\\]
are accessible onle when DLE is enabled"]
pub type DTM_PACKET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTM_PACKET_LENGTH` writer - DTM TX packet length. Bits \\[7:6\\]
are accessible onle when DLE is enabled"]
pub type DTM_PACKET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LE_RF_TEST_MODE_EXT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DTM TX packet length. Bits \\[7:6\\]
are accessible onle when DLE is enabled"]
    #[inline(always)]
    pub fn dtm_packet_length(&self) -> DTM_PACKET_LENGTH_R {
        DTM_PACKET_LENGTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTM TX packet length. Bits \\[7:6\\]
are accessible onle when DLE is enabled"]
    #[inline(always)]
    pub fn dtm_packet_length(&mut self) -> DTM_PACKET_LENGTH_W<0> {
        DTM_PACKET_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct Test Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_rf_test_mode_ext](index.html) module"]
pub struct LE_RF_TEST_MODE_EXT_SPEC;
impl crate::RegisterSpec for LE_RF_TEST_MODE_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le_rf_test_mode_ext::R](R) reader structure"]
impl crate::Readable for LE_RF_TEST_MODE_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [le_rf_test_mode_ext::W](W) writer structure"]
impl crate::Writable for LE_RF_TEST_MODE_EXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LE_RF_TEST_MODE_EXT to value 0"]
impl crate::Resettable for LE_RF_TEST_MODE_EXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
