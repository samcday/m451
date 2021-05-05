#[doc = "Register `EADC_DAT3` reader"]
pub struct R(crate::R<EADC_DAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_DAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_DAT3_SPEC>> for R {
    fn from(reader: crate::R<EADC_DAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - A/D Conversion Result\\nThis field contains 12 bits conversion result.\\nWhen DMOF (EADC_CTL\\[9\\]) is set to 0, 12-bit ADC conversion result with unsigned format will be filled in RESULT\\[11:0\\]
and zero will be filled in RESULT\\[15:12\\].\\nWhen DMOF (EADC_CTL\\[9\\]) set to 1, 12-bit ADC conversion result with 2'complement format will be filled in RESULT\\[11:0\\]
and signed bits to will be filled in RESULT\\[15:12\\]."]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Overrun Flag\\nIf converted data in RESULT\\[11:0\\]
has not been read before new conversion result is loaded to this register, OV is set to 1.\\nNote: It is cleared by hardware after EADC_DAT register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OV_A {
    #[doc = "0: Data in RESULT\\[11:0\\]
is recent conversion result"]
    _0 = 0,
    #[doc = "1: Data in RESULT\\[11:0\\]
is overwrite"]
    _1 = 1,
}
impl From<OV_A> for bool {
    #[inline(always)]
    fn from(variant: OV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV` reader - Overrun Flag\\nIf converted data in RESULT\\[11:0\\]
has not been read before new conversion result is loaded to this register, OV is set to 1.\\nNote: It is cleared by hardware after EADC_DAT register is read."]
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
#[doc = "Valid Flag\\nThis bit is set to 1 when corresponding sample module channel analog input conversion is completed and cleared by hardware after EADC_DAT register is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "0: Data in RESULT\\[11:0\\]
bits is not valid"]
    _0 = 0,
    #[doc = "1: Data in RESULT\\[11:0\\]
bits is valid"]
    _1 = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Valid Flag\\nThis bit is set to 1 when corresponding sample module channel analog input conversion is completed and cleared by hardware after EADC_DAT register is read."]
pub struct VALID_R(crate::FieldReader<bool, VALID_A>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::_0,
            true => VALID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == VALID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == VALID_A::_1
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, VALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - A/D Conversion Result\\nThis field contains 12 bits conversion result.\\nWhen DMOF (EADC_CTL\\[9\\]) is set to 0, 12-bit ADC conversion result with unsigned format will be filled in RESULT\\[11:0\\]
and zero will be filled in RESULT\\[15:12\\].\\nWhen DMOF (EADC_CTL\\[9\\]) set to 1, 12-bit ADC conversion result with 2'complement format will be filled in RESULT\\[11:0\\]
and signed bits to will be filled in RESULT\\[15:12\\]."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overrun Flag\\nIf converted data in RESULT\\[11:0\\]
has not been read before new conversion result is loaded to this register, OV is set to 1.\\nNote: It is cleared by hardware after EADC_DAT register is read."]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Valid Flag\\nThis bit is set to 1 when corresponding sample module channel analog input conversion is completed and cleared by hardware after EADC_DAT register is read."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "A/D Data Register 3 for Sample Module 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_dat3](index.html) module"]
pub struct EADC_DAT3_SPEC;
impl crate::RegisterSpec for EADC_DAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_dat3::R](R) reader structure"]
impl crate::Readable for EADC_DAT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EADC_DAT3 to value 0"]
impl crate::Resettable for EADC_DAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
