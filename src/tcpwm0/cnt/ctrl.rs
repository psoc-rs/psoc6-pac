#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTO_RELOAD_CC`"]
pub type AUTO_RELOAD_CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_CC`"]
pub struct AUTO_RELOAD_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_CC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `AUTO_RELOAD_PERIOD`"]
pub type AUTO_RELOAD_PERIOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_RELOAD_PERIOD`"]
pub struct AUTO_RELOAD_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_RELOAD_PERIOD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PWM_SYNC_KILL`"]
pub type PWM_SYNC_KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_SYNC_KILL`"]
pub struct PWM_SYNC_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_SYNC_KILL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PWM_STOP_ON_KILL`"]
pub type PWM_STOP_ON_KILL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM_STOP_ON_KILL`"]
pub struct PWM_STOP_ON_KILL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_STOP_ON_KILL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GENERIC_A {
    #[doc = "0: Divide by 1 (other-than-PWM_DT mode)"]
    DIVBY1,
    #[doc = "1: Divide by 2 (other-than-PWM_DT mode)"]
    DIVBY2,
    #[doc = "2: Divide by 4 (other-than-PWM_DT mode)"]
    DIVBY4,
    #[doc = "3: Divide by 8 (other-than-PWM_DT mode)"]
    DIVBY8,
    #[doc = "4: Divide by 16 (other-than-PWM_DT mode)"]
    DIVBY16,
    #[doc = "5: Divide by 32 (other-than-PWM_DT mode)"]
    DIVBY32,
    #[doc = "6: Divide by 64 (other-than-PWM_DT mode)"]
    DIVBY64,
    #[doc = "7: Divide by 128 (other-than-PWM_DT mode)"]
    DIVBY128,
}
impl From<GENERIC_A> for u8 {
    #[inline(always)]
    fn from(variant: GENERIC_A) -> Self {
        match variant {
            GENERIC_A::DIVBY1 => 0,
            GENERIC_A::DIVBY2 => 1,
            GENERIC_A::DIVBY4 => 2,
            GENERIC_A::DIVBY8 => 3,
            GENERIC_A::DIVBY16 => 4,
            GENERIC_A::DIVBY32 => 5,
            GENERIC_A::DIVBY64 => 6,
            GENERIC_A::DIVBY128 => 7,
        }
    }
}
#[doc = "Reader of field `GENERIC`"]
pub type GENERIC_R = crate::R<u8, GENERIC_A>;
impl GENERIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GENERIC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GENERIC_A::DIVBY1),
            1 => Val(GENERIC_A::DIVBY2),
            2 => Val(GENERIC_A::DIVBY4),
            3 => Val(GENERIC_A::DIVBY8),
            4 => Val(GENERIC_A::DIVBY16),
            5 => Val(GENERIC_A::DIVBY32),
            6 => Val(GENERIC_A::DIVBY64),
            7 => Val(GENERIC_A::DIVBY128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY1`"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == GENERIC_A::DIVBY1
    }
    #[doc = "Checks if the value of the field is `DIVBY2`"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == GENERIC_A::DIVBY2
    }
    #[doc = "Checks if the value of the field is `DIVBY4`"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == GENERIC_A::DIVBY4
    }
    #[doc = "Checks if the value of the field is `DIVBY8`"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == GENERIC_A::DIVBY8
    }
    #[doc = "Checks if the value of the field is `DIVBY16`"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == GENERIC_A::DIVBY16
    }
    #[doc = "Checks if the value of the field is `DIVBY32`"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == GENERIC_A::DIVBY32
    }
    #[doc = "Checks if the value of the field is `DIVBY64`"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == GENERIC_A::DIVBY64
    }
    #[doc = "Checks if the value of the field is `DIVBY128`"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == GENERIC_A::DIVBY128
    }
}
#[doc = "Write proxy for field `GENERIC`"]
pub struct GENERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> GENERIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GENERIC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY1)
    }
    #[doc = "Divide by 2 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY2)
    }
    #[doc = "Divide by 4 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY4)
    }
    #[doc = "Divide by 8 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY8)
    }
    #[doc = "Divide by 16 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY16)
    }
    #[doc = "Divide by 32 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY32)
    }
    #[doc = "Divide by 64 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY64)
    }
    #[doc = "Divide by 128 (other-than-PWM_DT mode)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut W {
        self.variant(GENERIC_A::DIVBY128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Determines counter direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_DOWN_MODE_A {
    #[doc = "0: Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    COUNT_UP,
    #[doc = "1: Count down (to '0'). An underflow event is generated when  the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_DOWN,
    #[doc = "2: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    COUNT_UPDN1,
    #[doc = "3: Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    COUNT_UPDN2,
}
impl From<UP_DOWN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UP_DOWN_MODE_A) -> Self {
        match variant {
            UP_DOWN_MODE_A::COUNT_UP => 0,
            UP_DOWN_MODE_A::COUNT_DOWN => 1,
            UP_DOWN_MODE_A::COUNT_UPDN1 => 2,
            UP_DOWN_MODE_A::COUNT_UPDN2 => 3,
        }
    }
}
#[doc = "Reader of field `UP_DOWN_MODE`"]
pub type UP_DOWN_MODE_R = crate::R<u8, UP_DOWN_MODE_A>;
impl UP_DOWN_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_DOWN_MODE_A {
        match self.bits {
            0 => UP_DOWN_MODE_A::COUNT_UP,
            1 => UP_DOWN_MODE_A::COUNT_DOWN,
            2 => UP_DOWN_MODE_A::COUNT_UPDN1,
            3 => UP_DOWN_MODE_A::COUNT_UPDN2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_UP`"]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UP
    }
    #[doc = "Checks if the value of the field is `COUNT_DOWN`"]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_DOWN
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN1`"]
    #[inline(always)]
    pub fn is_count_updn1(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN1
    }
    #[doc = "Checks if the value of the field is `COUNT_UPDN2`"]
    #[inline(always)]
    pub fn is_count_updn2(&self) -> bool {
        *self == UP_DOWN_MODE_A::COUNT_UPDN2
    }
}
#[doc = "Write proxy for field `UP_DOWN_MODE`"]
pub struct UP_DOWN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_DOWN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UP_DOWN_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Count up (to PERIOD). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. A terminal count event is generated when the counter changes from a state in which COUNTER equals PERIOD."]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UP)
    }
    #[doc = "Count down (to '0'). An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_DOWN)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0'."]
    #[inline(always)]
    pub fn count_updn1(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN1)
    }
    #[doc = "Count up (to PERIOD), then count down (to '0'). An overflow event is generated when the counter changes from a state in which COUNTER equals PERIOD. An underflow event is generated when the counter changes from a state in which COUNTER equals '0'. A terminal count event is generated when the counter changes from a state in which COUNTER equals '0' AND when the counter changes from a state in which COUNTER equals PERIOD (this counter direction can be used for PWM functionality with asymmetrical updates)."]
    #[inline(always)]
    pub fn count_updn2(self) -> &'a mut W {
        self.variant(UP_DOWN_MODE_A::COUNT_UPDN2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ONE_SHOT`"]
