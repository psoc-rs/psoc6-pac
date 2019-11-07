#[doc = "Reader of register SEQ_START"]
pub type R = crate::R<u32, super::SEQ_START>;
#[doc = "Writer for register SEQ_START"]
pub type W = crate::W<u32, super::SEQ_START>;
#[doc = "Register SEQ_START `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SEQ_MODE`"]
pub type SEQ_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEQ_MODE`"]
pub struct SEQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ABORT`"]
pub type ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABORT`"]
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DSI_START_EN`"]
pub type DSI_START_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_START_EN`"]
pub struct DSI_START_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_START_EN_W<'a> {
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
#[doc = "Reader of field `AZ0_SKIP`"]
pub type AZ0_SKIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AZ0_SKIP`"]
pub struct AZ0_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ0_SKIP_W<'a> {
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
#[doc = "Reader of field `AZ1_SKIP`"]
pub type AZ1_SKIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AZ1_SKIP`"]
pub struct AZ1_SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ1_SKIP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&self) -> SEQ_MODE_R {
        SEQ_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&self) -> DSI_START_EN_R {
        DSI_START_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&self) -> AZ0_SKIP_R {
        AZ0_SKIP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&self) -> AZ1_SKIP_R {
        AZ1_SKIP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&mut self) -> SEQ_MODE_W {
        SEQ_MODE_W { w: self }
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&mut self) -> DSI_START_EN_W {
        DSI_START_EN_W { w: self }
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&mut self) -> AZ0_SKIP_W {
        AZ0_SKIP_W { w: self }
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&mut self) -> AZ1_SKIP_W {
        AZ1_SKIP_W { w: self }
    }
}
