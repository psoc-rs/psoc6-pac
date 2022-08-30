#[doc = "Register `HOST_EP2_CTL` reader"]
pub struct R(crate::R<HOST_EP2_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_EP2_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_EP2_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_EP2_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_EP2_CTL` writer"]
pub struct W(crate::W<HOST_EP2_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_EP2_CTL_SPEC>;
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
impl From<crate::W<HOST_EP2_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_EP2_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKS2` reader - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMEA='1') is used, this Endpoint must not set from 0 to 2."]
pub type PKS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKS2` writer - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMEA='1') is used, this Endpoint must not set from 0 to 2."]
pub type PKS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HOST_EP2_CTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `NULLE` reader - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
pub type NULLE_R = crate::BitReader<bool>;
#[doc = "Field `NULLE` writer - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
pub type NULLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_EP2_CTL_SPEC, bool, O>;
#[doc = "Field `DMAE` reader - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
pub type DMAE_R = crate::BitReader<bool>;
#[doc = "Field `DMAE` writer - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_EP2_CTL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_EP2_CTL_SPEC, bool, O>;
#[doc = "Field `BFINI` reader - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
pub type BFINI_R = crate::BitReader<bool>;
#[doc = "Field `BFINI` writer - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
pub type BFINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HOST_EP2_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMEA='1') is used, this Endpoint must not set from 0 to 2."]
    #[inline(always)]
    pub fn pks2(&self) -> PKS2_R {
        PKS2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn nulle(&self) -> NULLE_R {
        NULLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
    #[inline(always)]
    pub fn bfini(&self) -> BFINI_R {
        BFINI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - This bit specifies the maximum size transferred by one packet. The configurable range is from 0x001 to 0x40. - If automatic buffer transfer mode (DMEA='1') is used, this Endpoint must not set from 0 to 2."]
    #[inline(always)]
    pub fn pks2(&mut self) -> PKS2_W<0> {
        PKS2_W::new(self)
    }
    #[doc = "Bit 10 - When a data transfer request in OUT the direction is transmitted while automatic buffer transfer mode is set (DMAE = 1), this bit sets a mode that transfers 0-byte data automatically upon the detection of the last packet transfer. '0' : Releases the NULL automatic transfer mode. '1' : Sets the NULL automatic transfer mode. Note : - For data transfer in the IN direction or when automatic buffer transfer mode is not set, the NULL bit configuration does not affect communication."]
    #[inline(always)]
    pub fn nulle(&mut self) -> NULLE_W<10> {
        NULLE_W::new(self)
    }
    #[doc = "Bit 11 - This bit sets a mode that uses DMA for writing or reading transfer data to/from send/receive buffer, and automatically transfers the send/receive data synchronized with an data request in the IN or OUT direction. Until the data size set in the DMA is reached, the data is transferred. '0' : Releases the automatic buffer transfer mode. '1' : Sets the automatic buffer transfer mode. Note : - The CPU must not access the send/receive buffer while the DMAE bit is set to '1'. For data transfer in the IN direction, set the DMA transfer size to the multiples of that set in PKS bits of the Host EP1 Control Register (HOST_EP1_CTL) and Host EP2 Control Register (HOST_EP2_CTR)."]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<11> {
        DMAE_W::new(self)
    }
    #[doc = "Bit 12 - This bit specifies the transfer direction the Endpoint support. '0' : IN Endpoint. '1' : OUT Endpoint Note: - This bit must be changed when INI_ST bit of the Host Endpoint 2 Status Register (HOST_EP2_STATUS) is '1'."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<12> {
        DIR_W::new(self)
    }
    #[doc = "Bit 15 - This bit initializes the send/receive buffer of transfer data. The BFINI bit is also automatically set by setting the RST bit of the HOST Control 1 Register (HOST_CTL1). If the RST bit was used for resetting, therefore, set the RST bit to '0' before clearing the BFINI bit. '0' : Clears the initialization. '1' : Initializes the send/receive buffer Note : - The EP2 buffer has a double-buffer configuration. The BFINI bit initialization initializes the double buffers concurrently and also initializes the EP2DRQ and EP2SPK bits."]
    #[inline(always)]
    pub fn bfini(&mut self) -> BFINI_W<15> {
        BFINI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Endpoint 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep2_ctl](index.html) module"]
pub struct HOST_EP2_CTL_SPEC;
impl crate::RegisterSpec for HOST_EP2_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ep2_ctl::R](R) reader structure"]
impl crate::Readable for HOST_EP2_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ep2_ctl::W](W) writer structure"]
impl crate::Writable for HOST_EP2_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_EP2_CTL to value 0x8040"]
impl crate::Resettable for HOST_EP2_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8040
    }
}
