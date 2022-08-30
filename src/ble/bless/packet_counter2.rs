#[doc = "Register `PACKET_COUNTER2` reader"]
pub struct R(crate::R<PACKET_COUNTER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKET_COUNTER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKET_COUNTER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKET_COUNTER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACKET_COUNTER2` writer"]
pub struct W(crate::W<PACKET_COUNTER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACKET_COUNTER2_SPEC>;
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
impl From<crate::W<PACKET_COUNTER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACKET_COUNTER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PACKET_COUNTER_UPPER` reader - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
pub type PACKET_COUNTER_UPPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PACKET_COUNTER_UPPER` writer - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
pub type PACKET_COUNTER_UPPER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PACKET_COUNTER2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_upper(&self) -> PACKET_COUNTER_UPPER_R {
        PACKET_COUNTER_UPPER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper 8 bits of the packet counter value passed as part of Nonce for the packet to be encrypted."]
    #[inline(always)]
    pub fn packet_counter_upper(&mut self) -> PACKET_COUNTER_UPPER_W<0> {
        PACKET_COUNTER_UPPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet counter 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packet_counter2](index.html) module"]
pub struct PACKET_COUNTER2_SPEC;
impl crate::RegisterSpec for PACKET_COUNTER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packet_counter2::R](R) reader structure"]
impl crate::Readable for PACKET_COUNTER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [packet_counter2::W](W) writer structure"]
impl crate::Writable for PACKET_COUNTER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PACKET_COUNTER2 to value 0"]
impl crate::Resettable for PACKET_COUNTER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
