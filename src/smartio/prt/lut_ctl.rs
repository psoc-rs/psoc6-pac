#[doc = "Register `LUT_CTL[%s]` reader"]
pub struct R(crate::R<LUT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT_CTL[%s]` writer"]
pub struct W(crate::W<LUT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_CTL_SPEC>;
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
impl From<crate::W<LUT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT` reader - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub type LUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT` writer - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
pub type LUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `LUT_OPC` reader - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
pub type LUT_OPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_OPC` writer - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
pub type LUT_OPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&self) -> LUT_R {
        LUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&self) -> LUT_OPC_R {
        LUT_OPC_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&mut self) -> LUT_W<0> {
        LUT_W::new(self)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&mut self) -> LUT_OPC_W<8> {
        LUT_OPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_ctl](index.html) module"]
pub struct LUT_CTL_SPEC;
impl crate::RegisterSpec for LUT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut_ctl::R](R) reader structure"]
impl crate::Readable for LUT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut_ctl::W](W) writer structure"]
impl crate::Writable for LUT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT_CTL[%s]
to value 0"]
impl crate::Resettable for LUT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
