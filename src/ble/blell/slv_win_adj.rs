#[doc = "Reader of register SLV_WIN_ADJ"]
pub type R = crate::R<u32, super::SLV_WIN_ADJ>;
#[doc = "Writer for register SLV_WIN_ADJ"]
pub type W = crate::W<u32, super::SLV_WIN_ADJ>;
#[doc = "Register SLV_WIN_ADJ `reset()`'s with value 0x10"]
impl crate::ResetValue for super::SLV_WIN_ADJ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `SLV_WIN_ADJ`"]
pub type SLV_WIN_ADJ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLV_WIN_ADJ`"]
pub struct SLV_WIN_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WIN_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
    #[inline(always)]
    pub fn slv_win_adj(&self) -> SLV_WIN_ADJ_R {
        SLV_WIN_ADJ_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Window Adjust value. This value is added to the calculated slave window widening value to be used as final window widen value."]
    #[inline(always)]
    pub fn slv_win_adj(&mut self) -> SLV_WIN_ADJ_W {
        SLV_WIN_ADJ_W { w: self }
    }
}
