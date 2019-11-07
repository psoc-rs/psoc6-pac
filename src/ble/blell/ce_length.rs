#[doc = "Reader of register CE_LENGTH"]
pub type R = crate::R<u32, super::CE_LENGTH>;
#[doc = "Writer for register CE_LENGTH"]
pub type W = crate::W<u32, super::CE_LENGTH>;
#[doc = "Register CE_LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::CE_LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONNECTION_EVENT_LENGTH`"]
pub type CONNECTION_EVENT_LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONNECTION_EVENT_LENGTH`"]
pub struct CONNECTION_EVENT_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECTION_EVENT_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
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
    pub fn connection_event_length(&mut self) -> CONNECTION_EVENT_LENGTH_W {
        CONNECTION_EVENT_LENGTH_W { w: self }
    }
}
