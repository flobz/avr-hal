use avr_device::atmega4809::clkctrl;
use avr_hal_generic::port;

fn init(portmux: &crate::pac::PORTMUX, clkctrl: &crate::pac::CLKCTRL, cpu: &crate::pac::CPU) {
    portmux.usartroutea.write(|w| w.usart1().alt1().usart3().alt1());
      // PORTMUX setting for TCA -> all outputs [0:2] point to PORTB pins [0:2] timer
    portmux.tcaroutea.write(|w|w.tca0().portb());

    portmux.tcbroutea.write(|w|w.tcb0().set_bit().tcb1().set_bit());

    cpu.ccp.write(|w|w.ccp().ioreg());
    clkctrl.mclkctrlb.write(|w|w.pen().clear_bit());
}

#[cfg(feature = "atmega4809")]
avr_hal_generic::impl_port_new! {
    init: |portmux,clkctrl,cpu|{init(portmux,clkctrl, cpu)},
    enum Ports {
        PORTA: crate::pac::PORTA,
        PORTB: crate::pac::PORTB,
        PORTC: crate::pac::PORTC,
        PORTD: crate::pac::PORTD,
        PORTE: crate::pac::PORTE,
        PORTF: crate::pac::PORTF,
    }

    pub struct Pins {
        pa0: PA0 = (crate::pac::PORTA, PORTA, 0),
        pa1: PA1 = (crate::pac::PORTA, PORTA, 1),
        pa2: PA2 = (crate::pac::PORTA, PORTA, 2),
        pa3: PA3 = (crate::pac::PORTA, PORTA, 3),
        pa4: PA4 = (crate::pac::PORTA, PORTA, 4),
        pa5: PA5 = (crate::pac::PORTA, PORTA, 5),
        pa6: PA6 = (crate::pac::PORTA, PORTA, 6),
        pa7: PA7 = (crate::pac::PORTA, PORTA, 7),
        pb0: PB0 = (crate::pac::PORTB, PORTB, 0),
        pb1: PB1 = (crate::pac::PORTB, PORTB, 1),
        pb2: PB2 = (crate::pac::PORTB, PORTB, 2),
        pb3: PB3 = (crate::pac::PORTB, PORTB, 3),
        pb4: PB4 = (crate::pac::PORTB, PORTB, 4),
        pb5: PB5 = (crate::pac::PORTB, PORTB, 5),
        pc0: PC0 = (crate::pac::PORTC, PORTC, 0),
        pc1: PC1 = (crate::pac::PORTC, PORTC, 1),
        pc2: PC2 = (crate::pac::PORTC, PORTC, 2),
        pc3: PC3 = (crate::pac::PORTC, PORTC, 3),
        pc4: PC4 = (crate::pac::PORTC, PORTC, 4),
        pc5: PC5 = (crate::pac::PORTC, PORTC, 5),
        pc6: PC6 = (crate::pac::PORTC, PORTC, 6),
        pc7: PC7 = (crate::pac::PORTC, PORTC, 7),
        pd0: PD0 = (crate::pac::PORTD, PORTD, 0),
        pd1: PD1 = (crate::pac::PORTD, PORTD, 1),
        pd2: PD2 = (crate::pac::PORTD, PORTD, 2),
        pd3: PD3 = (crate::pac::PORTD, PORTD, 3),
        pd4: PD4 = (crate::pac::PORTD, PORTD, 4),
        pd5: PD5 = (crate::pac::PORTD, PORTD, 5),
        pd6: PD6 = (crate::pac::PORTD, PORTD, 6),
        pd7: PD7 = (crate::pac::PORTD, PORTD, 7),
        pe0: PE0 = (crate::pac::PORTE, PORTE, 0),
        pe1: PE1 = (crate::pac::PORTE, PORTE, 1),
        pe2: PE2 = (crate::pac::PORTE, PORTE, 2),
        pe3: PE3 = (crate::pac::PORTE, PORTE, 3),
        pf0: PF0 = (crate::pac::PORTF, PORTF, 0),
        pf1: PF1 = (crate::pac::PORTF, PORTF, 1),
        pf2: PF2 = (crate::pac::PORTF, PORTF, 2),
        pf3: PF3 = (crate::pac::PORTF, PORTF, 3),
        pf4: PF4 = (crate::pac::PORTF, PORTF, 4),
        pf5: PF5 = (crate::pac::PORTF, PORTF, 5),
        pf6: PF6 = (crate::pac::PORTF, PORTF, 6),
    }
}
