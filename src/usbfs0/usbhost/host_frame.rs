#[doc = "Reader of register HOST_FRAME"]
pub type R = crate::R<u32, super::HOST_FRAME>;
#[doc = "Writer for register HOST_FRAME"]
pub type W = crate::W<u32, super::HOST_FRAME>;
#[doc = "Register HOST_FRAME `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_FRAME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAME`"]
pub struct FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W {
        FRAME_W { w: self }
    }
}
