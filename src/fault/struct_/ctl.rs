#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR_EN`"]
pub type TR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR_EN`"]
pub struct TR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_EN_W<'a> {
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
#[doc = "Reader of field `OUT_EN`"]
pub type OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_EN`"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
#[doc = "Reader of field `RESET_REQ_EN`"]
pub type RESET_REQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_REQ_EN`"]
pub struct RESET_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_REQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&self) -> TR_EN_R {
        TR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&self) -> RESET_REQ_EN_R {
        RESET_REQ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&mut self) -> TR_EN_W {
        TR_EN_W { w: self }
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&mut self) -> RESET_REQ_EN_W {
        RESET_REQ_EN_W { w: self }
    }
}
