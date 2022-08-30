#[doc = "Register `DATA_CHANNELS_H0` reader"]
pub struct R(crate::R<DATA_CHANNELS_H0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CHANNELS_H0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CHANNELS_H0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CHANNELS_H0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_CHANNELS_H0` writer"]
pub struct W(crate::W<DATA_CHANNELS_H0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_CHANNELS_H0_SPEC>;
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
impl From<crate::W<DATA_CHANNELS_H0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_CHANNELS_H0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CHANNELS_H0` reader - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused. Note: The Data channel map 0 and data channel map 1 are two sets of channel maps stored, common for all the connections. At any given time, only two maps can be maintained and the connections will use one of the two sets as indicated by the channel map index field in the CE_CNFG_STS registers specific to the link. Firmware must also manage to update this field along with the map."]
pub type DATA_CHANNELS_H0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_CHANNELS_H0` writer - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused. Note: The Data channel map 0 and data channel map 1 are two sets of channel maps stored, common for all the connections. At any given time, only two maps can be maintained and the connections will use one of the two sets as indicated by the channel map index field in the CE_CNFG_STS registers specific to the link. Firmware must also manage to update this field along with the map."]
pub type DATA_CHANNELS_H0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_CHANNELS_H0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused. Note: The Data channel map 0 and data channel map 1 are two sets of channel maps stored, common for all the connections. At any given time, only two maps can be maintained and the connections will use one of the two sets as indicated by the channel map index field in the CE_CNFG_STS registers specific to the link. Firmware must also manage to update this field along with the map."]
    #[inline(always)]
    pub fn data_channels_h0(&self) -> DATA_CHANNELS_H0_R {
        DATA_CHANNELS_H0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register field indicates which of the data channels are in use. This stores the information for the upper 5 (36:32) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused. Note: The Data channel map 0 and data channel map 1 are two sets of channel maps stored, common for all the connections. At any given time, only two maps can be maintained and the connections will use one of the two sets as indicated by the channel map index field in the CE_CNFG_STS registers specific to the link. Firmware must also manage to update this field along with the map."]
    #[inline(always)]
    pub fn data_channels_h0(&mut self) -> DATA_CHANNELS_H0_W<0> {
        DATA_CHANNELS_H0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data channel map 0 (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_h0](index.html) module"]
pub struct DATA_CHANNELS_H0_SPEC;
impl crate::RegisterSpec for DATA_CHANNELS_H0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_channels_h0::R](R) reader structure"]
impl crate::Readable for DATA_CHANNELS_H0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_channels_h0::W](W) writer structure"]
impl crate::Writable for DATA_CHANNELS_H0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_CHANNELS_H0 to value 0"]
impl crate::Resettable for DATA_CHANNELS_H0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
