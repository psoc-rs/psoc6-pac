#[doc = "Reader of register RES_CAUSE2"]
pub type R = crate::R<u32, super::RES_CAUSE2>;
#[doc = "Writer for register RES_CAUSE2"]
pub type W = crate::W<u32, super::RES_CAUSE2>;
#[doc = "Register RES_CAUSE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RES_CAUSE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESET_CSV_HF_LOSS`"]
pub type RESET_CSV_HF_LOSS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESET_CSV_HF_LOSS`"]
pub struct RESET_CSV_HF_LOSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CSV_HF_LOSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RESET_CSV_HF_FREQ`"]
pub type RESET_CSV_HF_FREQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESET_CSV_HF_FREQ`"]
pub struct RESET_CSV_HF_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CSV_HF_FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&self) -> RESET_CSV_HF_LOSS_R {
        RESET_CSV_HF_LOSS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&self) -> RESET_CSV_HF_FREQ_R {
        RESET_CSV_HF_FREQ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&mut self) -> RESET_CSV_HF_LOSS_W {
        RESET_CSV_HF_LOSS_W { w: self }
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK<K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&mut self) -> RESET_CSV_HF_FREQ_W {
        RESET_CSV_HF_FREQ_W { w: self }
    }
}
