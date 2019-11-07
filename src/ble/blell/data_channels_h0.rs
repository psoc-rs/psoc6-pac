#[doc = "Reader of register DATA_CHANNELS_H0"]
pub type R = crate::R<u32, super::DATA_CHANNELS_H0>;
#[doc = "Writer for register DATA_CHANNELS_H0"]
pub type W = crate::W<u32, super::DATA_CHANNELS_H0>;
#[doc = "Register DATA_CHANNELS_H0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_CHANNELS_H0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_CHANNELS_H0`"]
pub type DATA_CHANNELS_H0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_CHANNELS_H0`"]
pub struct DATA_CHANNELS_H0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CHANNELS_H0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
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
    pub fn data_channels_h0(&mut self) -> DATA_CHANNELS_H0_W {
        DATA_CHANNELS_H0_W { w: self }
    }
}
