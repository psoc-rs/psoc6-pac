#[doc = "Register `CLK_TRIM_PILO_CTL3` reader"]
pub struct R(crate::R<CLK_TRIM_PILO_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_PILO_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_PILO_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_PILO_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_PILO_CTL3` writer"]
pub struct W(crate::W<CLK_TRIM_PILO_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_PILO_CTL3_SPEC>;
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
impl From<crate::W<CLK_TRIM_PILO_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_PILO_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PILO_ENGOPT` reader - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PILO_ENGOPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PILO_ENGOPT` writer - Engineering options for PILO circuits 0: Short vdda to vpwr 1: Beta:mult current change 2: Iref generation Ptat current addition 3: Disable current path in secondary Beta:mult startup circuit 4: Double oscillator current 5: Switch between deep:sub:threshold and sub:threshold stacks in Vref generation block 6: Spare 7: Ptat component increase in Iref 8: vpwr_rc and vpwr_dig_rc shorting testmode 9: Switch b/w psub connection for cascode nfet for vref generation 10: Switch between sub:threshold and deep:sub:threshold stacks in comparator. 15-11: Frequency fine trim. See AKK-444 for an overview of the trim strategy."]
pub type PILO_ENGOPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_PILO_CTL3_SPEC, u16, u16, 16, O>;
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
    pub fn pilo_engopt(&mut self) -> PILO_ENGOPT_W<0> {
        PILO_ENGOPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PILO Trim Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_pilo_ctl3](index.html) module"]
pub struct CLK_TRIM_PILO_CTL3_SPEC;
impl crate::RegisterSpec for CLK_TRIM_PILO_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_pilo_ctl3::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_PILO_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_pilo_ctl3::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_PILO_CTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL3 to value 0x4800"]
impl crate::Resettable for CLK_TRIM_PILO_CTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4800
    }
}
