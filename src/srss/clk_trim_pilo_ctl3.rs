#[doc = "Reader of register CLK_TRIM_PILO_CTL3"]
pub type R = crate::R<u32, super::CLK_TRIM_PILO_CTL3>;
#[doc = "Writer for register CLK_TRIM_PILO_CTL3"]
pub type W = crate::W<u32, super::CLK_TRIM_PILO_CTL3>;
#[doc = "Register CLK_TRIM_PILO_CTL3 `reset()`'s with value 0x4800"]
impl crate::ResetValue for super::CLK_TRIM_PILO_CTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4800
    }
}
#[doc = "Reader of field `PILO_ENGOPT`"]
pub type PILO_ENGOPT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PILO_ENGOPT`"]
pub struct PILO_ENGOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_ENGOPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub fn pilo_engopt(&self) -> PILO_ENGOPT_R {
        PILO_ENGOPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
    #[inline(always)]
    pub fn pilo_engopt(&mut self) -> PILO_ENGOPT_W {
        PILO_ENGOPT_W { w: self }
    }
}
