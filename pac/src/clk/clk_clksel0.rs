#[doc = "Register `CLK_CLKSEL0` reader"]
pub struct R(crate::R<CLK_CLKSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CLKSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CLKSEL0_SPEC>> for R {
    fn from(reader: crate::R<CLK_CLKSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CLKSEL0` writer"]
pub struct W(crate::W<CLK_CLKSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CLKSEL0_SPEC>;
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
impl core::convert::From<crate::W<CLK_CLKSEL0_SPEC>> for W {
    fn from(writer: crate::W<CLK_CLKSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HCLK Clock Source Selection (Write Protect)\\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\\nThe default value is reloaded from the value of CFOSC (CONFIG0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HCLKSEL_A {
    #[doc = "0: Clock source from HXT"]
    _0 = 0,
    #[doc = "1: Clock source from LXT"]
    _1 = 1,
    #[doc = "2: Clock source from PLL"]
    _2 = 2,
    #[doc = "3: Clock source from LIRC"]
    _3 = 3,
    #[doc = "7: Clock source from HIRC"]
    _7 = 7,
}
impl From<HCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HCLKSEL` reader - HCLK Clock Source Selection (Write Protect)\\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\\nThe default value is reloaded from the value of CFOSC (CONFIG0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HCLKSEL_R(crate::FieldReader<u8, HCLKSEL_A>);
impl HCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HCLKSEL_A> {
        match self.bits {
            0 => Some(HCLKSEL_A::_0),
            1 => Some(HCLKSEL_A::_1),
            2 => Some(HCLKSEL_A::_2),
            3 => Some(HCLKSEL_A::_3),
            7 => Some(HCLKSEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == HCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == HCLKSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == HCLKSEL_A::_7
    }
}
impl core::ops::Deref for HCLKSEL_R {
    type Target = crate::FieldReader<u8, HCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLKSEL` writer - HCLK Clock Source Selection (Write Protect)\\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\\nThe default value is reloaded from the value of CFOSC (CONFIG0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct HCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from HXT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_0)
    }
    #[doc = "Clock source from LXT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_1)
    }
    #[doc = "Clock source from PLL"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_2)
    }
    #[doc = "Clock source from LIRC"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_3)
    }
    #[doc = "Clock source from HIRC"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(HCLKSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Cortex-M4 SysTick Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STCLKSEL_A {
    #[doc = "0: Clock source from HXT"]
    _0 = 0,
    #[doc = "1: Clock source from LXT"]
    _1 = 1,
    #[doc = "2: Clock source from HXT/2"]
    _2 = 2,
    #[doc = "3: Clock source from HCLK/2"]
    _3 = 3,
    #[doc = "7: Clock source from HIRC/2"]
    _7 = 7,
}
impl From<STCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STCLKSEL` reader - Cortex-M4 SysTick Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct STCLKSEL_R(crate::FieldReader<u8, STCLKSEL_A>);
impl STCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        STCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STCLKSEL_A> {
        match self.bits {
            0 => Some(STCLKSEL_A::_0),
            1 => Some(STCLKSEL_A::_1),
            2 => Some(STCLKSEL_A::_2),
            3 => Some(STCLKSEL_A::_3),
            7 => Some(STCLKSEL_A::_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STCLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STCLKSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == STCLKSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == STCLKSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == STCLKSEL_A::_7
    }
}
impl core::ops::Deref for STCLKSEL_R {
    type Target = crate::FieldReader<u8, STCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STCLKSEL` writer - Cortex-M4 SysTick Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct STCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source from HXT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_0)
    }
    #[doc = "Clock source from LXT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_1)
    }
    #[doc = "Clock source from HXT/2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_2)
    }
    #[doc = "Clock source from HCLK/2"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_3)
    }
    #[doc = "Clock source from HIRC/2"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(STCLKSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "PCLK0 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK0SEL_A {
    #[doc = "0: APB0 bus clock source from HCLK"]
    _0 = 0,
    #[doc = "1: APB0 bus clock source from HCLK/2"]
    _1 = 1,
}
impl From<PCLK0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCLK0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLK0SEL` reader - PCLK0 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PCLK0SEL_R(crate::FieldReader<bool, PCLK0SEL_A>);
impl PCLK0SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK0SEL_A {
        match self.bits {
            false => PCLK0SEL_A::_0,
            true => PCLK0SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCLK0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCLK0SEL_A::_1
    }
}
impl core::ops::Deref for PCLK0SEL_R {
    type Target = crate::FieldReader<bool, PCLK0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK0SEL` writer - PCLK0 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PCLK0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK0SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "APB0 bus clock source from HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCLK0SEL_A::_0)
    }
    #[doc = "APB0 bus clock source from HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCLK0SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PCLK1 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCLK1SEL_A {
    #[doc = "0: APB1 bus clock source from HCLK"]
    _0 = 0,
    #[doc = "1: APB1 bus clock source from HCLK/2"]
    _1 = 1,
}
impl From<PCLK1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PCLK1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLK1SEL` reader - PCLK1 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PCLK1SEL_R(crate::FieldReader<bool, PCLK1SEL_A>);
impl PCLK1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCLK1SEL_A {
        match self.bits {
            false => PCLK1SEL_A::_0,
            true => PCLK1SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCLK1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCLK1SEL_A::_1
    }
}
impl core::ops::Deref for PCLK1SEL_R {
    type Target = crate::FieldReader<bool, PCLK1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK1SEL` writer - PCLK1 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct PCLK1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCLK1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "APB1 bus clock source from HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCLK1SEL_A::_0)
    }
    #[doc = "APB1 bus clock source from HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCLK1SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HCLK Clock Source Selection (Write Protect)\\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\\nThe default value is reloaded from the value of CFOSC (CONFIG0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hclksel(&self) -> HCLKSEL_R {
        HCLKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Cortex-M4 SysTick Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn stclksel(&self) -> STCLKSEL_R {
        STCLKSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - PCLK0 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pclk0sel(&self) -> PCLK0SEL_R {
        PCLK0SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PCLK1 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pclk1sel(&self) -> PCLK1SEL_R {
        PCLK1SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - HCLK Clock Source Selection (Write Protect)\\nBefore clock switching, the related clock sources (both pre-select and new-select) must be turned on.\\nThe default value is reloaded from the value of CFOSC (CONFIG0\\[26:24\\]) in user configuration register of Flash controller by any reset. Therefore the default value is either 000b or 111b.\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn hclksel(&mut self) -> HCLKSEL_W {
        HCLKSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Cortex-M4 SysTick Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn stclksel(&mut self) -> STCLKSEL_W {
        STCLKSEL_W { w: self }
    }
    #[doc = "Bit 6 - PCLK0 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pclk0sel(&mut self) -> PCLK0SEL_W {
        PCLK0SEL_W { w: self }
    }
    #[doc = "Bit 7 - PCLK1 Clock Source Selection (Write Protect)\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn pclk1sel(&mut self) -> PCLK1SEL_W {
        PCLK1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Source Select Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_clksel0](index.html) module"]
pub struct CLK_CLKSEL0_SPEC;
impl crate::RegisterSpec for CLK_CLKSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_clksel0::R](R) reader structure"]
impl crate::Readable for CLK_CLKSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_clksel0::W](W) writer structure"]
impl crate::Writable for CLK_CLKSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CLKSEL0 to value 0x30"]
impl crate::Resettable for CLK_CLKSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
