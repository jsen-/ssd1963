use crate::gpio16bit_interface::{ReadWritePort, WritePort};
use stm32f1xx_hal::gpio::{
    gpioa::{CRH as ACRH, CRL as ACRL, PA0, PA1, PA10, PA11, PA12, PA13, PA14, PA15, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9},
    gpiob::{CRH as BCRH, CRL as BCRL, PB0, PB1, PB10, PB11, PB12, PB13, PB14, PB15, PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9},
    gpioc::{CRH as CCRH, CRL as CCRL, PC0, PC1, PC10, PC11, PC12, PC13, PC14, PC15, PC2, PC3, PC4, PC5, PC6, PC7, PC8, PC9},
    Floating, Input, Output, PushPull,
};

const INPUT: u32 = 0b_0100_0100_0100_0100_0100_0100_0100_0100; // Input<Floating>
const OUTPUT: u32 = 0b_0011_0011_0011_0011_0011_0011_0011_0011; // Output<PushPull>

pub struct PortA;
impl PortA {
    #[allow(unused_variables)]
    pub fn new(
        p0: PA0<Output<PushPull>>,
        p1: PA1<Output<PushPull>>,
        p2: PA2<Output<PushPull>>,
        p3: PA3<Output<PushPull>>,
        p4: PA4<Output<PushPull>>,
        p5: PA5<Output<PushPull>>,
        p6: PA6<Output<PushPull>>,
        p7: PA7<Output<PushPull>>,
        p8: PA8<Output<PushPull>>,
        p9: PA9<Output<PushPull>>,
        p10: PA10<Output<PushPull>>,
        p11: PA11<Output<PushPull>>,
        p12: PA12<Output<PushPull>>,
        p13: PA13<Output<PushPull>>,
        p14: PA14<Output<PushPull>>,
        p15: PA15<Output<PushPull>>,
    ) -> Self {
        Self
    }

    #[allow(unused_variables)]
    // TODO: find a better name
    pub fn new2(
        p0: PA0<Input<Floating>>,
        p1: PA1<Input<Floating>>,
        p2: PA2<Input<Floating>>,
        p3: PA3<Input<Floating>>,
        p4: PA4<Input<Floating>>,
        p5: PA5<Input<Floating>>,
        p6: PA6<Input<Floating>>,
        p7: PA7<Input<Floating>>,
        p8: PA8<Input<Floating>>,
        p9: PA9<Input<Floating>>,
        p10: PA10<Input<Floating>>,
        p11: PA11<Input<Floating>>,
        p12: PA12<Input<Floating>>,
        p13: PA13<Input<Floating>>,
        p14: PA14<Input<Floating>>,
        p15: PA15<Input<Floating>>,
        crl: &mut ACRL,
        crh: &mut ACRH,
    ) -> Self {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
        Self
    }
    pub fn into_rw(self, crl: ACRL, crh: ACRH) -> RwPortA {
        let mut p = RwPortA(crl, crh);
        p.dir_read();
        p
    }
}
impl WritePort for PortA {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOA::ptr()).odr.write(|w| w.bits(value as u32)) };
    }
}

pub struct RwPortA(ACRL, ACRH);
impl RwPortA {
    #[allow(unused_variables)]
    pub fn new(
        p0: PA0<Input<Floating>>,
        p1: PA1<Input<Floating>>,
        p2: PA2<Input<Floating>>,
        p3: PA3<Input<Floating>>,
        p4: PA4<Input<Floating>>,
        p5: PA5<Input<Floating>>,
        p6: PA6<Input<Floating>>,
        p7: PA7<Input<Floating>>,
        p8: PA8<Input<Floating>>,
        p9: PA9<Input<Floating>>,
        p10: PA10<Input<Floating>>,
        p11: PA11<Input<Floating>>,
        p12: PA12<Input<Floating>>,
        p13: PA13<Input<Floating>>,
        p14: PA14<Input<Floating>>,
        p15: PA15<Input<Floating>>,
        crl: ACRL,
        crh: ACRH,
    ) -> Self {
        Self(crl, crh)
    }
    pub fn into_ro(mut self) -> (PortA, ACRL, ACRH) {
        <Self as ReadWritePort>::dir_write(&mut self);
        (PortA, self.0, self.1)
    }
}
impl WritePort for RwPortA {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOA::ptr()).odr.write(|w| w.bits(value.into())) };
    }
}
impl ReadWritePort for RwPortA {
    fn get_value(&mut self) -> u16 {
        // TODO: try to find alternative without "as"
        unsafe { (&*stm32f1xx_hal::pac::GPIOA::ptr()).idr.read().bits() as u16 }
    }
    fn dir_read(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crl.write(|w| w.bits(INPUT));
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crh.write(|w| w.bits(INPUT));
        };
    }
    fn dir_write(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOA::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
    }
}

