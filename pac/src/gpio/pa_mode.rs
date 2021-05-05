#[doc = "Register `PA_MODE` reader"]
pub struct R(crate::R<PA_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PA_MODE_SPEC>> for R {
    fn from(reader: crate::R<PA_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PA_MODE` writer"]
pub struct W(crate::W<PA_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_MODE_SPEC>;
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
impl core::convert::From<crate::W<PA_MODE_SPEC>> for W {
    fn from(writer: crate::W<PA_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE0_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE0` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE0_R(crate::FieldReader<u8, MODE0_A>);
impl MODE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::_0,
            1 => MODE0_A::_1,
            2 => MODE0_A::_2,
            3 => MODE0_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE0_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE0_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE0_A::_3
    }
}
impl core::ops::Deref for MODE0_R {
    type Target = crate::FieldReader<u8, MODE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE0` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE0_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE0_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE0_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE0_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE1` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE1_R(crate::FieldReader<u8, MODE1_A>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::_0,
            1 => MODE1_A::_1,
            2 => MODE1_A::_2,
            3 => MODE1_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE1_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE1_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE1_A::_3
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, MODE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE1` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE1_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE1_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE1_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE1_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE2` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE2_R(crate::FieldReader<u8, MODE2_A>);
impl MODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::_0,
            1 => MODE2_A::_1,
            2 => MODE2_A::_2,
            3 => MODE2_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE2_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE2_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE2_A::_3
    }
}
impl core::ops::Deref for MODE2_R {
    type Target = crate::FieldReader<u8, MODE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE2` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE2_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE2_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE2_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE2_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE3_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE3` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE3_R(crate::FieldReader<u8, MODE3_A>);
impl MODE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::_0,
            1 => MODE3_A::_1,
            2 => MODE3_A::_2,
            3 => MODE3_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE3_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE3_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE3_A::_3
    }
}
impl core::ops::Deref for MODE3_R {
    type Target = crate::FieldReader<u8, MODE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE3` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE3_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE3_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE3_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE3_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE4_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE4` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE4_R(crate::FieldReader<u8, MODE4_A>);
impl MODE4_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE4_A {
        match self.bits {
            0 => MODE4_A::_0,
            1 => MODE4_A::_1,
            2 => MODE4_A::_2,
            3 => MODE4_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE4_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE4_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE4_A::_3
    }
}
impl core::ops::Deref for MODE4_R {
    type Target = crate::FieldReader<u8, MODE4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE4` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE4_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE4_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE4_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE4_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE5_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE5` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE5_R(crate::FieldReader<u8, MODE5_A>);
impl MODE5_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE5_A {
        match self.bits {
            0 => MODE5_A::_0,
            1 => MODE5_A::_1,
            2 => MODE5_A::_2,
            3 => MODE5_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE5_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE5_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE5_A::_3
    }
}
impl core::ops::Deref for MODE5_R {
    type Target = crate::FieldReader<u8, MODE5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE5` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE5_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE5_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE5_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE5_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE6_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE6` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE6_R(crate::FieldReader<u8, MODE6_A>);
impl MODE6_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE6_A {
        match self.bits {
            0 => MODE6_A::_0,
            1 => MODE6_A::_1,
            2 => MODE6_A::_2,
            3 => MODE6_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE6_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE6_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE6_A::_3
    }
}
impl core::ops::Deref for MODE6_R {
    type Target = crate::FieldReader<u8, MODE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE6` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE6_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE6_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE6_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE6_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE7_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE7` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE7_R(crate::FieldReader<u8, MODE7_A>);
impl MODE7_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE7_A {
        match self.bits {
            0 => MODE7_A::_0,
            1 => MODE7_A::_1,
            2 => MODE7_A::_2,
            3 => MODE7_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE7_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE7_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE7_A::_3
    }
}
impl core::ops::Deref for MODE7_R {
    type Target = crate::FieldReader<u8, MODE7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE7` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE7_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE7_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE7_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE7_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE8_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE8_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE8` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE8_R(crate::FieldReader<u8, MODE8_A>);
impl MODE8_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            0 => MODE8_A::_0,
            1 => MODE8_A::_1,
            2 => MODE8_A::_2,
            3 => MODE8_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE8_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE8_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE8_A::_3
    }
}
impl core::ops::Deref for MODE8_R {
    type Target = crate::FieldReader<u8, MODE8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE8` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE8_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE8_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE8_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE8_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE9_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE9_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE9` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE9_R(crate::FieldReader<u8, MODE9_A>);
impl MODE9_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9_A {
        match self.bits {
            0 => MODE9_A::_0,
            1 => MODE9_A::_1,
            2 => MODE9_A::_2,
            3 => MODE9_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE9_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE9_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE9_A::_3
    }
}
impl core::ops::Deref for MODE9_R {
    type Target = crate::FieldReader<u8, MODE9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE9` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE9_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE9_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE9_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE9_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE10_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE10_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE10` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE10_R(crate::FieldReader<u8, MODE10_A>);
impl MODE10_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE10_A {
        match self.bits {
            0 => MODE10_A::_0,
            1 => MODE10_A::_1,
            2 => MODE10_A::_2,
            3 => MODE10_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE10_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE10_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE10_A::_3
    }
}
impl core::ops::Deref for MODE10_R {
    type Target = crate::FieldReader<u8, MODE10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE10` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE10_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE10_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE10_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE10_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE11_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE11_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE11` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE11_R(crate::FieldReader<u8, MODE11_A>);
impl MODE11_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE11_A {
        match self.bits {
            0 => MODE11_A::_0,
            1 => MODE11_A::_1,
            2 => MODE11_A::_2,
            3 => MODE11_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE11_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE11_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE11_A::_3
    }
}
impl core::ops::Deref for MODE11_R {
    type Target = crate::FieldReader<u8, MODE11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE11` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE11_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE11_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE11_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE11_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE12_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE12_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE12` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE12_R(crate::FieldReader<u8, MODE12_A>);
impl MODE12_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE12_A {
        match self.bits {
            0 => MODE12_A::_0,
            1 => MODE12_A::_1,
            2 => MODE12_A::_2,
            3 => MODE12_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE12_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE12_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE12_A::_3
    }
}
impl core::ops::Deref for MODE12_R {
    type Target = crate::FieldReader<u8, MODE12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE12` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE12_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE12_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE12_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE12_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE13_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE13_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE13` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE13_R(crate::FieldReader<u8, MODE13_A>);
impl MODE13_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE13_A {
        match self.bits {
            0 => MODE13_A::_0,
            1 => MODE13_A::_1,
            2 => MODE13_A::_2,
            3 => MODE13_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE13_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE13_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE13_A::_3
    }
}
impl core::ops::Deref for MODE13_R {
    type Target = crate::FieldReader<u8, MODE13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE13` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE13_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE13_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE13_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE13_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE14_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE14_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE14` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE14_R(crate::FieldReader<u8, MODE14_A>);
impl MODE14_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE14_A {
        match self.bits {
            0 => MODE14_A::_0,
            1 => MODE14_A::_1,
            2 => MODE14_A::_2,
            3 => MODE14_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE14_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE14_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE14_A::_3
    }
}
impl core::ops::Deref for MODE14_R {
    type Target = crate::FieldReader<u8, MODE14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE14` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE14_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE14_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE14_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE14_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE15_A {
    #[doc = "0: Px.n is in Input mode"]
    _0 = 0,
    #[doc = "1: Px.n is in Push-pull Output mode"]
    _1 = 1,
    #[doc = "2: Px.n is in Open-drain Output mode"]
    _2 = 2,
    #[doc = "3: Px.n is in Quasi-bidirectional mode"]
    _3 = 3,
}
impl From<MODE15_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE15` reader - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE15_R(crate::FieldReader<u8, MODE15_A>);
impl MODE15_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE15_A {
        match self.bits {
            0 => MODE15_A::_0,
            1 => MODE15_A::_1,
            2 => MODE15_A::_2,
            3 => MODE15_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE15_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == MODE15_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == MODE15_A::_3
    }
}
impl core::ops::Deref for MODE15_R {
    type Target = crate::FieldReader<u8, MODE15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE15` writer - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct MODE15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Px.n is in Input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE15_A::_0)
    }
    #[doc = "Px.n is in Push-pull Output mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE15_A::_1)
    }
    #[doc = "Px.n is in Open-drain Output mode"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(MODE15_A::_2)
    }
    #[doc = "Px.n is in Quasi-bidirectional mode"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(MODE15_A::_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bits 2:3 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 4:5 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 6:7 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bits 8:9 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W { w: self }
    }
    #[doc = "Bits 10:11 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W { w: self }
    }
    #[doc = "Bits 12:13 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W { w: self }
    }
    #[doc = "Bits 14:15 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W { w: self }
    }
    #[doc = "Bits 16:17 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W { w: self }
    }
    #[doc = "Bits 18:19 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bits 20:21 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W { w: self }
    }
    #[doc = "Bits 22:23 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W { w: self }
    }
    #[doc = "Bits 24:25 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W { w: self }
    }
    #[doc = "Bits 26:27 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W { w: self }
    }
    #[doc = "Bits 28:29 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W { w: self }
    }
    #[doc = "Bits 30:31 - Port A-f I/O Pin\\[N\\]
Mode Control\\nDetermine each I/O mode of Px.n pins.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PA I/O Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_mode](index.html) module"]
pub struct PA_MODE_SPEC;
impl crate::RegisterSpec for PA_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_mode::R](R) reader structure"]
impl crate::Readable for PA_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_mode::W](W) writer structure"]
impl crate::Writable for PA_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PA_MODE to value 0"]
impl crate::Resettable for PA_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
