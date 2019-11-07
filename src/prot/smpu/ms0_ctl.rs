#[doc = "Reader of register MS0_CTL"]
pub type R = crate::R<u32, super::MS0_CTL>;
#[doc = "Writer for register MS0_CTL"]
pub type W = crate::W<u32, super::MS0_CTL>;
#[doc = "Register MS0_CTL `reset()`'s with value 0x0303"]
impl crate::ResetValue for super::MS0_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0303
    }
}
#[doc = "Reader of field `P`"]
pub type P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P`"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
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
#[doc = "Reader of field `NS`"]
pub type NS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NS`"]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
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
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIO`"]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PC_MASK_0`"]
pub type PC_MASK_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PC_MASK_15_TO_1`"]
pub type PC_MASK_15_TO_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PC_MASK_15_TO_1`"]
pub struct PC_MASK_15_TO_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_MASK_15_TO_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | (((value as u32) & 0x7fff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-escure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Protection context mask for protection context '0'. This field is a constant '0': - PC_MASK_0 is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to '0' and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PC_MASK_0_R {
        PC_MASK_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\] indicates if the MPU MS_CTL.PC\\[3:0\\] protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\] is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to 'i+1'; and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed. - PC_MASK_15_TO_1\\[i\\] is '1': MPU MS_CTL.PC\\[3:0\\] can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\] is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\] always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\] on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PC_MASK_15_TO_1_R {
        PC_MASK_15_TO_1_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged setting ('0': user mode; '1': privileged mode). Notes: This field is ONLY used for masters that do NOT provide their own user/privileged access control attribute. The default/reset field value provides privileged mode access capabilities."]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Bit 1 - Security setting ('0': secure mode; '1': non-secure mode). Notes: This field is ONLY used for masters that do NOT provide their own secure/non-escure access control attribute. Note that the default/reset field value provides non-secure mode access capabilities to all masters."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bits 8:9 - Device wide bus arbitration priority setting ('0': highest priority, '3': lowest priority). Notes: The AHB-Lite interconnect performs arbitration on the individual beats/transfers of a burst (this optimizes latency over locality/bandwidth). The AXI-Lite interconnects performs a single arbitration for the complete burst (this optimizes locality/bandwidth over latency). Masters with the same priority setting form a 'priority group'. Within a 'priority group', roundrobin arbitration is performed."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
    #[doc = "Bits 17:31 - Protection context mask for protection contexts '15' downto '1'. Bit PC_MASK_15_TO_1\\[i\\] indicates if the MPU MS_CTL.PC\\[3:0\\] protection context field can be set to the value 'i+1': - PC_MASK_15_TO_1\\[i\\] is '0': MPU MS_CTL.PC\\[3:0\\] can NOT be set to 'i+1'; and PC\\[3:0\\] is not changed. If the protection context of the write transfer is '0', protection is not applied and PC\\[3:0\\] can be changed. - PC_MASK_15_TO_1\\[i\\] is '1': MPU MS_CTL.PC\\[3:0\\] can be set to 'i+1'. Note: When CPUSS_CM0_PC_CTL.VALID\\[i\\] is '1' (the associated protection context handler is valid), write transfers to PC_MASK_15_TO_1\\[i-1\\] always write '0', regardless of data written. This ensures that when valid protection context handlers are used to enter protection contexts 1, 2 or 3 through (HW modifies MPU MS_CTL.PC\\[3:0\\] on entry of the handler), it is NOT possible for SW to enter those protection contexts (SW modifies MPU MS_CTL.PC\\[3:0\\])."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&mut self) -> PC_MASK_15_TO_1_W {
        PC_MASK_15_TO_1_W { w: self }
    }
}
