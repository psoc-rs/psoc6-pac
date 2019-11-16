#[doc = "Reader of register ARB_CFG"]
pub type R = crate::R<u32, super::ARB_CFG>;
#[doc = "Writer for register ARB_CFG"]
pub type W = crate::W<u32, super::ARB_CFG>;
#[doc = "Register ARB_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::ARB_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTO_MEM`"]
pub type AUTO_MEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_MEM`"]
pub struct AUTO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_MEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "DMA Access Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_CFG_A {
    #[doc = "0: No DMA"]
    DMA_NONE = 0,
    #[doc = "1: Manual DMA"]
    DMA_MANUAL = 1,
    #[doc = "2: Auto DMA"]
    DMA_AUTO = 2,
}
impl From<DMA_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA_CFG`"]
pub type DMA_CFG_R = crate::R<u8, DMA_CFG_A>;
impl DMA_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMA_CFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMA_CFG_A::DMA_NONE),
            1 => Val(DMA_CFG_A::DMA_MANUAL),
            2 => Val(DMA_CFG_A::DMA_AUTO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_NONE`"]
    #[inline(always)]
    pub fn is_dma_none(&self) -> bool {
        *self == DMA_CFG_A::DMA_NONE
    }
    #[doc = "Checks if the value of the field is `DMA_MANUAL`"]
    #[inline(always)]
    pub fn is_dma_manual(&self) -> bool {
        *self == DMA_CFG_A::DMA_MANUAL
    }
    #[doc = "Checks if the value of the field is `DMA_AUTO`"]
    #[inline(always)]
    pub fn is_dma_auto(&self) -> bool {
        *self == DMA_CFG_A::DMA_AUTO
    }
}
#[doc = "Write proxy for field `DMA_CFG`"]
pub struct DMA_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_CFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn dma_none(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_NONE)
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn dma_manual(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_MANUAL)
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn dma_auto(self) -> &'a mut W {
        self.variant(DMA_CFG_A::DMA_AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `CFG_CMP`"]
pub type CFG_CMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFG_CMP`"]
pub struct CFG_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_CMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&self) -> AUTO_MEM_R {
        AUTO_MEM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&self) -> DMA_CFG_R {
        DMA_CFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&self) -> CFG_CMP_R {
        CFG_CMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&mut self) -> AUTO_MEM_W {
        AUTO_MEM_W { w: self }
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&mut self) -> DMA_CFG_W {
        DMA_CFG_W { w: self }
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&mut self) -> CFG_CMP_W {
        CFG_CMP_W { w: self }
    }
}
