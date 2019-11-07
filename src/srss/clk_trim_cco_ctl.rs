#[doc = "Reader of register CLK_TRIM_CCO_CTL"]
pub type R = crate::R<u32, super::CLK_TRIM_CCO_CTL>;
#[doc = "Writer for register CLK_TRIM_CCO_CTL"]
pub type W = crate::W<u32, super::CLK_TRIM_CCO_CTL>;
#[doc = "Register CLK_TRIM_CCO_CTL `reset()`'s with value 0xa700_0020"]
impl crate::ResetValue for super::CLK_TRIM_CCO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa700_0020
    }
}
#[doc = "Reader of field `CCO_RCSTRIM`"]
pub type CCO_RCSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_RCSTRIM`"]
pub struct CCO_RCSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_RCSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CCO_STABLE_CNT`"]
pub type CCO_STABLE_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_STABLE_CNT`"]
pub struct CCO_STABLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_STABLE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_CNT`"]
pub type ENABLE_CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_CNT`"]
pub struct ENABLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_CNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&self) -> CCO_RCSTRIM_R {
        CCO_RCSTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&self) -> CCO_STABLE_CNT_R {
        CCO_STABLE_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&self) -> ENABLE_CNT_R {
        ENABLE_CNT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&mut self) -> CCO_RCSTRIM_W {
        CCO_RCSTRIM_W { w: self }
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&mut self) -> CCO_STABLE_CNT_W {
        CCO_STABLE_CNT_W { w: self }
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&mut self) -> ENABLE_CNT_W {
        ENABLE_CNT_W { w: self }
    }
}
