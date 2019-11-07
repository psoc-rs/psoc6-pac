#[doc = "Reader of register TRIM_LDO_0"]
pub type R = crate::R<u32, super::TRIM_LDO_0>;
#[doc = "Writer for register TRIM_LDO_0"]
pub type W = crate::W<u32, super::TRIM_LDO_0>;
#[doc = "Register TRIM_LDO_0 `reset()`'s with value 0x58"]
impl crate::ResetValue for super::TRIM_LDO_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x58
    }
}
#[doc = "Reader of field `ACT_LDO_VREG`"]
pub type ACT_LDO_VREG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_LDO_VREG`"]
pub struct ACT_LDO_VREG_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_LDO_VREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ACT_LDO_ITAIL`"]
pub type ACT_LDO_ITAIL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_LDO_ITAIL`"]
pub struct ACT_LDO_ITAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_LDO_ITAIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - To trim the regulated voltage in steps of 25mV typically"]
    #[inline(always)]
    pub fn act_ldo_vreg(&self) -> ACT_LDO_VREG_R {
        ACT_LDO_VREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - To trim the bias currents for all the active mode blocks"]
    #[inline(always)]
    pub fn act_ldo_itail(&self) -> ACT_LDO_ITAIL_R {
        ACT_LDO_ITAIL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - To trim the regulated voltage in steps of 25mV typically"]
    #[inline(always)]
    pub fn act_ldo_vreg(&mut self) -> ACT_LDO_VREG_W {
        ACT_LDO_VREG_W { w: self }
    }
    #[doc = "Bits 4:7 - To trim the bias currents for all the active mode blocks"]
    #[inline(always)]
    pub fn act_ldo_itail(&mut self) -> ACT_LDO_ITAIL_W {
        ACT_LDO_ITAIL_W { w: self }
    }
}
