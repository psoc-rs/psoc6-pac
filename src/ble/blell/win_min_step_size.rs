#[doc = "Reader of register WIN_MIN_STEP_SIZE"]
pub type R = crate::R<u32, super::WIN_MIN_STEP_SIZE>;
#[doc = "Writer for register WIN_MIN_STEP_SIZE"]
pub type W = crate::W<u32, super::WIN_MIN_STEP_SIZE>;
#[doc = "Register WIN_MIN_STEP_SIZE `reset()`'s with value 0x2064"]
impl crate::ResetValue for super::WIN_MIN_STEP_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2064
    }
}
#[doc = "Reader of field `STEPDN`"]
pub type STEPDN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STEPDN`"]
pub struct STEPDN_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `STEPUP`"]
pub type STEPUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STEPUP`"]
pub struct STEPUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `WINDOW_MIN_FW`"]
pub type WINDOW_MIN_FW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WINDOW_MIN_FW`"]
pub struct WINDOW_MIN_FW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_MIN_FW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepdn(&self) -> STEPDN_R {
        STEPDN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepup(&self) -> STEPUP_R {
        STEPUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
    #[inline(always)]
    pub fn window_min_fw(&self) -> WINDOW_MIN_FW_R {
        WINDOW_MIN_FW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - After receiving 2 consecutive good packets the reference window is gradually decremented by step down size until it reaches window minimum. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepdn(&mut self) -> STEPDN_W {
        STEPDN_W { w: self }
    }
    #[doc = "Bits 4:7 - If packets are missed, the reference window is gradually increased by step up size, until it receives 2 consecutive good packets. The unit is in microseconds"]
    #[inline(always)]
    pub fn stepup(&mut self) -> STEPUP_W {
        STEPUP_W { w: self }
    }
    #[doc = "Bits 8:15 - Minimum window interval value programmed by firmware. While the slave receive window is decremented, the windows_min_fw sets the lowest value of the window widen value to ensure packets are not missed. The unit is in microseconds."]
    #[inline(always)]
    pub fn window_min_fw(&mut self) -> WINDOW_MIN_FW_W {
        WINDOW_MIN_FW_W { w: self }
    }
}
