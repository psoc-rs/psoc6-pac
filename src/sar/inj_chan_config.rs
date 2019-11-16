#[doc = "Reader of register INJ_CHAN_CONFIG"]
pub type R = crate::R<u32, super::INJ_CHAN_CONFIG>;
#[doc = "Writer for register INJ_CHAN_CONFIG"]
pub type W = crate::W<u32, super::INJ_CHAN_CONFIG>;
#[doc = "Register INJ_CHAN_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::INJ_CHAN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INJ_PIN_ADDR`"]
pub type INJ_PIN_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INJ_PIN_ADDR`"]
pub struct INJ_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Address of the port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INJ_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "6: AROUTE virtual port"]
    AROUTE_VIRT = 6,
    #[doc = "7: SARMUX virtual port"]
    SARMUX_VIRT = 7,
}
impl From<INJ_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: INJ_PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INJ_PORT_ADDR`"]
pub type INJ_PORT_ADDR_R = crate::R<u8, INJ_PORT_ADDR_A>;
impl INJ_PORT_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INJ_PORT_ADDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INJ_PORT_ADDR_A::SARMUX),
            1 => Val(INJ_PORT_ADDR_A::CTB0),
            2 => Val(INJ_PORT_ADDR_A::CTB1),
            3 => Val(INJ_PORT_ADDR_A::CTB2),
            4 => Val(INJ_PORT_ADDR_A::CTB3),
            6 => Val(INJ_PORT_ADDR_A::AROUTE_VIRT),
            7 => Val(INJ_PORT_ADDR_A::SARMUX_VIRT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == INJ_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT`"]
    #[inline(always)]
    pub fn is_aroute_virt(&self) -> bool {
        *self == INJ_PORT_ADDR_A::AROUTE_VIRT
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == INJ_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Write proxy for field `INJ_PORT_ADDR`"]
pub struct INJ_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INJ_PORT_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port"]
    #[inline(always)]
    pub fn aroute_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::AROUTE_VIRT)
    }
    #[doc = "SARMUX virtual port"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `INJ_DIFFERENTIAL_EN`"]
pub type INJ_DIFFERENTIAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_DIFFERENTIAL_EN`"]
pub struct INJ_DIFFERENTIAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_DIFFERENTIAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `INJ_AVG_EN`"]
pub type INJ_AVG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_AVG_EN`"]
pub struct INJ_AVG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_AVG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `INJ_SAMPLE_TIME_SEL`"]
pub type INJ_SAMPLE_TIME_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INJ_SAMPLE_TIME_SEL`"]
pub struct INJ_SAMPLE_TIME_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_SAMPLE_TIME_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `INJ_TAILGATING`"]
pub type INJ_TAILGATING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_TAILGATING`"]
pub struct INJ_TAILGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_TAILGATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `INJ_START_EN`"]
pub type INJ_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INJ_START_EN`"]
pub struct INJ_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INJ_START_EN_W<'a> {
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
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&self) -> INJ_PIN_ADDR_R {
        INJ_PIN_ADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&self) -> INJ_PORT_ADDR_R {
        INJ_PORT_ADDR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&self) -> INJ_DIFFERENTIAL_EN_R {
        INJ_DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&self) -> INJ_AVG_EN_R {
        INJ_AVG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&self) -> INJ_SAMPLE_TIME_SEL_R {
        INJ_SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&self) -> INJ_TAILGATING_R {
        INJ_TAILGATING_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&self) -> INJ_START_EN_R {
        INJ_START_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&mut self) -> INJ_PIN_ADDR_W {
        INJ_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&mut self) -> INJ_PORT_ADDR_W {
        INJ_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&mut self) -> INJ_DIFFERENTIAL_EN_W {
        INJ_DIFFERENTIAL_EN_W { w: self }
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&mut self) -> INJ_AVG_EN_W {
        INJ_AVG_EN_W { w: self }
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&mut self) -> INJ_SAMPLE_TIME_SEL_W {
        INJ_SAMPLE_TIME_SEL_W { w: self }
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&mut self) -> INJ_TAILGATING_W {
        INJ_TAILGATING_W { w: self }
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&mut self) -> INJ_START_EN_W {
        INJ_START_EN_W { w: self }
    }
}
