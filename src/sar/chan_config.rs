#[doc = "Reader of register CHAN_CONFIG[%s]"]
pub type R = crate::R<u32, super::CHAN_CONFIG>;
#[doc = "Writer for register CHAN_CONFIG[%s]"]
pub type W = crate::W<u32, super::CHAN_CONFIG>;
#[doc = "Register CHAN_CONFIG[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CHAN_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POS_PIN_ADDR`"]
pub type POS_PIN_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POS_PIN_ADDR`"]
pub struct POS_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POS_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX,
    #[doc = "1: CTB0"]
    CTB0,
    #[doc = "2: CTB1"]
    CTB1,
    #[doc = "3: CTB2"]
    CTB2,
    #[doc = "4: CTB3"]
    CTB3,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT,
}
impl From<POS_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: POS_PORT_ADDR_A) -> Self {
        match variant {
            POS_PORT_ADDR_A::SARMUX => 0,
            POS_PORT_ADDR_A::CTB0 => 1,
            POS_PORT_ADDR_A::CTB1 => 2,
            POS_PORT_ADDR_A::CTB2 => 3,
            POS_PORT_ADDR_A::CTB3 => 4,
            POS_PORT_ADDR_A::AROUTE_VIRT2 => 5,
            POS_PORT_ADDR_A::AROUTE_VIRT1 => 6,
            POS_PORT_ADDR_A::SARMUX_VIRT => 7,
        }
    }
}
#[doc = "Reader of field `POS_PORT_ADDR`"]
pub type POS_PORT_ADDR_R = crate::R<u8, POS_PORT_ADDR_A>;
impl POS_PORT_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POS_PORT_ADDR_A {
        match self.bits {
            0 => POS_PORT_ADDR_A::SARMUX,
            1 => POS_PORT_ADDR_A::CTB0,
            2 => POS_PORT_ADDR_A::CTB1,
            3 => POS_PORT_ADDR_A::CTB2,
            4 => POS_PORT_ADDR_A::CTB3,
            5 => POS_PORT_ADDR_A::AROUTE_VIRT2,
            6 => POS_PORT_ADDR_A::AROUTE_VIRT1,
            7 => POS_PORT_ADDR_A::SARMUX_VIRT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == POS_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == POS_PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT2`"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == POS_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT1`"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == POS_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == POS_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Write proxy for field `POS_PORT_ADDR`"]
pub struct POS_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POS_PORT_ADDR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIFFERENTIAL_EN`"]
pub type DIFFERENTIAL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIFFERENTIAL_EN`"]
pub struct DIFFERENTIAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFERENTIAL_EN_W<'a> {
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
#[doc = "Reader of field `AVG_EN`"]
pub type AVG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVG_EN`"]
pub struct AVG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_EN_W<'a> {
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
#[doc = "Reader of field `SAMPLE_TIME_SEL`"]
pub type SAMPLE_TIME_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMPLE_TIME_SEL`"]
pub struct SAMPLE_TIME_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `NEG_PIN_ADDR`"]
pub type NEG_PIN_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEG_PIN_ADDR`"]
pub struct NEG_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Address of the neg port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEG_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT,
}
impl From<NEG_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_PORT_ADDR_A) -> Self {
        match variant {
            NEG_PORT_ADDR_A::SARMUX => 0,
            NEG_PORT_ADDR_A::AROUTE_VIRT2 => 5,
            NEG_PORT_ADDR_A::AROUTE_VIRT1 => 6,
            NEG_PORT_ADDR_A::SARMUX_VIRT => 7,
        }
    }
}
#[doc = "Reader of field `NEG_PORT_ADDR`"]
pub type NEG_PORT_ADDR_R = crate::R<u8, NEG_PORT_ADDR_A>;
impl NEG_PORT_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NEG_PORT_ADDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NEG_PORT_ADDR_A::SARMUX),
            5 => Val(NEG_PORT_ADDR_A::AROUTE_VIRT2),
            6 => Val(NEG_PORT_ADDR_A::AROUTE_VIRT1),
            7 => Val(NEG_PORT_ADDR_A::SARMUX_VIRT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == NEG_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT2`"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == NEG_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT1`"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == NEG_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == NEG_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Write proxy for field `NEG_PORT_ADDR`"]
pub struct NEG_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEG_PORT_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::SARMUX)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `NEG_ADDR_EN`"]
pub type NEG_ADDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEG_ADDR_EN`"]
pub struct NEG_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_ADDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DSI_OUT_EN`"]
pub type DSI_OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_OUT_EN`"]
pub struct DSI_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_OUT_EN_W<'a> {
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
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&self) -> POS_PIN_ADDR_R {
        POS_PIN_ADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&self) -> POS_PORT_ADDR_R {
        POS_PORT_ADDR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn differential_en(&self) -> DIFFERENTIAL_EN_R {
        DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SAMPLE_TIME_SEL_R {
        SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&self) -> NEG_PIN_ADDR_R {
        NEG_PIN_ADDR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&self) -> NEG_PORT_ADDR_R {
        NEG_PORT_ADDR_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&self) -> NEG_ADDR_EN_R {
        NEG_ADDR_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DSI_OUT_EN_R {
        DSI_OUT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&mut self) -> POS_PIN_ADDR_W {
        POS_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&mut self) -> POS_PORT_ADDR_W {
        POS_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\] is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\] is ignored)."]
    #[inline(always)]
    pub fn differential_en(&mut self) -> DIFFERENTIAL_EN_W {
        DIFFERENTIAL_EN_W { w: self }
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&mut self) -> AVG_EN_W {
        AVG_EN_W { w: self }
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&mut self) -> SAMPLE_TIME_SEL_W {
        SAMPLE_TIME_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&mut self) -> NEG_PIN_ADDR_W {
        NEG_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&mut self) -> NEG_PORT_ADDR_W {
        NEG_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&mut self) -> NEG_ADDR_EN_W {
        NEG_ADDR_EN_W { w: self }
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&mut self) -> DSI_OUT_EN_W {
        DSI_OUT_EN_W { w: self }
    }
}
