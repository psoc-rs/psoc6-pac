#[doc = "Reader of register WAIT_CTL"]
pub type R = crate::R<u32, super::WAIT_CTL>;
#[doc = "Writer for register WAIT_CTL"]
pub type W = crate::W<u32, super::WAIT_CTL>;
#[doc = "Register WAIT_CTL `reset()`'s with value 0x0003_0b09"]
impl crate::ResetValue for super::WAIT_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0b09
    }
}
#[doc = "Reader of field `WAIT_FM_MEM_RD`"]
pub type WAIT_FM_MEM_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_MEM_RD`"]
pub struct WAIT_FM_MEM_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_MEM_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WAIT_FM_HV_RD`"]
pub type WAIT_FM_HV_RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_HV_RD`"]
pub struct WAIT_FM_HV_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_HV_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAIT_FM_HV_WR`"]
pub type WAIT_FM_HV_WR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_FM_HV_WR`"]
pub struct WAIT_FM_HV_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FM_HV_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&self) -> WAIT_FM_MEM_RD_R {
        WAIT_FM_MEM_RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&self) -> WAIT_FM_HV_RD_R {
        WAIT_FM_HV_RD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&self) -> WAIT_FM_HV_WR_R {
        WAIT_FM_HV_WR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&mut self) -> WAIT_FM_MEM_RD_W {
        WAIT_FM_MEM_RD_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&mut self) -> WAIT_FM_HV_RD_W {
        WAIT_FM_HV_RD_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&mut self) -> WAIT_FM_HV_WR_W {
        WAIT_FM_HV_WR_W { w: self }
    }
}
