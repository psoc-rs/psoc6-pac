#[doc = "Register `MS0_CTL` reader"]
pub struct R(crate::R<MS0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS0_CTL` writer"]
pub struct W(crate::W<MS0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS0_CTL_SPEC>;
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
impl From<crate::W<MS0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P` reader - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, MS0_CTL_SPEC, bool, O>;
#[doc = "Field `NS` reader - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
pub type NS_R = crate::BitReader<bool>;
#[doc = "Field `NS` writer - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
pub type NS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MS0_CTL_SPEC, bool, O>;
#[doc = "Field `PRIO` reader - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MS0_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PC_MASK_0` reader - Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to '0' and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed."]
pub type PC_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `PC_MASK_15_TO_1` reader - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
pub type PC_MASK_15_TO_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
pub type PC_MASK_15_TO_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MS0_CTL_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to '0' and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W<0> {
        P_W::new(self)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-secure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<1> {
        NS_W::new(self)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', round robin arbitration is performed."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<8> {
        PRIO_W::new(self)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' down to '1'. Bit PC_MASK_15_TO_1\\[i\\]
indicates if the MPU MS_CTL.PC\\[3:0\\]
protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\]
is '0': MPU MS_CTL.PC\\[3:0\\]
can NOT be set to 'i+1'; and PC\\[3:0\\]
is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\]
can be changed. - PC_MASK_15_TO_1\\[i\\]
is '1': MPU MS_CTL.PC\\[3:0\\]
can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\]
is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\]
always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\]
on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W<17> {
        PC_MASK_15_TO_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master 0 protection context control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms0_ctl](index.html) module"]
pub struct MS0_CTL_SPEC;
impl crate::RegisterSpec for MS0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms0_ctl::R](R) reader structure"]
impl crate::Readable for MS0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms0_ctl::W](W) writer structure"]
impl crate::Writable for MS0_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MS0_CTL to value 0x0303"]
impl crate::Resettable for MS0_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0303
    }
}
