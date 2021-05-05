#[doc = "Register `SC_INTSTS` reader"]
pub struct R(crate::R<SC_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SC_INTSTS_SPEC>> for R {
    fn from(reader: crate::R<SC_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC_INTSTS` writer"]
pub struct W(crate::W<SC_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_INTSTS_SPEC>;
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
impl core::convert::From<crate::W<SC_INTSTS_SPEC>> for W {
    fn from(writer: crate::W<SC_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDAIF` reader - Receive Data Reach Interrupt Status Flag (Read Only)\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt status flag.\\nNote: This field is the status flag of received data reaching RXTRGLV (SC_CTL\\[7:6\\]). If software reads data from SC_DAT and receiver buffer data byte number is less than RXTRGLV (SC_CTL\\[7:6\\]), this bit will be cleared automatically."]
pub struct RDAIF_R(crate::FieldReader<bool, bool>);
impl RDAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBEIF` reader - Transmit Buffer Empty Interrupt Status Flag (Read Only)\\nThis field is used for transmit buffer empty interrupt status flag.\\nNote: This field is the status flag of transmit buffer empty state. If software wants to clear this bit, software must write data to DAT(SC_DAT\\[7:0\\]) buffer and then this bit will be cleared automatically."]
pub struct TBEIF_R(crate::FieldReader<bool, bool>);
impl TBEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERRIF` reader - Transfer Error Interrupt Status Flag (Read Only)\\nThis field is used for transfer error interrupt status flag. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\], parity error PEF(SC_STATUS\\[4\\]
and receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]
and transmitter retry over limit error TXOVERR(SC_STATUS\\[30\\]).\\nNote: This field is the status flag of BEF(SC_STATUS\\[6\\]), FEF(SC_STATUS\\[5\\]), PEF(SC_STATUS\\[4\\]), RXOV(SC_STATUS\\[0\\]), TXOV(SC_STATUS\\[8\\]), RXOVERR(SC_STATUS\\[22\\]) or TXOVERR(SC_STATUS\\[30\\]). So, if software wants to clear this bit, software must write 1 to each field."]
pub struct TERRIF_R(crate::FieldReader<bool, bool>);
impl TERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR0IF` reader - Timer0 Interrupt Status Flag (Read Only)\\nThis field is used for TMR0 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct TMR0IF_R(crate::FieldReader<bool, bool>);
impl TMR0IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR0IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR0IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR1IF` reader - Timer1 Interrupt Status Flag (Read Only)\\nThis field is used for TMR1 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct TMR1IF_R(crate::FieldReader<bool, bool>);
impl TMR1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2IF` reader - Timer2 Interrupt Status Flag (Read Only)\\nThis field is used for TMR2 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct TMR2IF_R(crate::FieldReader<bool, bool>);
impl TMR2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGTIF` reader - "]
pub struct BGTIF_R(crate::FieldReader<bool, bool>);
impl BGTIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGTIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGTIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDIF` reader - Card Detect Interrupt Status Flag (Read Only)\\nThis field is used for card detect interrupt status flag. The card detect status is CINSERT (SC_STATUS\\[12\\]) and CREMOVE(SC_STATUS\\[11\\]).\\nNote: This field is the status flag of CINSERT(SC_STATUS\\[12\\]) or CREMOVE(SC_STATUS\\[11\\])\\]. So if software wants to clear this bit, software must write 1 to this field."]
pub struct CDIF_R(crate::FieldReader<bool, bool>);
impl CDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITIF` reader - Initial End Interrupt Status Flag (Read Only)\\nThis field is used for activation (ACTEN(SC_ALTCTL\\[3\\])), deactivation (DACTEN (SC_ALTCTL\\[2\\])) and warm reset (WARSTEN (SC_ALTCTL\\[4\\])) sequence interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct INITIF_R(crate::FieldReader<bool, bool>);
impl INITIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBTOIF` reader - Receiver Buffer Time-out Interrupt Status Flag (Read Only)\\nThis field is used for receiver buffer time-out interrupt status flag.\\nNote: This field is the status flag of receiver buffer time-out state. If software wants to clear this bit, software must read all receiver buffer remaining data by reading SC_DAT buffer,"]
pub struct RBTOIF_R(crate::FieldReader<bool, bool>);
impl RBTOIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBTOIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBTOIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACERRIF` reader - Auto Convention Error Interrupt Status Flag (Read Only)\\nThis field indicates auto convention sequence error. If the received TS at ATR state is neither 0x3B nor 0x3F, this bit will be set.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
pub struct ACERRIF_R(crate::FieldReader<bool, bool>);
impl ACERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACERRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACERRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Reach Interrupt Status Flag (Read Only)\\nThis field is used for received data reaching trigger level RXTRGLV (SC_CTL\\[7:6\\]) interrupt status flag.\\nNote: This field is the status flag of received data reaching RXTRGLV (SC_CTL\\[7:6\\]). If software reads data from SC_DAT and receiver buffer data byte number is less than RXTRGLV (SC_CTL\\[7:6\\]), this bit will be cleared automatically."]
    #[inline(always)]
    pub fn rdaif(&self) -> RDAIF_R {
        RDAIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty Interrupt Status Flag (Read Only)\\nThis field is used for transmit buffer empty interrupt status flag.\\nNote: This field is the status flag of transmit buffer empty state. If software wants to clear this bit, software must write data to DAT(SC_DAT\\[7:0\\]) buffer and then this bit will be cleared automatically."]
    #[inline(always)]
    pub fn tbeif(&self) -> TBEIF_R {
        TBEIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Status Flag (Read Only)\\nThis field is used for transfer error interrupt status flag. The transfer error states is at SC_STATUS register which includes receiver break error BEF(SC_STATUS\\[6\\]), frame error FEF(SC_STATUS\\[5\\], parity error PEF(SC_STATUS\\[4\\]
and receiver buffer overflow error RXOV(SC_STATUS\\[0\\]), transmit buffer overflow error TXOV(SC_STATUS\\[8\\]), receiver retry over limit error RXOVERR(SC_STATUS\\[22\\]
and transmitter retry over limit error TXOVERR(SC_STATUS\\[30\\]).\\nNote: This field is the status flag of BEF(SC_STATUS\\[6\\]), FEF(SC_STATUS\\[5\\]), PEF(SC_STATUS\\[4\\]), RXOV(SC_STATUS\\[0\\]), TXOV(SC_STATUS\\[8\\]), RXOVERR(SC_STATUS\\[22\\]) or TXOVERR(SC_STATUS\\[30\\]). So, if software wants to clear this bit, software must write 1 to each field."]
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer0 Interrupt Status Flag (Read Only)\\nThis field is used for TMR0 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn tmr0if(&self) -> TMR0IF_R {
        TMR0IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timer1 Interrupt Status Flag (Read Only)\\nThis field is used for TMR1 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn tmr1if(&self) -> TMR1IF_R {
        TMR1IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer2 Interrupt Status Flag (Read Only)\\nThis field is used for TMR2 interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn tmr2if(&self) -> TMR2IF_R {
        TMR2IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bgtif(&self) -> BGTIF_R {
        BGTIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Card Detect Interrupt Status Flag (Read Only)\\nThis field is used for card detect interrupt status flag. The card detect status is CINSERT (SC_STATUS\\[12\\]) and CREMOVE(SC_STATUS\\[11\\]).\\nNote: This field is the status flag of CINSERT(SC_STATUS\\[12\\]) or CREMOVE(SC_STATUS\\[11\\])\\]. So if software wants to clear this bit, software must write 1 to this field."]
    #[inline(always)]
    pub fn cdif(&self) -> CDIF_R {
        CDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Initial End Interrupt Status Flag (Read Only)\\nThis field is used for activation (ACTEN(SC_ALTCTL\\[3\\])), deactivation (DACTEN (SC_ALTCTL\\[2\\])) and warm reset (WARSTEN (SC_ALTCTL\\[4\\])) sequence interrupt status flag.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn initif(&self) -> INITIF_R {
        INITIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receiver Buffer Time-out Interrupt Status Flag (Read Only)\\nThis field is used for receiver buffer time-out interrupt status flag.\\nNote: This field is the status flag of receiver buffer time-out state. If software wants to clear this bit, software must read all receiver buffer remaining data by reading SC_DAT buffer,"]
    #[inline(always)]
    pub fn rbtoif(&self) -> RBTOIF_R {
        RBTOIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Auto Convention Error Interrupt Status Flag (Read Only)\\nThis field indicates auto convention sequence error. If the received TS at ATR state is neither 0x3B nor 0x3F, this bit will be set.\\nNote: This bit is read only, but it can be cleared by writing 1 to it."]
    #[inline(always)]
    pub fn acerrif(&self) -> ACERRIF_R {
        ACERRIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SC Interrupt Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc_intsts](index.html) module"]
pub struct SC_INTSTS_SPEC;
impl crate::RegisterSpec for SC_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc_intsts::R](R) reader structure"]
impl crate::Readable for SC_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc_intsts::W](W) writer structure"]
impl crate::Writable for SC_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC_INTSTS to value 0x02"]
impl crate::Resettable for SC_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
