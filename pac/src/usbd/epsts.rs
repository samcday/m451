#[doc = "Register `EPSTS` reader"]
pub struct R(crate::R<EPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EPSTS_SPEC>> for R {
    fn from(reader: crate::R<EPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Overrun\\nIt indicates that the received data is over the maximum payload number or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OV_A {
    #[doc = "0: No overrun"]
    _0 = 0,
    #[doc = "1: Out Data is more than the Max Payload in MXPLD register or the Setup Data is more than 8 bytes"]
    _1 = 1,
}
impl From<OV_A> for bool {
    #[inline(always)]
    fn from(variant: OV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV` reader - Overrun\\nIt indicates that the received data is over the maximum payload number or not."]
pub struct OV_R(crate::FieldReader<bool, OV_A>);
impl OV_R {
    pub(crate) fn new(bits: bool) -> Self {
        OV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OV_A {
        match self.bits {
            false => OV_A::_0,
            true => OV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == OV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OV_A::_1
    }
}
impl core::ops::Deref for OV_R {
    type Target = crate::FieldReader<bool, OV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 0 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS0_A {
    #[doc = "0: In ACK"]
    IN_ACK = 0,
    #[doc = "1: In NAK"]
    IN_NAK = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    OUT_DATA0_ACK = 2,
    #[doc = "6: Out Packet Data1 ACK"]
    OUT_DATA1_ACK = 6,
    #[doc = "3: Setup ACK"]
    SETUP_ACK = 3,
    #[doc = "7: Isochronous transfer end"]
    ISO_END = 7,
}
impl From<EPSTS0_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS0` reader - Endpoint 0 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS0_R(crate::FieldReader<u8, EPSTS0_A>);
impl EPSTS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS0_A> {
        match self.bits {
            0 => Some(EPSTS0_A::IN_ACK),
            1 => Some(EPSTS0_A::IN_NAK),
            2 => Some(EPSTS0_A::OUT_DATA0_ACK),
            6 => Some(EPSTS0_A::OUT_DATA1_ACK),
            3 => Some(EPSTS0_A::SETUP_ACK),
            7 => Some(EPSTS0_A::ISO_END),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IN_ACK`"]
    #[inline(always)]
    pub fn is_in_ack(&self) -> bool {
        **self == EPSTS0_A::IN_ACK
    }
    #[doc = "Checks if the value of the field is `IN_NAK`"]
    #[inline(always)]
    pub fn is_in_nak(&self) -> bool {
        **self == EPSTS0_A::IN_NAK
    }
    #[doc = "Checks if the value of the field is `OUT_DATA0_ACK`"]
    #[inline(always)]
    pub fn is_out_data0_ack(&self) -> bool {
        **self == EPSTS0_A::OUT_DATA0_ACK
    }
    #[doc = "Checks if the value of the field is `OUT_DATA1_ACK`"]
    #[inline(always)]
    pub fn is_out_data1_ack(&self) -> bool {
        **self == EPSTS0_A::OUT_DATA1_ACK
    }
    #[doc = "Checks if the value of the field is `SETUP_ACK`"]
    #[inline(always)]
    pub fn is_setup_ack(&self) -> bool {
        **self == EPSTS0_A::SETUP_ACK
    }
    #[doc = "Checks if the value of the field is `ISO_END`"]
    #[inline(always)]
    pub fn is_iso_end(&self) -> bool {
        **self == EPSTS0_A::ISO_END
    }
}
impl core::ops::Deref for EPSTS0_R {
    type Target = crate::FieldReader<u8, EPSTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 1 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS1_A = EPSTS0_A;
#[doc = "Field `EPSTS1` reader - Endpoint 1 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS1_R = EPSTS0_R;
#[doc = "Endpoint 2 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS2_A = EPSTS0_A;
#[doc = "Field `EPSTS2` reader - Endpoint 2 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS2_R = EPSTS0_R;
#[doc = "Endpoint 3 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS3_A = EPSTS0_A;
#[doc = "Field `EPSTS3` reader - Endpoint 3 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS3_R = EPSTS0_R;
#[doc = "Endpoint 4 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS4_A = EPSTS0_A;
#[doc = "Field `EPSTS4` reader - Endpoint 4 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS4_R = EPSTS0_R;
#[doc = "Endpoint 5 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS5_A = EPSTS0_A;
#[doc = "Field `EPSTS5` reader - Endpoint 5 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS5_R = EPSTS0_R;
#[doc = "Endpoint 6 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS6_A = EPSTS0_A;
#[doc = "Field `EPSTS6` reader - Endpoint 6 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS6_R = EPSTS0_R;
#[doc = "Endpoint 7 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS7_A = EPSTS0_A;
#[doc = "Field `EPSTS7` reader - Endpoint 7 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub type EPSTS7_R = EPSTS0_R;
impl R {
    #[doc = "Bit 7 - Overrun\\nIt indicates that the received data is over the maximum payload number or not."]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Endpoint 0 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts0(&self) -> EPSTS0_R {
        EPSTS0_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Endpoint 1 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts1(&self) -> EPSTS1_R {
        EPSTS1_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - Endpoint 2 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts2(&self) -> EPSTS2_R {
        EPSTS2_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Endpoint 3 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts3(&self) -> EPSTS3_R {
        EPSTS3_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Endpoint 4 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts4(&self) -> EPSTS4_R {
        EPSTS4_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - Endpoint 5 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts5(&self) -> EPSTS5_R {
        EPSTS5_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Endpoint 6 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts6(&self) -> EPSTS6_R {
        EPSTS6_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Endpoint 7 Status\\nThese bits are used to indicate the current status of this endpoint"]
    #[inline(always)]
    pub fn epsts7(&self) -> EPSTS7_R {
        EPSTS7_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
#[doc = "USB Device Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epsts](index.html) module"]
pub struct EPSTS_SPEC;
impl crate::RegisterSpec for EPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epsts::R](R) reader structure"]
impl crate::Readable for EPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EPSTS to value 0"]
impl crate::Resettable for EPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
