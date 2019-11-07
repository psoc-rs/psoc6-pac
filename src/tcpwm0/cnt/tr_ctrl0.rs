#[doc = "Reader of register TR_CTRL0"]
pub type R = crate::R<u32, super::TR_CTRL0>;
#[doc = "Writer for register TR_CTRL0"]
pub type W = crate::W<u32, super::TR_CTRL0>;
#[doc = "Register TR_CTRL0 `reset()`'s with value 0x10"]
impl crate::ResetValue for super::TR_CTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `CAPTURE_SEL`"]
pub type CAPTURE_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPTURE_SEL`"]
pub struct CAPTURE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `COUNT_SEL`"]
pub type COUNT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT_SEL`"]
pub struct COUNT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RELOAD_SEL`"]
pub type RELOAD_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RELOAD_SEL`"]
pub struct RELOAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `STOP_SEL`"]
pub type STOP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STOP_SEL`"]
pub struct STOP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `START_SEL`"]
pub type START_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `START_SEL`"]
pub struct START_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture_sel(&self) -> CAPTURE_SEL_R {
        CAPTURE_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&self) -> COUNT_SEL_R {
        COUNT_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&self) -> STOP_SEL_R {
        STOP_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&self) -> START_SEL_R {
        START_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture_sel(&mut self) -> CAPTURE_SEL_W {
        CAPTURE_SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&mut self) -> COUNT_SEL_W {
        COUNT_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W {
        RELOAD_SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&mut self) -> STOP_SEL_W {
        STOP_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&mut self) -> START_SEL_W {
        START_SEL_W { w: self }
    }
}
