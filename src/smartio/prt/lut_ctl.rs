#[doc = "Reader of register LUT_CTL[%s]"]
pub type R = crate::R<u32, super::LUT_CTL>;
#[doc = "Writer for register LUT_CTL[%s]"]
pub type W = crate::W<u32, super::LUT_CTL>;
#[doc = "Register LUT_CTL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::LUT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LUT`"]
pub type LUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LUT`"]
pub struct LUT_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LUT_OPC`"]
pub type LUT_OPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LUT_OPC`"]
pub struct LUT_OPC_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_OPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&self) -> LUT_R {
        LUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&self) -> LUT_OPC_R {
        LUT_OPC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LUT configuration. Depending on the LUT opcode LUT_OPC, the internal state lut_reg (captured in a flip-flop) and the LUT input signals tr0_in, tr1_in, tr2_in, the LUT configuration is used to determine the LUT output signal and the next sequential state (lut_reg)."]
    #[inline(always)]
    pub fn lut(&mut self) -> LUT_W {
        LUT_W { w: self }
    }
    #[doc = "Bits 8:9 - LUT opcode specifies the LUT operation: '0': Combinatoral output, no feedback. tr_out = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. '1': Combinatorial output, feedback. tr_out = LUT\\[{lut_reg, tr1_in, tr0_in}\\]. On clock: lut_reg <= tr_in2. '2': Sequential output, no feedback. temp = LUT\\[{tr2_in, tr1_in, tr0_in}\\]. tr_out = lut_reg. On clock: lut_reg <= temp. '3': Register with asynchronous set and reset. tr_out = lut_reg. enable = (tr2_in ^ LUT\\[4\\]) | LUT\\[5\\]. set = enable & (tr1_in ^ LUT\\[2\\]) & LUT\\[3\\]. clr = enable & (tr0_in ^ LUT\\[0\\]) & LUT\\[1\\]. Asynchronously (no clock required): lut_reg <= if (clr) '0' else if (set) '1'"]
    #[inline(always)]
    pub fn lut_opc(&mut self) -> LUT_OPC_W {
        LUT_OPC_W { w: self }
    }
}
