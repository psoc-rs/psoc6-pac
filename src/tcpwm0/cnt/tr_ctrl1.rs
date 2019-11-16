#[doc = "Reader of register TR_CTRL1"]
pub type R = crate::R<u32, super::TR_CTRL1>;
#[doc = "Writer for register TR_CTRL1"]
pub type W = crate::W<u32, super::TR_CTRL1>;
#[doc = "Register TR_CTRL1 `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::TR_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
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
#[doc = "Reader of field `CAPTURE_EDGE`"]
pub type CAPTURE_EDGE_R = crate::R<u8, CAPTURE_EDGE_A>;
impl CAPTURE_EDGE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CAPTURE_EDGE`"]
pub struct CAPTURE_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE_EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
#[doc = "Reader of field `COUNT_EDGE`"]
pub type COUNT_EDGE_R = crate::R<u8, COUNT_EDGE_A>;
impl COUNT_EDGE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `COUNT_EDGE`"]
pub struct COUNT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COUNT_EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
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
#[doc = "Reader of field `RELOAD_EDGE`"]
pub type RELOAD_EDGE_R = crate::R<u8, RELOAD_EDGE_A>;
impl RELOAD_EDGE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `RELOAD_EDGE`"]
pub struct RELOAD_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
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
#[doc = "Reader of field `STOP_EDGE`"]
pub type STOP_EDGE_R = crate::R<u8, STOP_EDGE_A>;
impl STOP_EDGE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `STOP_EDGE`"]
pub struct STOP_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
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
#[doc = "Reader of field `START_EDGE`"]
pub type START_EDGE_R = crate::R<u8, START_EDGE_A>;
impl START_EDGE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `START_EDGE`"]
pub struct START_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_EDGE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn capture_edge(&self) -> CAPTURE_EDGE_R {
        CAPTURE_EDGE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&self) -> COUNT_EDGE_R {
        COUNT_EDGE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&self) -> RELOAD_EDGE_R {
        RELOAD_EDGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&self) -> STOP_EDGE_R {
        STOP_EDGE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&self) -> START_EDGE_R {
        START_EDGE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A capture event will copy the counter value into the CC register."]
    #[inline(always)]
    pub fn capture_edge(&mut self) -> CAPTURE_EDGE_W {
        CAPTURE_EDGE_W { w: self }
    }
    #[doc = "Bits 2:3 - A counter event will increase or decrease the counter by '1'."]
    #[inline(always)]
    pub fn count_edge(&mut self) -> COUNT_EDGE_W {
        COUNT_EDGE_W { w: self }
    }
    #[doc = "Bits 4:5 - A reload event will initialize the counter. When counting up, the counter is initialized to '0'. When counting down, the counter is initialized with PERIOD."]
    #[inline(always)]
    pub fn reload_edge(&mut self) -> RELOAD_EDGE_W {
        RELOAD_EDGE_W { w: self }
    }
    #[doc = "Bits 6:7 - A stop event, will stop the counter; i.e. it will no longer be running. Stopping will NOT disable the counter."]
    #[inline(always)]
    pub fn stop_edge(&mut self) -> STOP_EDGE_W {
        STOP_EDGE_W { w: self }
    }
    #[doc = "Bits 8:9 - A start event will start the counter; i.e. the counter will become running. Starting does NOT enable the counter. A start event will not initialize the counter whereas the reload event does."]
    #[inline(always)]
    pub fn start_edge(&mut self) -> START_EDGE_W {
        START_EDGE_W { w: self }
    }
}
