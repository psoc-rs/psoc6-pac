#[doc = "Register `NEXT_CONN` reader"]
pub struct R(crate::R<NEXT_CONN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_CONN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_CONN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_CONN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NEXT_CONN` writer"]
pub struct W(crate::W<NEXT_CONN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NEXT_CONN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NEXT_CONN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NEXT_CONN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXT_CONN_INDEX` reader - Connection Index to be serviced. Allowed values are 0,1,2,3."]
pub type NEXT_CONN_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXT_CONN_INDEX` writer - Connection Index to be serviced. Allowed values are 0,1,2,3."]
pub type NEXT_CONN_INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NEXT_CONN_SPEC, u8, u8, 5, O>;
#[doc = "Field `NEXT_CONN_TYPE` reader - Connection type 1 - Master Connection 0 - Slave Connection"]
pub type NEXT_CONN_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `NEXT_CONN_TYPE` writer - Connection type 1 - Master Connection 0 - Slave Connection"]
pub type NEXT_CONN_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NEXT_CONN_SPEC, bool, O>;
#[doc = "Field `NI_VALID` reader - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
pub type NI_VALID_R = crate::BitReader<bool>;
#[doc = "Field `NI_VALID` writer - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
pub type NI_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, NEXT_CONN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&self) -> NEXT_CONN_INDEX_R {
        NEXT_CONN_INDEX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&self) -> NEXT_CONN_TYPE_R {
        NEXT_CONN_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&self) -> NI_VALID_R {
        NI_VALID_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&mut self) -> NEXT_CONN_INDEX_W<0> {
        NEXT_CONN_INDEX_W::new(self)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&mut self) -> NEXT_CONN_TYPE_W<5> {
        NEXT_CONN_TYPE_W::new(self)
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&mut self) -> NI_VALID_W<6> {
        NI_VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_conn](index.html) module"]
pub struct NEXT_CONN_SPEC;
impl crate::RegisterSpec for NEXT_CONN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_conn::R](R) reader structure"]
impl crate::Readable for NEXT_CONN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [next_conn::W](W) writer structure"]
impl crate::Writable for NEXT_CONN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NEXT_CONN to value 0"]
impl crate::Resettable for NEXT_CONN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
