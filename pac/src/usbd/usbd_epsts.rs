#[doc = "Register `USBD_EPSTS` reader"]
pub struct R(crate::R<USBD_EPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBD_EPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBD_EPSTS_SPEC>> for R {
    fn from(reader: crate::R<USBD_EPSTS_SPEC>) -> Self {
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
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
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
            0 => Some(EPSTS0_A::_0),
            1 => Some(EPSTS0_A::_1),
            2 => Some(EPSTS0_A::_2),
            3 => Some(EPSTS0_A::_3),
            6 => Some(EPSTS0_A::_6),
            7 => Some(EPSTS0_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS0_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS0_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS0_A::_7
    }
}
impl core::ops::Deref for EPSTS0_R {
    type Target = crate::FieldReader<u8, EPSTS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 1 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS1_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS1_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS1` reader - Endpoint 1 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS1_R(crate::FieldReader<u8, EPSTS1_A>);
impl EPSTS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS1_A> {
        match self.bits {
            0 => Some(EPSTS1_A::_0),
            1 => Some(EPSTS1_A::_1),
            2 => Some(EPSTS1_A::_2),
            3 => Some(EPSTS1_A::_3),
            6 => Some(EPSTS1_A::_6),
            7 => Some(EPSTS1_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS1_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS1_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS1_A::_7
    }
}
impl core::ops::Deref for EPSTS1_R {
    type Target = crate::FieldReader<u8, EPSTS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 2 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS2_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS2_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS2` reader - Endpoint 2 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS2_R(crate::FieldReader<u8, EPSTS2_A>);
impl EPSTS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS2_A> {
        match self.bits {
            0 => Some(EPSTS2_A::_0),
            1 => Some(EPSTS2_A::_1),
            2 => Some(EPSTS2_A::_2),
            3 => Some(EPSTS2_A::_3),
            6 => Some(EPSTS2_A::_6),
            7 => Some(EPSTS2_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS2_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS2_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS2_A::_7
    }
}
impl core::ops::Deref for EPSTS2_R {
    type Target = crate::FieldReader<u8, EPSTS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 3 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS3_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS3_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS3` reader - Endpoint 3 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS3_R(crate::FieldReader<u8, EPSTS3_A>);
impl EPSTS3_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS3_A> {
        match self.bits {
            0 => Some(EPSTS3_A::_0),
            1 => Some(EPSTS3_A::_1),
            2 => Some(EPSTS3_A::_2),
            3 => Some(EPSTS3_A::_3),
            6 => Some(EPSTS3_A::_6),
            7 => Some(EPSTS3_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS3_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS3_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS3_A::_7
    }
}
impl core::ops::Deref for EPSTS3_R {
    type Target = crate::FieldReader<u8, EPSTS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 4 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS4_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS4_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS4` reader - Endpoint 4 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS4_R(crate::FieldReader<u8, EPSTS4_A>);
impl EPSTS4_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS4_A> {
        match self.bits {
            0 => Some(EPSTS4_A::_0),
            1 => Some(EPSTS4_A::_1),
            2 => Some(EPSTS4_A::_2),
            3 => Some(EPSTS4_A::_3),
            6 => Some(EPSTS4_A::_6),
            7 => Some(EPSTS4_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS4_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS4_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS4_A::_7
    }
}
impl core::ops::Deref for EPSTS4_R {
    type Target = crate::FieldReader<u8, EPSTS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 5 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS5_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS5_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS5` reader - Endpoint 5 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS5_R(crate::FieldReader<u8, EPSTS5_A>);
impl EPSTS5_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS5_A> {
        match self.bits {
            0 => Some(EPSTS5_A::_0),
            1 => Some(EPSTS5_A::_1),
            2 => Some(EPSTS5_A::_2),
            3 => Some(EPSTS5_A::_3),
            6 => Some(EPSTS5_A::_6),
            7 => Some(EPSTS5_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS5_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS5_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS5_A::_7
    }
}
impl core::ops::Deref for EPSTS5_R {
    type Target = crate::FieldReader<u8, EPSTS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 6 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS6_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS6_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS6` reader - Endpoint 6 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS6_R(crate::FieldReader<u8, EPSTS6_A>);
impl EPSTS6_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS6_A> {
        match self.bits {
            0 => Some(EPSTS6_A::_0),
            1 => Some(EPSTS6_A::_1),
            2 => Some(EPSTS6_A::_2),
            3 => Some(EPSTS6_A::_3),
            6 => Some(EPSTS6_A::_6),
            7 => Some(EPSTS6_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS6_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS6_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS6_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS6_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS6_A::_7
    }
}
impl core::ops::Deref for EPSTS6_R {
    type Target = crate::FieldReader<u8, EPSTS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Endpoint 7 Status\\nThese bits are used to indicate the current status of this endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPSTS7_A {
    #[doc = "0: In ACK"]
    _0 = 0,
    #[doc = "1: In NAK"]
    _1 = 1,
    #[doc = "2: Out Packet Data0 ACK"]
    _2 = 2,
    #[doc = "3: Setup ACK"]
    _3 = 3,
    #[doc = "6: Out Packet Data1 ACK"]
    _6 = 6,
    #[doc = "7: Isochronous transfer end"]
    _7 = 7,
}
impl From<EPSTS7_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSTS7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPSTS7` reader - Endpoint 7 Status\\nThese bits are used to indicate the current status of this endpoint"]
pub struct EPSTS7_R(crate::FieldReader<u8, EPSTS7_A>);
impl EPSTS7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPSTS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPSTS7_A> {
        match self.bits {
            0 => Some(EPSTS7_A::_0),
            1 => Some(EPSTS7_A::_1),
            2 => Some(EPSTS7_A::_2),
            3 => Some(EPSTS7_A::_3),
            6 => Some(EPSTS7_A::_6),
            7 => Some(EPSTS7_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EPSTS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EPSTS7_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == EPSTS7_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == EPSTS7_A::_3
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == EPSTS7_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == EPSTS7_A::_7
    }
}
impl core::ops::Deref for EPSTS7_R {
    type Target = crate::FieldReader<u8, EPSTS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "USB Device Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbd_epsts](index.html) module"]
pub struct USBD_EPSTS_SPEC;
impl crate::RegisterSpec for USBD_EPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbd_epsts::R](R) reader structure"]
impl crate::Readable for USBD_EPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBD_EPSTS to value 0"]
impl crate::Resettable for USBD_EPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
