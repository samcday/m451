#[doc = "Register `GPIO_DBCTL` reader"]
pub struct R(crate::R<GPIO_DBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_DBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_DBCTL_SPEC>> for R {
    fn from(reader: crate::R<GPIO_DBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_DBCTL` writer"]
pub struct W(crate::W<GPIO_DBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_DBCTL_SPEC>;
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
impl core::convert::From<crate::W<GPIO_DBCTL_SPEC>> for W {
    fn from(writer: crate::W<GPIO_DBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "De-bounce Sampling Cycle Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBCLKSEL_A {
    #[doc = "0: Sample interrupt input once per 1 clocks"]
    _0 = 0,
    #[doc = "1: Sample interrupt input once per 2 clocks"]
    _1 = 1,
    #[doc = "2: Sample interrupt input once per 4 clocks"]
    _2 = 2,
    #[doc = "3: Sample interrupt input once per 8 clocks"]
    _3 = 3,
    #[doc = "4: Sample interrupt input once per 16 clocks"]
    _4 = 4,
    #[doc = "5: Sample interrupt input once per 32 clocks"]
    _5 = 5,
    #[doc = "6: Sample interrupt input once per 64 clocks"]
    _6 = 6,
    #[doc = "7: Sample interrupt input once per 128 clocks"]
    _7 = 7,
    #[doc = "8: Sample interrupt input once per 256 clocks"]
    _8 = 8,
    #[doc = "9: Sample interrupt input once per 2*256 clocks"]
    _9 = 9,
    #[doc = "10: Sample interrupt input once per 4*256 clocks"]
    _10 = 10,
    #[doc = "11: Sample interrupt input once per 8*256 clocks"]
    _11 = 11,
    #[doc = "12: Sample interrupt input once per 16*256 clocks"]
    _12 = 12,
    #[doc = "13: Sample interrupt input once per 32*256 clocks"]
    _13 = 13,
    #[doc = "14: Sample interrupt input once per 64*256 clocks"]
    _14 = 14,
    #[doc = "15: Sample interrupt input once per 128*256 clocks"]
    _15 = 15,
}
impl From<DBCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBCLKSEL` reader - De-bounce Sampling Cycle Selection"]
pub struct DBCLKSEL_R(crate::FieldReader<u8, DBCLKSEL_A>);
impl DBCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBCLKSEL_A {
        match self.bits {
            0 => DBCLKSEL_A::_0,
            1 => DBCLKSEL_A::_1,
            2 => DBCLKSEL_A::_2,
            3 => DBCLKSEL_A::_3,
            4 => DBCLKSEL_A::_4,
            5 => DBCLKSEL_A::_5,
            6 => DBCLKSEL_A::_6,
            7 => DBCLKSEL_A::_7,
            8 => DBCLKSEL_A::_8,
            9 => DBCLKSEL_A::_9,
            10 => DBCLKSEL_A::_10,
            11 => DBCLKSEL_A::_11,
            12 => DBCLKSEL_A::_12,
            13 => DBCLKSEL_A::_13,
            14 => DBCLKSEL_A::_14,
            15 => DBCLKSEL_A::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == DBCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == DBCLKSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == DBCLKSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == DBCLKSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == DBCLKSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == DBCLKSEL_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == DBCLKSEL_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        **self == DBCLKSEL_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == DBCLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == DBCLKSEL_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        **self == DBCLKSEL_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == DBCLKSEL_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        **self == DBCLKSEL_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        **self == DBCLKSEL_A::_15
    }
}
impl core::ops::Deref for DBCLKSEL_R {
    type Target = crate::FieldReader<u8, DBCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCLKSEL` writer - De-bounce Sampling Cycle Selection"]
pub struct DBCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sample interrupt input once per 1 clocks"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_0)
    }
    #[doc = "Sample interrupt input once per 2 clocks"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_1)
    }
    #[doc = "Sample interrupt input once per 4 clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_2)
    }
    #[doc = "Sample interrupt input once per 8 clocks"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_3)
    }
    #[doc = "Sample interrupt input once per 16 clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_4)
    }
    #[doc = "Sample interrupt input once per 32 clocks"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_5)
    }
    #[doc = "Sample interrupt input once per 64 clocks"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_6)
    }
    #[doc = "Sample interrupt input once per 128 clocks"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_7)
    }
    #[doc = "Sample interrupt input once per 256 clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_8)
    }
    #[doc = "Sample interrupt input once per 2*256 clocks"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_9)
    }
    #[doc = "Sample interrupt input once per 4*256 clocks"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_10)
    }
    #[doc = "Sample interrupt input once per 8*256 clocks"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_11)
    }
    #[doc = "Sample interrupt input once per 16*256 clocks"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_12)
    }
    #[doc = "Sample interrupt input once per 32*256 clocks"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_13)
    }
    #[doc = "Sample interrupt input once per 64*256 clocks"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_14)
    }
    #[doc = "Sample interrupt input once per 128*256 clocks"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(DBCLKSEL_A::_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "De-bounce Counter Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBCLKSRC_A {
    #[doc = "0: De-bounce counter clock source is the HCLK"]
    _0 = 0,
    #[doc = "1: De-bounce counter clock source is the 10 kHz internal low speed RC oscillator (LIRC)"]
    _1 = 1,
}
impl From<DBCLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: DBCLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBCLKSRC` reader - De-bounce Counter Clock Source Selection"]
pub struct DBCLKSRC_R(crate::FieldReader<bool, DBCLKSRC_A>);
impl DBCLKSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCLKSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBCLKSRC_A {
        match self.bits {
            false => DBCLKSRC_A::_0,
            true => DBCLKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBCLKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBCLKSRC_A::_1
    }
}
impl core::ops::Deref for DBCLKSRC_R {
    type Target = crate::FieldReader<bool, DBCLKSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCLKSRC` writer - De-bounce Counter Clock Source Selection"]
pub struct DBCLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBCLKSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "De-bounce counter clock source is the HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBCLKSRC_A::_0)
    }
    #[doc = "De-bounce counter clock source is the 10 kHz internal low speed RC oscillator (LIRC)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBCLKSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Interrupt Clock on Mode\\nNote: It is recommended to disable this bit to save system power if no special application concern.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICLKON_A {
    #[doc = "0: Edge detection circuit is active only if I/O pin corresponding RHIEN (Px_INTEN\\[n+16\\])/FLIEN (Px_INTEN\\[n\\]) bit is set to 1"]
    _0 = 0,
    #[doc = "1: All I/O pins edge detection circuit is always active after reset"]
    _1 = 1,
}
impl From<ICLKON_A> for bool {
    #[inline(always)]
    fn from(variant: ICLKON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLKON` reader - Interrupt Clock on Mode\\nNote: It is recommended to disable this bit to save system power if no special application concern."]
pub struct ICLKON_R(crate::FieldReader<bool, ICLKON_A>);
impl ICLKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICLKON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICLKON_A {
        match self.bits {
            false => ICLKON_A::_0,
            true => ICLKON_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ICLKON_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ICLKON_A::_1
    }
}
impl core::ops::Deref for ICLKON_R {
    type Target = crate::FieldReader<bool, ICLKON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICLKON` writer - Interrupt Clock on Mode\\nNote: It is recommended to disable this bit to save system power if no special application concern."]
pub struct ICLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> ICLKON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICLKON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge detection circuit is active only if I/O pin corresponding RHIEN (Px_INTEN\\[n+16\\])/FLIEN (Px_INTEN\\[n\\]) bit is set to 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICLKON_A::_0)
    }
    #[doc = "All I/O pins edge detection circuit is always active after reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICLKON_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - De-bounce Sampling Cycle Selection"]
    #[inline(always)]
    pub fn dbclksel(&self) -> DBCLKSEL_R {
        DBCLKSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - De-bounce Counter Clock Source Selection"]
    #[inline(always)]
    pub fn dbclksrc(&self) -> DBCLKSRC_R {
        DBCLKSRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Clock on Mode\\nNote: It is recommended to disable this bit to save system power if no special application concern."]
    #[inline(always)]
    pub fn iclkon(&self) -> ICLKON_R {
        ICLKON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - De-bounce Sampling Cycle Selection"]
    #[inline(always)]
    pub fn dbclksel(&mut self) -> DBCLKSEL_W {
        DBCLKSEL_W { w: self }
    }
    #[doc = "Bit 4 - De-bounce Counter Clock Source Selection"]
    #[inline(always)]
    pub fn dbclksrc(&mut self) -> DBCLKSRC_W {
        DBCLKSRC_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Clock on Mode\\nNote: It is recommended to disable this bit to save system power if no special application concern."]
    #[inline(always)]
    pub fn iclkon(&mut self) -> ICLKON_W {
        ICLKON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt De-bounce Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_dbctl](index.html) module"]
pub struct GPIO_DBCTL_SPEC;
impl crate::RegisterSpec for GPIO_DBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_dbctl::R](R) reader structure"]
impl crate::Readable for GPIO_DBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_dbctl::W](W) writer structure"]
impl crate::Writable for GPIO_DBCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_DBCTL to value 0x20"]
impl crate::Resettable for GPIO_DBCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
