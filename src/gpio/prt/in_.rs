#[doc = "Reader of register IN"]
pub type R = crate::R<u32, super::IN>;
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT_IN`"]
pub type FLT_IN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin."]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO pin state for pin 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO pin state for pin 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO pin state for pin 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO pin state for pin 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO pin state for pin 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO pin state for pin 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO pin state for pin 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
    #[inline(always)]
    pub fn flt_in(&self) -> FLT_IN_R {
        FLT_IN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
