#[doc = "Register `CH_CTL` reader"]
pub struct R(crate::R<CH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_CTL` writer"]
pub struct W(crate::W<CH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CTL_SPEC>;
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
impl From<crate::W<CH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CTL_SPEC, bool, O>;
#[doc = "Field `NS` reader - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CTL_SPEC, bool, O>;
#[doc = "Field `B` reader - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CTL_SPEC, bool, O>;
#[doc = "Field `PC` reader - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
pub type PC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PC` writer - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
pub type PC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRIO` reader - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PREEMPTABLE` reader - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
pub type PREEMPTABLE_R = crate::BitReader<bool>;
#[doc = "Field `PREEMPTABLE` writer - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
pub type PREEMPTABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    pub fn preemptable(&self) -> PREEMPTABLE_R {
        PREEMPTABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the P field for the user/privileged access control ('hprot\\[1\\]')."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bit 1 - Secure/on-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the transaction that writes this register; i.e. the 'write data' is ignored and instead the access control is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel use the NS field for the secure/non-secure access control ('hprot\\[4\\]')."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<1> {
        NS_W::new(self)
    }
    #[doc = "Bit 2 - Non-bufferable/bufferable access control: '0': non-bufferable. '1': bufferable. This field is used to indicate to an AMBA bridge that a write transaction can complete without waiting for the destination to accept the write transaction data. All transactions for this channel uses the B field for the non-bufferable/bufferable access control ('hprot\\[2\\]')."]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<2> {
        B_W::new(self)
    }
    #[doc = "Bits 4:7 - Protection context. This field is set with the protection context of the transaction that writes this register; i.e. the 'write data' is ignored and instead the context is inherited from the write transaction (note the field attributes should be HW:RW, SW:R). All transactions for this channel uses the PC field for the protection context."]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W<4> {
        PC_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel priority: '0': highest priority. '1' '2' '3': lowest priority. Channels with the same priority constitute a priority group. Priority decoding determines the highest priority pending channel. This channel is determined as follows. First, the highest priority group with pending channels is identified. Second, within this priority group, round robin arbitration is applied. Round robin arbitration (within a priority group) gives the highest priority to the lower channel indices (within the priority group)."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<16> {
        PRIO_W::new(self)
    }
    #[doc = "Bit 18 - Specifies if the channel is preemptable. '0': Not preemptable. '1': Preemptable. This field allows higher priority pending channels (from a higher priority group; i.e. an active channel can NOT be preempted by a pending channel in the same priority group) to preempt the active channel in between 'single transfers' (a 1D transfer consists out of X_COUNT single transfers; a 2D transfer consists out of X_COUNT*Y_COUNT single transfers). Preemption will NOT affect the pending status of channel. As a result, after completion of a higher priority activated channel, the current channel may be reactivated."]
    #[inline(always)]
    pub fn preemptable(&mut self) -> PREEMPTABLE_W<18> {
        PREEMPTABLE_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. The channel's trigger is ignored and the channel cannot be made pending and therefore cannot be made active. If a pending channel is disabled, the channel is made non pending. If the activate channel is disabled, the channel is de-activated (bus transactions are completed). '1': Enabled. SW sets this field to '1' to enable a specific channel. HW sets this field to '0' on an error interrupt cause (the specific error is specified by CH_STATUS.INTR_CAUSE)."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ctl](index.html) module"]
pub struct CH_CTL_SPEC;
impl crate::RegisterSpec for CH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_ctl::R](R) reader structure"]
impl crate::Readable for CH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_ctl::W](W) writer structure"]
impl crate::Writable for CH_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_CTL to value 0x02"]
impl crate::Resettable for CH_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
