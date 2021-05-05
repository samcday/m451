#[doc = "Register `WWDT_CTL` reader"]
pub struct R(crate::R<WWDT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WWDT_CTL_SPEC>> for R {
    fn from(reader: crate::R<WWDT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDT_CTL` writer"]
pub struct W(crate::W<WWDT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDT_CTL_SPEC>;
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
impl core::convert::From<crate::W<WWDT_CTL_SPEC>> for W {
    fn from(writer: crate::W<WWDT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WWDT Enable Control Bit\\nSet this bit to enable WWDT counter counting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDTEN_A {
    #[doc = "0: WWDT counter is stopped"]
    _0 = 0,
    #[doc = "1: WWDT counter is starting counting"]
    _1 = 1,
}
impl From<WWDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WWDTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTEN` reader - WWDT Enable Control Bit\\nSet this bit to enable WWDT counter counting."]
pub struct WWDTEN_R(crate::FieldReader<bool, WWDTEN_A>);
impl WWDTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDTEN_A {
        match self.bits {
            false => WWDTEN_A::_0,
            true => WWDTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WWDTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WWDTEN_A::_1
    }
}
impl core::ops::Deref for WWDTEN_R {
    type Target = crate::FieldReader<bool, WWDTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDTEN` writer - WWDT Enable Control Bit\\nSet this bit to enable WWDT counter counting."]
pub struct WWDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WWDT counter is stopped"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WWDTEN_A::_0)
    }
    #[doc = "WWDT counter is starting counting"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WWDTEN_A::_1)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "WWDT Interrupt Enable Control Bit\\nIf this bit is enabled, the WWDT counter compare match interrupt signal is generated and inform to CPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: WWDT counter compare match interrupt Disabled"]
    _0 = 0,
    #[doc = "1: WWDT counter compare match interrupt Enabled"]
    _1 = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - WWDT Interrupt Enable Control Bit\\nIf this bit is enabled, the WWDT counter compare match interrupt signal is generated and inform to CPU."]
pub struct INTEN_R(crate::FieldReader<bool, INTEN_A>);
impl INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::_0,
            true => INTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INTEN_A::_1
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<bool, INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - WWDT Interrupt Enable Control Bit\\nIf this bit is enabled, the WWDT counter compare match interrupt signal is generated and inform to CPU."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WWDT counter compare match interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTEN_A::_0)
    }
    #[doc = "WWDT counter compare match interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTEN_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "WWDT Counter Prescale Period Selection\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSCSEL_A {
    #[doc = "0: Pre-scale is 1; Max time-out period is 1 * 64 * WWDT_CLK"]
    _0 = 0,
    #[doc = "1: Pre-scale is 2; Max time-out period is 2 * 64 * WWDT_CLK"]
    _1 = 1,
    #[doc = "2: Pre-scale is 4; Max time-out period is 4 * 64 * WWDT_CLK"]
    _2 = 2,
    #[doc = "3: Pre-scale is 8; Max time-out period is 8 * 64 * WWDT_CLK"]
    _3 = 3,
    #[doc = "4: Pre-scale is 16; Max time-out period is 16 * 64 * WWDT_CLK"]
    _4 = 4,
    #[doc = "5: Pre-scale is 32; Max time-out period is 32 * 64 * WWDT_CLK"]
    _5 = 5,
    #[doc = "6: Pre-scale is 64; Max time-out period is 64 * 64 * WWDT_CLK"]
    _6 = 6,
    #[doc = "7: Pre-scale is 128; Max time-out period is 128 * 64 * WWDT_CLK"]
    _7 = 7,
    #[doc = "8: Pre-scale is 192; Max time-out period is 192 * 64 * WWDT_CLK"]
    _8 = 8,
    #[doc = "9: Pre-scale is 256; Max time-out period is 256 * 64 * WWDT_CLK"]
    _9 = 9,
    #[doc = "10: Pre-scale is 384; Max time-out period is 384 * 64 * WWDT_CLK"]
    _10 = 10,
    #[doc = "11: Pre-scale is 512; Max time-out period is 512 * 64 * WWDT_CLK"]
    _11 = 11,
    #[doc = "12: Pre-scale is 768; Max time-out period is 768 * 64 * WWDT_CLK"]
    _12 = 12,
    #[doc = "13: Pre-scale is 1024; Max time-out period is 1024 * 64 * WWDT_CLK"]
    _13 = 13,
    #[doc = "14: Pre-scale is 1536; Max time-out period is 1536 * 64 * WWDT_CLK"]
    _14 = 14,
    #[doc = "15: Pre-scale is 2048; Max time-out period is 2048 * 64 * WWDT_CLK"]
    _15 = 15,
}
impl From<PSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSCSEL` reader - WWDT Counter Prescale Period Selection"]
pub struct PSCSEL_R(crate::FieldReader<u8, PSCSEL_A>);
impl PSCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSCSEL_A {
        match self.bits {
            0 => PSCSEL_A::_0,
            1 => PSCSEL_A::_1,
            2 => PSCSEL_A::_2,
            3 => PSCSEL_A::_3,
            4 => PSCSEL_A::_4,
            5 => PSCSEL_A::_5,
            6 => PSCSEL_A::_6,
            7 => PSCSEL_A::_7,
            8 => PSCSEL_A::_8,
            9 => PSCSEL_A::_9,
            10 => PSCSEL_A::_10,
            11 => PSCSEL_A::_11,
            12 => PSCSEL_A::_12,
            13 => PSCSEL_A::_13,
            14 => PSCSEL_A::_14,
            15 => PSCSEL_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PSCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PSCSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == PSCSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == PSCSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == PSCSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == PSCSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == PSCSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == PSCSEL_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == PSCSEL_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == PSCSEL_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PSCSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PSCSEL_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == PSCSEL_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == PSCSEL_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == PSCSEL_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == PSCSEL_A::_15
    }
}
impl core::ops::Deref for PSCSEL_R {
    type Target = crate::FieldReader<u8, PSCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSCSEL` writer - WWDT Counter Prescale Period Selection"]
pub struct PSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pre-scale is 1; Max time-out period is 1 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSCSEL_A::_0)
    }
    #[doc = "Pre-scale is 2; Max time-out period is 2 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSCSEL_A::_1)
    }
    #[doc = "Pre-scale is 4; Max time-out period is 4 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(PSCSEL_A::_2)
    }
    #[doc = "Pre-scale is 8; Max time-out period is 8 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(PSCSEL_A::_3)
    }
    #[doc = "Pre-scale is 16; Max time-out period is 16 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(PSCSEL_A::_4)
    }
    #[doc = "Pre-scale is 32; Max time-out period is 32 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PSCSEL_A::_5)
    }
    #[doc = "Pre-scale is 64; Max time-out period is 64 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(PSCSEL_A::_6)
    }
    #[doc = "Pre-scale is 128; Max time-out period is 128 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PSCSEL_A::_7)
    }
    #[doc = "Pre-scale is 192; Max time-out period is 192 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSCSEL_A::_8)
    }
    #[doc = "Pre-scale is 256; Max time-out period is 256 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(PSCSEL_A::_9)
    }
    #[doc = "Pre-scale is 384; Max time-out period is 384 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PSCSEL_A::_10)
    }
    #[doc = "Pre-scale is 512; Max time-out period is 512 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PSCSEL_A::_11)
    }
    #[doc = "Pre-scale is 768; Max time-out period is 768 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(PSCSEL_A::_12)
    }
    #[doc = "Pre-scale is 1024; Max time-out period is 1024 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(PSCSEL_A::_13)
    }
    #[doc = "Pre-scale is 1536; Max time-out period is 1536 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(PSCSEL_A::_14)
    }
    #[doc = "Pre-scale is 2048; Max time-out period is 2048 * 64 * WWDT_CLK"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(PSCSEL_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CMPDAT` reader - WWDT Window Compare Register\\nSet this register to adjust the valid reload window. \\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT. If user writes WWDT_RLDCNT register when current WWDT counter value larger than CMPDAT, WWDT reset signal will generate immediately."]
pub struct CMPDAT_R(crate::FieldReader<u8, u8>);
impl CMPDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDAT` writer - WWDT Window Compare Register\\nSet this register to adjust the valid reload window. \\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT. If user writes WWDT_RLDCNT register when current WWDT counter value larger than CMPDAT, WWDT reset signal will generate immediately."]
pub struct CMPDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "ICE Debug Mode Acknowledge Disable Control\\nWWDT down counter will keep going no matter CPU is held by ICE or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEDEBUG_A {
    #[doc = "0: ICE debug mode acknowledgement effects WWDT counting"]
    _0 = 0,
    #[doc = "1: ICE debug mode acknowledgement Disabled"]
    _1 = 1,
}
impl From<ICEDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: ICEDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEDEBUG` reader - ICE Debug Mode Acknowledge Disable Control\\nWWDT down counter will keep going no matter CPU is held by ICE or not."]
pub struct ICEDEBUG_R(crate::FieldReader<bool, ICEDEBUG_A>);
impl ICEDEBUG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEDEBUG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEDEBUG_A {
        match self.bits {
            false => ICEDEBUG_A::_0,
            true => ICEDEBUG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICEDEBUG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICEDEBUG_A::_1
    }
}
impl core::ops::Deref for ICEDEBUG_R {
    type Target = crate::FieldReader<bool, ICEDEBUG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEDEBUG` writer - ICE Debug Mode Acknowledge Disable Control\\nWWDT down counter will keep going no matter CPU is held by ICE or not."]
pub struct ICEDEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEDEBUG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICEDEBUG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ICE debug mode acknowledgement effects WWDT counting"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_0)
    }
    #[doc = "ICE debug mode acknowledgement Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICEDEBUG_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WWDT Enable Control Bit\\nSet this bit to enable WWDT counter counting."]
    #[inline(always)]
    pub fn wwdten(&self) -> WWDTEN_R {
        WWDTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WWDT Interrupt Enable Control Bit\\nIf this bit is enabled, the WWDT counter compare match interrupt signal is generated and inform to CPU."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - WWDT Counter Prescale Period Selection"]
    #[inline(always)]
    pub fn pscsel(&self) -> PSCSEL_R {
        PSCSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - WWDT Window Compare Register\\nSet this register to adjust the valid reload window. \\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT. If user writes WWDT_RLDCNT register when current WWDT counter value larger than CMPDAT, WWDT reset signal will generate immediately."]
    #[inline(always)]
    pub fn cmpdat(&self) -> CMPDAT_R {
        CMPDAT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Control\\nWWDT down counter will keep going no matter CPU is held by ICE or not."]
    #[inline(always)]
    pub fn icedebug(&self) -> ICEDEBUG_R {
        ICEDEBUG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDT Enable Control Bit\\nSet this bit to enable WWDT counter counting."]
    #[inline(always)]
    pub fn wwdten(&mut self) -> WWDTEN_W {
        WWDTEN_W { w: self }
    }
    #[doc = "Bit 1 - WWDT Interrupt Enable Control Bit\\nIf this bit is enabled, the WWDT counter compare match interrupt signal is generated and inform to CPU."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - WWDT Counter Prescale Period Selection"]
    #[inline(always)]
    pub fn pscsel(&mut self) -> PSCSEL_W {
        PSCSEL_W { w: self }
    }
    #[doc = "Bits 16:21 - WWDT Window Compare Register\\nSet this register to adjust the valid reload window. \\nNote: User can only write WWDT_RLDCNT register to reload WWDT counter value when current WWDT counter value between 0 and CMPDAT. If user writes WWDT_RLDCNT register when current WWDT counter value larger than CMPDAT, WWDT reset signal will generate immediately."]
    #[inline(always)]
    pub fn cmpdat(&mut self) -> CMPDAT_W {
        CMPDAT_W { w: self }
    }
    #[doc = "Bit 31 - ICE Debug Mode Acknowledge Disable Control\\nWWDT down counter will keep going no matter CPU is held by ICE or not."]
    #[inline(always)]
    pub fn icedebug(&mut self) -> ICEDEBUG_W {
        ICEDEBUG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdt_ctl](index.html) module"]
pub struct WWDT_CTL_SPEC;
impl crate::RegisterSpec for WWDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdt_ctl::R](R) reader structure"]
impl crate::Readable for WWDT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdt_ctl::W](W) writer structure"]
impl crate::Writable for WWDT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDT_CTL to value 0x003f_0800"]
impl crate::Resettable for WWDT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_0800
    }
}
