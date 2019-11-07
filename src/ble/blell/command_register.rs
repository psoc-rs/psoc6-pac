#[doc = "Writer for register COMMAND_REGISTER"]
pub type W = crate::W<u32, super::COMMAND_REGISTER>;
#[doc = "Register COMMAND_REGISTER `reset()`'s with value 0"]
impl crate::ResetValue for super::COMMAND_REGISTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `COMMAND`"]
pub struct COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn command(&mut self) -> COMMAND_W {
        COMMAND_W { w: self }
    }
}
