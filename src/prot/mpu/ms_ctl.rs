#[doc = "Reader of register MS_CTL"]
pub type R = crate::R<u32, super::MS_CTL>;
#[doc = "Writer for register MS_CTL"]
pub type W = crate::W<u32, super::MS_CTL>;
#[doc = "Register MS_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MS_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC`"]
pub struct PC_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PC_SAVED`"]
pub type PC_SAVED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PC_SAVED`"]
pub struct PC_SAVED_W<'a> {
    w: &'a mut W,
}
impl<'a> PC_SAVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\] fields."]
    #[inline(always)]
    pub fn pc_saved(&self) -> PC_SAVED_R {
        PC_SAVED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn pc(&mut self) -> PC_W {
        PC_W { w: self }
    }
    #[doc = "Bits 16:19 - Saved protection context. Modifications to this field are constrained by the associated MS_CTL.PC_MASK_0 and MS_CTL.PC_MASK_15_TO_1\\[\\] fields."]
    #[inline(always)]
    pub fn pc_saved(&mut self) -> PC_SAVED_W {
        PC_SAVED_W { w: self }
    }
}
