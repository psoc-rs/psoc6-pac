#[doc = "Register `HOST_EOF` reader"]
pub struct R(crate::R<HOST_EOF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_EOF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_EOF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_EOF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_EOF` writer"]
pub struct W(crate::W<HOST_EOF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_EOF_SPEC>;
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
impl From<crate::W<HOST_EOF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_EOF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOF` reader - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EOF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EOF` writer - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EOF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_EOF_SPEC, u16, u16, 14, O>;
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
    pub fn eof(&mut self) -> EOF_W<0> {
        EOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host EOF Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_eof](index.html) module"]
pub struct HOST_EOF_SPEC;
impl crate::RegisterSpec for HOST_EOF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_eof::R](R) reader structure"]
impl crate::Readable for HOST_EOF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_eof::W](W) writer structure"]
impl crate::Writable for HOST_EOF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_EOF to value 0"]
impl crate::Resettable for HOST_EOF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
