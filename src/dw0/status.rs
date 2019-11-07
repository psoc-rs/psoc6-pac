#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Reader of field `CH_IDX`"]
pub type CH_IDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `PREEMPTABLE`"]
pub type PREEMPTABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Active channel protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active channel index."]
    #[inline(always)]
    pub fn ch_idx(&self) -> CH_IDX_R {
        CH_IDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Active channel priority."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Active channel preemptable."]
    #[inline(always)]
    pub fn preemptable(&self) -> PREEMPTABLE_R {
        PREEMPTABLE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': Update of active control information (e.g. source and destination addresses). '5': Wait for trigger de-activation."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Active channel present: '0': No. '1': Yes."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
