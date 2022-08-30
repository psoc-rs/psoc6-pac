#[doc = "Register `TR_CTRL1` reader"]
pub struct R(crate::R<TR_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTRL1` writer"]
pub struct W(crate::W<TR_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTRL1_SPEC>;
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
impl From<crate::W<TR_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A capture event will copy the counter value into the CC register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTURE_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<CAPTURE_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTURE_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPTURE_EDGE` reader - A capture event will copy the counter value into the CC register."]
pub type CAPTURE_EDGE_R = crate::FieldReader<u8, CAPTURE_EDGE_A>;
impl CAPTURE_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_EDGE_A {
        match self.bits {
            0 => CAPTURE_EDGE_A::RISING_EDGE,
            1 => CAPTURE_EDGE_A::FALLING_EDGE,
            2 => CAPTURE_EDGE_A::BOTH_EDGES,
            3 => CAPTURE_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAPTURE_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAPTURE_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CAPTURE_EDGE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == CAPTURE_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `CAPTURE_EDGE` writer - A capture event will copy the counter value into the CC register."]
pub type CAPTURE_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTRL1_SPEC, u8, CAPTURE_EDGE_A, 2, O>;
impl<'a, const O: u8> CAPTURE_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CAPTURE_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CAPTURE_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(CAPTURE_EDGE_A::BOTH_EDGES)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(CAPTURE_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "A counter event will increase or decrease the counter by '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COUNT_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<COUNT_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: COUNT_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COUNT_EDGE` reader - A counter event will increase or decrease the counter by '1'."]
pub type COUNT_EDGE_R = crate::FieldReader<u8, COUNT_EDGE_A>;
impl COUNT_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COUNT_EDGE_A {
        match self.bits {
            0 => COUNT_EDGE_A::RISING_EDGE,
            1 => COUNT_EDGE_A::FALLING_EDGE,
            2 => COUNT_EDGE_A::BOTH_EDGES,
            3 => COUNT_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == COUNT_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == COUNT_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == COUNT_EDGE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == COUNT_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `COUNT_EDGE` writer - A counter event will increase or decrease the counter by '1'."]
pub type COUNT_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTRL1_SPEC, u8, COUNT_EDGE_A, 2, O>;
impl<'a, const O: u8> COUNT_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::BOTH_EDGES)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(COUNT_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RELOAD_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<RELOAD_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RELOAD_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RELOAD_EDGE` reader - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type RELOAD_EDGE_R = crate::FieldReader<u8, RELOAD_EDGE_A>;
impl RELOAD_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_EDGE_A {
        match self.bits {
            0 => RELOAD_EDGE_A::RISING_EDGE,
            1 => RELOAD_EDGE_A::FALLING_EDGE,
            2 => RELOAD_EDGE_A::BOTH_EDGES,
            3 => RELOAD_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == RELOAD_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == RELOAD_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == RELOAD_EDGE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == RELOAD_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `RELOAD_EDGE` writer - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
pub type RELOAD_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTRL1_SPEC, u8, RELOAD_EDGE_A, 2, O>;
impl<'a, const O: u8> RELOAD_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::BOTH_EDGES)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(RELOAD_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOP_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<STOP_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STOP_EDGE` reader - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type STOP_EDGE_R = crate::FieldReader<u8, STOP_EDGE_A>;
impl STOP_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_EDGE_A {
        match self.bits {
            0 => STOP_EDGE_A::RISING_EDGE,
            1 => STOP_EDGE_A::FALLING_EDGE,
            2 => STOP_EDGE_A::BOTH_EDGES,
            3 => STOP_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == STOP_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == STOP_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == STOP_EDGE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == STOP_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `STOP_EDGE` writer - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
pub type STOP_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTRL1_SPEC, u8, STOP_EDGE_A, 2, O>;
impl<'a, const O: u8> STOP_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::BOTH_EDGES)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(STOP_EDGE_A::NO_EDGE_DET)
    }
}
#[doc = "A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum START_EDGE_A {
    #[doc = "0: Rising edge. Any rising edge generates an event."]
    RISING_EDGE = 0,
    #[doc = "1: Falling edge. Any falling edge generates an event."]
    FALLING_EDGE = 1,
    #[doc = "2: Rising AND falling edge. Any odd amount of edges generates an event."]
    BOTH_EDGES = 2,
    #[doc = "3: No edge detection, use trigger as is."]
    NO_EDGE_DET = 3,
}
impl From<START_EDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: START_EDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `START_EDGE` reader - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type START_EDGE_R = crate::FieldReader<u8, START_EDGE_A>;
impl START_EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_EDGE_A {
        match self.bits {
            0 => START_EDGE_A::RISING_EDGE,
            1 => START_EDGE_A::FALLING_EDGE,
            2 => START_EDGE_A::BOTH_EDGES,
            3 => START_EDGE_A::NO_EDGE_DET,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == START_EDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == START_EDGE_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == START_EDGE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `NO_EDGE_DET`"]
    #[inline(always)]
    pub fn is_no_edge_det(&self) -> bool {
        *self == START_EDGE_A::NO_EDGE_DET
    }
}
#[doc = "Field `START_EDGE` writer - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
pub type START_EDGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TR_CTRL1_SPEC, u8, START_EDGE_A, 2, O>;
impl<'a, const O: u8> START_EDGE_W<'a, O> {
    #[doc = "Rising edge. Any rising edge generates an event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(START_EDGE_A::RISING_EDGE)
    }
    #[doc = "Falling edge. Any falling edge generates an event."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(START_EDGE_A::FALLING_EDGE)
    }
    #[doc = "Rising AND falling edge. Any odd amount of edges generates an event."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(START_EDGE_A::BOTH_EDGES)
    }
    #[doc = "No edge detection, use trigger as is."]
    #[inline(always)]
    pub fn no_edge_det(self) -> &'a mut W {
        self.variant(START_EDGE_A::NO_EDGE_DET)
    }
}
impl R {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn capture_edge(&self) -> CAPTURE_EDGE_R {
        CAPTURE_EDGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&self) -> COUNT_EDGE_R {
        COUNT_EDGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&self) -> RELOAD_EDGE_R {
        RELOAD_EDGE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&self) -> STOP_EDGE_R {
        STOP_EDGE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&self) -> START_EDGE_R {
        START_EDGE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn capture_edge(&mut self) -> CAPTURE_EDGE_W<0> {
        CAPTURE_EDGE_W::new(self)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&mut self) -> COUNT_EDGE_W<2> {
        COUNT_EDGE_W::new(self)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&mut self) -> RELOAD_EDGE_W<4> {
        RELOAD_EDGE_W::new(self)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&mut self) -> STOP_EDGE_W<6> {
        STOP_EDGE_W::new(self)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&mut self) -> START_EDGE_W<8> {
        START_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter trigger control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctrl1](index.html) module"]
pub struct TR_CTRL1_SPEC;
impl crate::RegisterSpec for TR_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctrl1::R](R) reader structure"]
impl crate::Readable for TR_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctrl1::W](W) writer structure"]
impl crate::Writable for TR_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTRL1 to value 0x03ff"]
impl crate::Resettable for TR_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
