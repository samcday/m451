#[doc = "Register `CAN_IIDR` reader"]
pub struct R(crate::R<CAN_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_IIDR_SPEC>> for R {
    fn from(reader: crate::R<CAN_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IntId` reader - Interrupt Identifier (Indicates the Source of the Interrupt)\\nIf several interrupts are pending, the CAN Interrupt Register will point to the pending interrupt with the highest priority, disregarding their chronological order. An interrupt remains pending until the application software has cleared it. If IntId is different from 0x0000 and IE (CAN_CON\\[1\\]) is set, the IRQ interrupt signal to the EIC is active. The interrupt remains active until IntId is back to value 0x0000 (the cause of the interrupt is reset) or until IE is reset.\\nThe Status Interrupt has the highest priority. Among the message interrupts, the Message Object' s interrupt priority decreases with increasing message number.\\nA message interrupt is cleared by clearing the Message Object's IntPnd bit (CAN_IFn_MCON\\[13\\]). The Status Interrupt is cleared by reading the Status Register."]
pub struct INTID_R(crate::FieldReader<u16, u16>);
impl INTID_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt Identifier (Indicates the Source of the Interrupt)\\nIf several interrupts are pending, the CAN Interrupt Register will point to the pending interrupt with the highest priority, disregarding their chronological order. An interrupt remains pending until the application software has cleared it. If IntId is different from 0x0000 and IE (CAN_CON\\[1\\]) is set, the IRQ interrupt signal to the EIC is active. The interrupt remains active until IntId is back to value 0x0000 (the cause of the interrupt is reset) or until IE is reset.\\nThe Status Interrupt has the highest priority. Among the message interrupts, the Message Object' s interrupt priority decreases with increasing message number.\\nA message interrupt is cleared by clearing the Message Object's IntPnd bit (CAN_IFn_MCON\\[13\\]). The Status Interrupt is cleared by reading the Status Register."]
    #[inline(always)]
    pub fn int_id(&self) -> INTID_R {
        INTID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Interrupt Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_iidr](index.html) module"]
pub struct CAN_IIDR_SPEC;
impl crate::RegisterSpec for CAN_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_iidr::R](R) reader structure"]
impl crate::Readable for CAN_IIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAN_IIDR to value 0"]
impl crate::Resettable for CAN_IIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
