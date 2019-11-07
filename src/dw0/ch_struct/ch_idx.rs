#[doc = "Reader of register CH_IDX"]
pub type R = crate::R<u32, super::CH_IDX>;
#[doc = "Writer for register CH_IDX"]
pub type W = crate::W<u32, super::CH_IDX>;
#[doc = "Register CH_IDX `reset()`'s with value 0"]
impl crate::ResetValue for super::CH_IDX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X_IDX`"]
pub type X_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `X_IDX`"]
pub struct X_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> X_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `Y_IDX`"]
pub type Y_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Y_IDX`"]
pub struct Y_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&self) -> X_IDX_R {
        X_IDX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&self) -> Y_IDX_R {
        Y_IDX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&mut self) -> X_IDX_W {
        X_IDX_W { w: self }
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&mut self) -> Y_IDX_W {
        Y_IDX_W { w: self }
    }
}