pub struct PortB;
impl PortB {
    #[allow(unused_variables)]
    pub fn new(
        p0: PB0<Output<PushPull>>,
        p1: PB1<Output<PushPull>>,
        p2: PB2<Output<PushPull>>,
        p3: PB3<Output<PushPull>>,
        p4: PB4<Output<PushPull>>,
        p5: PB5<Output<PushPull>>,
        p6: PB6<Output<PushPull>>,
        p7: PB7<Output<PushPull>>,
        p8: PB8<Output<PushPull>>,
        p9: PB9<Output<PushPull>>,
        p10: PB10<Output<PushPull>>,
        p11: PB11<Output<PushPull>>,
        p12: PB12<Output<PushPull>>,
        p13: PB13<Output<PushPull>>,
        p14: PB14<Output<PushPull>>,
        p15: PB15<Output<PushPull>>,
    ) -> Self {
        Self
    }

    #[allow(unused_variables)]
    // TODO: find a better name
    pub fn new2(
        p0: PB0<Input<Floating>>,
        p1: PB1<Input<Floating>>,
        p2: PB2<Input<Floating>>,
        p3: PB3<Input<Floating>>,
        p4: PB4<Input<Floating>>,
        p5: PB5<Input<Floating>>,
        p6: PB6<Input<Floating>>,
        p7: PB7<Input<Floating>>,
        p8: PB8<Input<Floating>>,
        p9: PB9<Input<Floating>>,
        p10: PB10<Input<Floating>>,
        p11: PB11<Input<Floating>>,
        p12: PB12<Input<Floating>>,
        p13: PB13<Input<Floating>>,
        p14: PB14<Input<Floating>>,
        p15: PB15<Input<Floating>>,
        crl: &mut BCRL,
        crh: &mut BCRH,
    ) -> Self {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
        Self
    }

    pub fn into_rw(self, crl: BCRL, crh: BCRH) -> RwPortB {
        let mut p = RwPortB(crl, crh);
        p.dir_read();
        p
    }
}
impl WritePort for PortB {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOB::ptr()).odr.write(|w| w.bits(value as u32)) };
    }
}

pub struct RwPortB(BCRL, BCRH);
impl RwPortB {
    #[allow(unused_variables)]
    pub fn new(
        p0: PB0<Input<Floating>>,
        p1: PB1<Input<Floating>>,
        p2: PB2<Input<Floating>>,
        p3: PB3<Input<Floating>>,
        p4: PB4<Input<Floating>>,
        p5: PB5<Input<Floating>>,
        p6: PB6<Input<Floating>>,
        p7: PB7<Input<Floating>>,
        p8: PB8<Input<Floating>>,
        p9: PB9<Input<Floating>>,
        p10: PB10<Input<Floating>>,
        p11: PB11<Input<Floating>>,
        p12: PB12<Input<Floating>>,
        p13: PB13<Input<Floating>>,
        p14: PB14<Input<Floating>>,
        p15: PB15<Input<Floating>>,
        crl: BCRL,
        crh: BCRH,
    ) -> Self {
        Self(crl, crh)
    }
    pub fn into_ro(mut self) -> (PortB, BCRL, BCRH) {
        <Self as ReadWritePort>::dir_write(&mut self);
        (PortB, self.0, self.1)
    }
}
impl WritePort for RwPortB {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOB::ptr()).odr.write(|w| w.bits(value.into())) };
    }
}
impl ReadWritePort for RwPortB {
    fn get_value(&mut self) -> u16 {
        // TODO: try to find alternative without "as"
        unsafe { (&*stm32f1xx_hal::pac::GPIOB::ptr()).idr.read().bits() as u16 }
    }
    fn dir_read(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crl.write(|w| w.bits(INPUT));
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crh.write(|w| w.bits(INPUT));
        };
    }
    fn dir_write(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOB::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
    }
}

