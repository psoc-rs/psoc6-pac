#[doc = "Register `CE_LENGTH` reader"]
pub struct R(crate::R<CE_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_LENGTH` writer"]
pub struct W(crate::W<CE_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_LENGTH_SPEC>;
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
impl From<crate::W<CE_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONNECTION_EVENT_LENGTH` reader - This field defines the length of Connection event. This value is derived from the CE length HCI parameters received from the host. This determines the number of master transmit slots in a connection event, subject to either of the MD bits being set. If both MD bits are set to 0, this has no effect. Units: 625us Note: The connection event length as specified by the CE_LENGTH shall not exceed CONN_INTERVAL - 1.25 ms. The CE-length parameter, according to the Bluetooth specification, is the length of the connection event. Take an example to illustrate this scenario: Assume a connection with interval = 100ms. that the application has put allowed 20ms of CE-length. Here, the CE-length can be upto 100ms (100ms - 150us to be exact). If the connection is maintained for 5 minutes, there could be 10*60*5 = 3000 connection-intervals. The CE-length need not be maintained constant during all the 3000 connection events. Here are the typical cases that determine the value of CE-length: (1) No data packets exchanged. we are just maintaining time and frequency synchronization. In this case, only a packet pair will be exchanged every connection interval. Here, CE-length = 1. (2) Average of 10 packets to be sent per connection event. We can pump data in multiple ways here: 2.1: Send data at uniform rate : In this case, the CE-length will be enough to accommodate 10 packets, which will take about 7ms. As this is less than application enforced limit of 20ms, we can comfortably push all the 10 data packets in this connection interval. So data will be pumped to the other BT device at the same rate as is received from my application. 2.2: Can send data in bursts. Assume that we accumulate data for 1 second and pump out at the end of 1 second(this is not done by our Bluetooth stack, the application needs to buffer the data). So, at 10th connection interval, we have 100 packets accumulated. We are now ready to pump this data. 100 packets take about 70 ms. This is above the application enforced 20ms. So, the hardware can pump data that can fill up 20ms. The remaining data will be deferred to the next connection interval. So, in this case, you would see a CE-length spread over time like this (Per connection interval): 0,0,0,0,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, and so on. (3) We are receiving data at the same rate as in (2). This case is to honor data sent by the other BT-device by giving it more time in the current connection interval. In (2) and (3) you will see non-empty packets either transmitted or received. We can also utilize the CE-length for different reasons: (4) A transaction is in progress, and we are expecting a response packet very soon. In this case, we may be exchanging only empty packets now, and in the next few packet-pairs. In this case, you will the CE-length to be large, and a non-empty packet may not be exchanged in all the slots."]
pub type CONNECTION_EVENT_LENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONNECTION_EVENT_LENGTH` writer - This field defines the length of Connection event. This value is derived from the CE length HCI parameters received from the host. This determines the number of master transmit slots in a connection event, subject to either of the MD bits being set. If both MD bits are set to 0, this has no effect. Units: 625us Note: The connection event length as specified by the CE_LENGTH shall not exceed CONN_INTERVAL - 1.25 ms. The CE-length parameter, according to the Bluetooth specification, is the length of the connection event. Take an example to illustrate this scenario: Assume a connection with interval = 100ms. that the application has put allowed 20ms of CE-length. Here, the CE-length can be upto 100ms (100ms - 150us to be exact). If the connection is maintained for 5 minutes, there could be 10*60*5 = 3000 connection-intervals. The CE-length need not be maintained constant during all the 3000 connection events. Here are the typical cases that determine the value of CE-length: (1) No data packets exchanged. we are just maintaining time and frequency synchronization. In this case, only a packet pair will be exchanged every connection interval. Here, CE-length = 1. (2) Average of 10 packets to be sent per connection event. We can pump data in multiple ways here: 2.1: Send data at uniform rate : In this case, the CE-length will be enough to accommodate 10 packets, which will take about 7ms. As this is less than application enforced limit of 20ms, we can comfortably push all the 10 data packets in this connection interval. So data will be pumped to the other BT device at the same rate as is received from my application. 2.2: Can send data in bursts. Assume that we accumulate data for 1 second and pump out at the end of 1 second(this is not done by our Bluetooth stack, the application needs to buffer the data). So, at 10th connection interval, we have 100 packets accumulated. We are now ready to pump this data. 100 packets take about 70 ms. This is above the application enforced 20ms. So, the hardware can pump data that can fill up 20ms. The remaining data will be deferred to the next connection interval. So, in this case, you would see a CE-length spread over time like this (Per connection interval): 0,0,0,0,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, and so on. (3) We are receiving data at the same rate as in (2). This case is to honor data sent by the other BT-device by giving it more time in the current connection interval. In (2) and (3) you will see non-empty packets either transmitted or received. We can also utilize the CE-length for different reasons: (4) A transaction is in progress, and we are expecting a response packet very soon. In this case, we may be exchanging only empty packets now, and in the next few packet-pairs. In this case, you will the CE-length to be large, and a non-empty packet may not be exchanged in all the slots."]
pub type CONNECTION_EVENT_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CE_LENGTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field defines the length of Connection event. This value is derived from the CE length HCI parameters received from the host. This determines the number of master transmit slots in a connection event, subject to either of the MD bits being set. If both MD bits are set to 0, this has no effect. Units: 625us Note: The connection event length as specified by the CE_LENGTH shall not exceed CONN_INTERVAL - 1.25 ms. The CE-length parameter, according to the Bluetooth specification, is the length of the connection event. Take an example to illustrate this scenario: Assume a connection with interval = 100ms. that the application has put allowed 20ms of CE-length. Here, the CE-length can be upto 100ms (100ms - 150us to be exact). If the connection is maintained for 5 minutes, there could be 10*60*5 = 3000 connection-intervals. The CE-length need not be maintained constant during all the 3000 connection events. Here are the typical cases that determine the value of CE-length: (1) No data packets exchanged. we are just maintaining time and frequency synchronization. In this case, only a packet pair will be exchanged every connection interval. Here, CE-length = 1. (2) Average of 10 packets to be sent per connection event. We can pump data in multiple ways here: 2.1: Send data at uniform rate : In this case, the CE-length will be enough to accommodate 10 packets, which will take about 7ms. As this is less than application enforced limit of 20ms, we can comfortably push all the 10 data packets in this connection interval. So data will be pumped to the other BT device at the same rate as is received from my application. 2.2: Can send data in bursts. Assume that we accumulate data for 1 second and pump out at the end of 1 second(this is not done by our Bluetooth stack, the application needs to buffer the data). So, at 10th connection interval, we have 100 packets accumulated. We are now ready to pump this data. 100 packets take about 70 ms. This is above the application enforced 20ms. So, the hardware can pump data that can fill up 20ms. The remaining data will be deferred to the next connection interval. So, in this case, you would see a CE-length spread over time like this (Per connection interval): 0,0,0,0,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, and so on. (3) We are receiving data at the same rate as in (2). This case is to honor data sent by the other BT-device by giving it more time in the current connection interval. In (2) and (3) you will see non-empty packets either transmitted or received. We can also utilize the CE-length for different reasons: (4) A transaction is in progress, and we are expecting a response packet very soon. In this case, we may be exchanging only empty packets now, and in the next few packet-pairs. In this case, you will the CE-length to be large, and a non-empty packet may not be exchanged in all the slots."]
    #[inline(always)]
    pub fn connection_event_length(&self) -> CONNECTION_EVENT_LENGTH_R {
        CONNECTION_EVENT_LENGTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field defines the length of Connection event. This value is derived from the CE length HCI parameters received from the host. This determines the number of master transmit slots in a connection event, subject to either of the MD bits being set. If both MD bits are set to 0, this has no effect. Units: 625us Note: The connection event length as specified by the CE_LENGTH shall not exceed CONN_INTERVAL - 1.25 ms. The CE-length parameter, according to the Bluetooth specification, is the length of the connection event. Take an example to illustrate this scenario: Assume a connection with interval = 100ms. that the application has put allowed 20ms of CE-length. Here, the CE-length can be upto 100ms (100ms - 150us to be exact). If the connection is maintained for 5 minutes, there could be 10*60*5 = 3000 connection-intervals. The CE-length need not be maintained constant during all the 3000 connection events. Here are the typical cases that determine the value of CE-length: (1) No data packets exchanged. we are just maintaining time and frequency synchronization. In this case, only a packet pair will be exchanged every connection interval. Here, CE-length = 1. (2) Average of 10 packets to be sent per connection event. We can pump data in multiple ways here: 2.1: Send data at uniform rate : In this case, the CE-length will be enough to accommodate 10 packets, which will take about 7ms. As this is less than application enforced limit of 20ms, we can comfortably push all the 10 data packets in this connection interval. So data will be pumped to the other BT device at the same rate as is received from my application. 2.2: Can send data in bursts. Assume that we accumulate data for 1 second and pump out at the end of 1 second(this is not done by our Bluetooth stack, the application needs to buffer the data). So, at 10th connection interval, we have 100 packets accumulated. We are now ready to pump this data. 100 packets take about 70 ms. This is above the application enforced 20ms. So, the hardware can pump data that can fill up 20ms. The remaining data will be deferred to the next connection interval. So, in this case, you would see a CE-length spread over time like this (Per connection interval): 0,0,0,0,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, 20,20,20,10,0,0,0,0,0,0, and so on. (3) We are receiving data at the same rate as in (2). This case is to honor data sent by the other BT-device by giving it more time in the current connection interval. In (2) and (3) you will see non-empty packets either transmitted or received. We can also utilize the CE-length for different reasons: (4) A transaction is in progress, and we are expecting a response packet very soon. In this case, we may be exchanging only empty packets now, and in the next few packet-pairs. In this case, you will the CE-length to be large, and a non-empty packet may not be exchanged in all the slots."]
    #[inline(always)]
    pub fn connection_event_length(&mut self) -> CONNECTION_EVENT_LENGTH_W<0> {
        CONNECTION_EVENT_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection event length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_length](index.html) module"]
pub struct CE_LENGTH_SPEC;
impl crate::RegisterSpec for CE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_length::R](R) reader structure"]
impl crate::Readable for CE_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_length::W](W) writer structure"]
impl crate::Writable for CE_LENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_LENGTH to value 0"]
impl crate::Resettable for CE_LENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
