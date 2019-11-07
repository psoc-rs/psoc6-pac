#[doc = "Reader of register LE_RF_TEST_MODE_EXT"]
pub type R = crate::R<u32, super::LE_RF_TEST_MODE_EXT>;
#[doc = "Writer for register LE_RF_TEST_MODE_EXT"]
pub type W = crate::W<u32, super::LE_RF_TEST_MODE_EXT>;
#[doc = "Register LE_RF_TEST_MODE_EXT `reset()`'s with value 0"]
impl crate::ResetValue for super::LE_RF_TEST_MODE_EXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTM_PACKET_LENGTH`"]
pub type DTM_PACKET_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTM_PACKET_LENGTH`"]
pub struct DTM_PACKET_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_PACKET_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DTM TX packet length. Bits \\[7:6\\] are accessible onle when DLE is enabled"]
    #[inline(always)]
    pub fn dtm_packet_length(&self) -> DTM_PACKET_LENGTH_R {
        DTM_PACKET_LENGTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTM TX packet length. Bits \\[7:6\\] are accessible onle when DLE is enabled"]
    #[inline(always)]
    pub fn dtm_packet_length(&mut self) -> DTM_PACKET_LENGTH_W {
        DTM_PACKET_LENGTH_W { w: self }
    }
}
