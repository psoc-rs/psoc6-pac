#[doc = "Reader of register RECEIVE_TRIG_CTRL"]
pub type R = crate::R<u32, super::RECEIVE_TRIG_CTRL>;
#[doc = "Writer for register RECEIVE_TRIG_CTRL"]
pub type W = crate::W<u32, super::RECEIVE_TRIG_CTRL>;
#[doc = "Register RECEIVE_TRIG_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RECEIVE_TRIG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACC_TRIGGER_THRESHOLD`"]
pub type ACC_TRIGGER_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACC_TRIGGER_THRESHOLD`"]
pub struct ACC_TRIGGER_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_TRIGGER_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ACC_TRIGGER_TIMEOUT`"]
pub type ACC_TRIGGER_TIMEOUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACC_TRIGGER_TIMEOUT`"]
pub struct ACC_TRIGGER_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_TRIGGER_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
    #[inline(always)]
    pub fn acc_trigger_threshold(&self) -> ACC_TRIGGER_THRESHOLD_R {
        ACC_TRIGGER_THRESHOLD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
    #[inline(always)]
    pub fn acc_trigger_timeout(&self) -> ACC_TRIGGER_TIMEOUT_R {
        ACC_TRIGGER_TIMEOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Access address match threshold value. Number of bits of ac-cess address that should match with the expected access ad-dress to trigger an access code match. Max value : 32 (for 32-bit access address) Lower values may be programmed for bad radios or channels but care must be taken to ensure there are no 'false' matches due to reduced number of bits required to match."]
    #[inline(always)]
    pub fn acc_trigger_threshold(&mut self) -> ACC_TRIGGER_THRESHOLD_W {
        ACC_TRIGGER_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - If access address match does not occur then within this time from the start of receive operation, the receive operation times out and stops. An internal counter value of 1usec resolution is continuously compared with the value programmed. Max value :0xFF"]
    #[inline(always)]
    pub fn acc_trigger_timeout(&mut self) -> ACC_TRIGGER_TIMEOUT_W {
        ACC_TRIGGER_TIMEOUT_W { w: self }
    }
}