pub type ONE_SHOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE_SHOT`"]
pub struct ONE_SHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_SHOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADRATURE_MODE_A {
    #[doc = "0: X1 encoding (QUAD mode)"]
    X1,
    #[doc = "1: X2 encoding (QUAD mode)"]
    X2,
    #[doc = "2: X4 encoding (QUAD mode)"]
    X4,
}
impl From<QUADRATURE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: QUADRATURE_MODE_A) -> Self {
        match variant {
            QUADRATURE_MODE_A::X1 => 0,
            QUADRATURE_MODE_A::X2 => 1,
            QUADRATURE_MODE_A::X4 => 2,
        }
    }
}
#[doc = "Reader of field `QUADRATURE_MODE`"]
pub type QUADRATURE_MODE_R = crate::R<u8, QUADRATURE_MODE_A>;
impl QUADRATURE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, QUADRATURE_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(QUADRATURE_MODE_A::X1),
            1 => Val(QUADRATURE_MODE_A::X2),
            2 => Val(QUADRATURE_MODE_A::X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == QUADRATURE_MODE_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == QUADRATURE_MODE_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == QUADRATURE_MODE_A::X4
    }
}
#[doc = "Write proxy for field `QUADRATURE_MODE`"]
pub struct QUADRATURE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADRATURE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADRATURE_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "X1 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X1)
    }
    #[doc = "X2 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X2)
    }
    #[doc = "X4 encoding (QUAD mode)"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(QUADRATURE_MODE_A::X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Counter mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Timer mode"]
    TIMER,
    #[doc = "2: Capture mode"]
    CAPTURE,
    #[doc = "3: Quadrature encoding mode"]
    QUAD,
    #[doc = "4: Pulse width modulation (PWM) mode"]
    PWM,
    #[doc = "5: PWM with deadtime insertion mode"]
    PWM_DT,
    #[doc = "6: Pseudo random pulse width modulation"]
    PWM_PR,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::TIMER => 0,
            MODE_A::CAPTURE => 2,
            MODE_A::QUAD => 3,
            MODE_A::PWM => 4,
            MODE_A::PWM_DT => 5,
            MODE_A::PWM_PR => 6,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::TIMER),
            2 => Val(MODE_A::CAPTURE),
            3 => Val(MODE_A::QUAD),
            4 => Val(MODE_A::PWM),
            5 => Val(MODE_A::PWM_DT),
            6 => Val(MODE_A::PWM_PR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == MODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == MODE_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == MODE_A::QUAD
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `PWM_DT`"]
    #[inline(always)]
    pub fn is_pwm_dt(&self) -> bool {
        *self == MODE_A::PWM_DT
    }
    #[doc = "Checks if the value of the field is `PWM_PR`"]
    #[inline(always)]
    pub fn is_pwm_pr(&self) -> bool {
        *self == MODE_A::PWM_PR
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer mode"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(MODE_A::TIMER)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(MODE_A::CAPTURE)
    }
    #[doc = "Quadrature encoding mode"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(MODE_A::QUAD)
    }
    #[doc = "Pulse width modulation (PWM) mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
    #[doc = "PWM with deadtime insertion mode"]
    #[inline(always)]
    pub fn pwm_dt(self) -> &'a mut W {
        self.variant(MODE_A::PWM_DT)
    }
    #[doc = "Pseudo random pulse width modulation"]
    #[inline(always)]
    pub fn pwm_pr(self) -> &'a mut W {
        self.variant(MODE_A::PWM_PR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc(&self) -> AUTO_RELOAD_CC_R {
        AUTO_RELOAD_CC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_period(&self) -> AUTO_RELOAD_PERIOD_R {
        AUTO_RELOAD_PERIOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&self) -> PWM_SYNC_KILL_R {
        PWM_SYNC_KILL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&self) -> PWM_STOP_ON_KILL_R {
        PWM_STOP_ON_KILL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn generic(&self) -> GENERIC_R {
        GENERIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    pub fn up_down_mode(&self) -> UP_DOWN_MODE_R {
        UP_DOWN_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&self) -> ONE_SHOT_R {
        ONE_SHOT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quadrature_mode(&self) -> QUADRATURE_MODE_R {
        QUADRATURE_MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies switching of the CC and buffered CC values. This field has a function in TIMER, PWM, PWM_DT and PWM_PR modes. Timer mode: '0': never switch. '1': switch on a compare match event. PWM, PWM_DT, PWM_PR modes: '0: never switch. '1': switch on a terminal count event with an actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_cc(&mut self) -> AUTO_RELOAD_CC_W {
        AUTO_RELOAD_CC_W { w: self }
    }
    #[doc = "Bit 1 - Specifies switching of the PERIOD and buffered PERIOD values. This field has a function in PWM, PWM_DT and PWM_PR modes. '0': never switch. '1': switch on a terminal count event with and actively pending switch event."]
    #[inline(always)]
    pub fn auto_reload_period(&mut self) -> AUTO_RELOAD_PERIOD_W {
        AUTO_RELOAD_PERIOD_W { w: self }
    }
    #[doc = "Bit 2 - Specifies asynchronous/synchronous kill behavior: '1': synchronous kill mode: the kill event disables the 'dt_line_out' and 'dt_line_compl_out' signals till the next terminal count event (synchronous kill). In synchronous kill mode, STOP_EDGE should be RISING_EDGE. '0': asynchronous kill mode: the kill event only disables the 'dt_line_out' and 'dt_line_compl_out' signals when present. In asynchronous kill mode, STOP_EDGE should be NO_EDGE_DET. This field has a function in PWM and PWM_DT modes only. This field is only used when PWM_STOP_ON_KILL is '0'."]
    #[inline(always)]
    pub fn pwm_sync_kill(&mut self) -> PWM_SYNC_KILL_W {
        PWM_SYNC_KILL_W { w: self }
    }
    #[doc = "Bit 3 - Specifies whether the counter stops on a kill events: '0': kill event does NOT stop counter. '1': kill event stops counter. This field has a function in PWM, PWM_DT and PWM_PR modes only."]
    #[inline(always)]
    pub fn pwm_stop_on_kill(&mut self) -> PWM_STOP_ON_KILL_W {
        PWM_STOP_ON_KILL_W { w: self }
    }
    #[doc = "Bits 8:15 - Generic 8-bit control field. In PWM_DT mode, this field is used to determine the dead time: amount of dead time cycles in the counter clock domain. In all other modes, the lower 3 bits of this field determine pre-scaling of the selected counter clock."]
    #[inline(always)]
    pub fn generic(&mut self) -> GENERIC_W {
        GENERIC_W { w: self }
    }
    #[doc = "Bits 16:17 - Determines counter direction."]
    #[inline(always)]
    pub fn up_down_mode(&mut self) -> UP_DOWN_MODE_W {
        UP_DOWN_MODE_W { w: self }
    }
    #[doc = "Bit 18 - When '0', counter runs continuous. When '1', counter is turned off by hardware when a terminal count event is generated."]
    #[inline(always)]
    pub fn one_shot(&mut self) -> ONE_SHOT_W {
        ONE_SHOT_W { w: self }
    }
    #[doc = "Bits 20:21 - In QUAD mode selects quadrature encoding mode (X1/X2/X4). In PWM, PWM_DT and PWM_PR modes, these two bits can be used to invert 'dt_line_out' and 'dt_line_compl_out'. Inversion is the last step in generation of 'dt_line_out' and 'dt_line_compl_out'; i.e. a disabled output line 'dt_line_out' has the value QUADRATURE_MODE\\[0\\] and a disabled output line 'dt_line_compl_out' has the value QUADRATURE_MODE\\[1\\]."]
    #[inline(always)]
    pub fn quadrature_mode(&mut self) -> QUADRATURE_MODE_W {
        QUADRATURE_MODE_W { w: self }
    }
    #[doc = "Bits 24:26 - Counter mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