pub struct PortC;
impl PortC {
    #[allow(unused_variables)]
    pub fn new(
        p0: PC0<Output<PushPull>>,
        p1: PC1<Output<PushPull>>,
        p2: PC2<Output<PushPull>>,
        p3: PC3<Output<PushPull>>,
        p4: PC4<Output<PushPull>>,
        p5: PC5<Output<PushPull>>,
        p6: PC6<Output<PushPull>>,
        p7: PC7<Output<PushPull>>,
        p8: PC8<Output<PushPull>>,
        p9: PC9<Output<PushPull>>,
        p10: PC10<Output<PushPull>>,
        p11: PC11<Output<PushPull>>,
        p12: PC12<Output<PushPull>>,
        p13: PC13<Output<PushPull>>,
        p14: PC14<Output<PushPull>>,
        p15: PC15<Output<PushPull>>,
    ) -> Self {
        Self
    }
    #[allow(unused_variables)]
    // TODO: find a better name
    pub fn new2(
        p0: PC0<Input<Floating>>,
        p1: PC1<Input<Floating>>,
        p2: PC2<Input<Floating>>,
        p3: PC3<Input<Floating>>,
        p4: PC4<Input<Floating>>,
        p5: PC5<Input<Floating>>,
        p6: PC6<Input<Floating>>,
        p7: PC7<Input<Floating>>,
        p8: PC8<Input<Floating>>,
        p9: PC9<Input<Floating>>,
        p10: PC10<Input<Floating>>,
        p11: PC11<Input<Floating>>,
        p12: PC12<Input<Floating>>,
        p13: PC13<Input<Floating>>,
        p14: PC14<Input<Floating>>,
        p15: PC15<Input<Floating>>,
        crl: &mut CCRL,
        crh: &mut CCRH,
    ) -> Self {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
        Self
    }
    pub fn into_rw(self, crl: BCRL, crh: BCRH) -> RwPortB {
        let mut p = RwPortB(crl, crh);
        p.dir_read();
        p
    }
}
impl WritePort for PortC {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOC::ptr()).odr.write(|w| w.bits(value as u32)) };
    }
}

pub struct RwPortC(CCRL, CCRH);
impl RwPortC {
    #[allow(unused_variables)]
    pub fn new(
        p0: PC0<Input<Floating>>,
        p1: PC1<Input<Floating>>,
        p2: PC2<Input<Floating>>,
        p3: PC3<Input<Floating>>,
        p4: PC4<Input<Floating>>,
        p5: PC5<Input<Floating>>,
        p6: PC6<Input<Floating>>,
        p7: PC7<Input<Floating>>,
        p8: PC8<Input<Floating>>,
        p9: PC9<Input<Floating>>,
        p10: PC10<Input<Floating>>,
        p11: PC11<Input<Floating>>,
        p12: PC12<Input<Floating>>,
        p13: PC13<Input<Floating>>,
        p14: PC14<Input<Floating>>,
        p15: PC15<Input<Floating>>,
        crl: CCRL,
        crh: CCRH,
    ) -> Self {
        Self(crl, crh)
    }
    pub fn into_ro(mut self) -> (PortC, CCRL, CCRH) {
        <Self as ReadWritePort>::dir_write(&mut self);
        (PortC, self.0, self.1)
    }
}
impl WritePort for RwPortC {
    fn set_value(&mut self, value: u16) {
        unsafe { (&*stm32f1xx_hal::pac::GPIOC::ptr()).odr.write(|w| w.bits(value.into())) };
    }
}
impl ReadWritePort for RwPortC {
    fn get_value(&mut self) -> u16 {
        unsafe { (&*stm32f1xx_hal::pac::GPIOC::ptr()).idr.read().bits() as u16 }
    }
    fn dir_read(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crl.write(|w| w.bits(INPUT));
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crh.write(|w| w.bits(INPUT));
        };
    }
    fn dir_write(&mut self) {
        unsafe {
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crl.write(|w| w.bits(OUTPUT));
            (&*stm32f1xx_hal::pac::GPIOC::ptr()).crh.write(|w| w.bits(OUTPUT));
        };
    }
}
