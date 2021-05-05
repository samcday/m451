#[doc = "Register `PC_PIN` reader"]
pub struct R(crate::R<PC_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PC_PIN_SPEC>> for R {
    fn from(reader: crate::R<PC_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIN0` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN0_R(crate::FieldReader<bool, bool>);
impl PIN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN1` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN1_R(crate::FieldReader<bool, bool>);
impl PIN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN2` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN2_R(crate::FieldReader<bool, bool>);
impl PIN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN3` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN3_R(crate::FieldReader<bool, bool>);
impl PIN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN4` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN4_R(crate::FieldReader<bool, bool>);
impl PIN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN5` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN5_R(crate::FieldReader<bool, bool>);
impl PIN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN6` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN6_R(crate::FieldReader<bool, bool>);
impl PIN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN7` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN7_R(crate::FieldReader<bool, bool>);
impl PIN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN8` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN8_R(crate::FieldReader<bool, bool>);
impl PIN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN9` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN9_R(crate::FieldReader<bool, bool>);
impl PIN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN10` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN10_R(crate::FieldReader<bool, bool>);
impl PIN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN11` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN11_R(crate::FieldReader<bool, bool>);
impl PIN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN12` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN12_R(crate::FieldReader<bool, bool>);
impl PIN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN13` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN13_R(crate::FieldReader<bool, bool>);
impl PIN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN14` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN14_R(crate::FieldReader<bool, bool>);
impl PIN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN15` reader - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct PIN15_R(crate::FieldReader<bool, bool>);
impl PIN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Pin Value\\nEach bit of the register reflects the actual status of the respective Px.n pin. If the bit is 1, it indicates the corresponding pin status is high; else the pin status is low.\\nNote1: \\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "PC Pin Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_pin](index.html) module"]
pub struct PC_PIN_SPEC;
impl crate::RegisterSpec for PC_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_pin::R](R) reader structure"]
impl crate::Readable for PC_PIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PC_PIN to value 0"]
impl crate::Resettable for PC_PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
