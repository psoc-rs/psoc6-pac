#[doc = "Reader of register HOST_EOF"]
pub type R = crate::R<u32, super::HOST_EOF>;
#[doc = "Writer for register HOST_EOF"]
pub type W = crate::W<u32, super::HOST_EOF>;
#[doc = "Register HOST_EOF `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_EOF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOF`"]
pub type EOF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EOF`"]
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn eof(&self) -> EOF_R {
        EOF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
}
