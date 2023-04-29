// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Input as DefaultMode;

#[cfg(feature = "gpio-f031")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [0, 1, 2, 3]),
    PA1: (pa1, 1, [0, 1, 2, 3]),
    PA2: (pa2, 2, [0, 1, 2, 3]),
    PA3: (pa3, 3, [0, 1, 2, 3]),
    PA4: (pa4, 4, [0, 1, 3, 4]),
    PA5: (pa5, 5, [0, 1, 2, 3]),
    PA6: (pa6, 6, [0, 1, 2, 3, 5, 6]),
    PA7: (pa7, 7, [0, 1, 2, 3, 4, 5, 6]),
    PA8: (pa8, 8, [0, 1, 2, 3]),
    PA9: (pa9, 9, [0, 1, 2, 3, 4]),
    PA10: (pa10, 10, [0, 1, 2, 3, 4]),
    PA11: (pa11, 11, [0, 1, 2, 3]),
    PA12: (pa12, 12, [0, 1, 2, 3]),
    PA13: (pa13, 13, [1], super::Debugger),
    PA14: (pa14, 14, [0, 1], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3], super::Debugger),
]);

#[cfg(feature = "gpio-f031")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3]),
    PB1: (pb1, 1, [0, 1, 2, 3]),
    PB2: (pb2, 2, [3]),
    PB3: (pb3, 3, [0, 1, 2, 3], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3]),
    PB8: (pb8, 8, [1, 2, 3]),
    PB9: (pb9, 9, [0, 1, 2, 3]),
    PB10: (pb10, 10, [1, 2, 3]),
    PB11: (pb11, 11, [0, 1, 2, 3]),
    PB12: (pb12, 12, [0, 1, 2, 3]),
    PB13: (pb13, 13, [0, 2, 3]),
    PB14: (pb14, 14, [0, 1, 2, 3]),
    PB15: (pb15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-f031")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f031")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, []),
    PF1: (pf1, 1, []),
    PF6: (pf6, 6, []),
    PF7: (pf7, 7, []),
]);

#[cfg(feature = "gpio-f042")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3]),
    PA1: (pa1, 1, [0, 1, 2, 3]),
    PA2: (pa2, 2, [1, 2, 3]),
    PA3: (pa3, 3, [1, 2, 3]),
    PA4: (pa4, 4, [0, 1, 2, 3, 4]),
    PA5: (pa5, 5, [0, 1, 2, 3]),
    PA6: (pa6, 6, [0, 1, 2, 3, 5, 6]),
    PA7: (pa7, 7, [0, 1, 2, 3, 4, 5, 6]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4]),
    PA9: (pa9, 9, [1, 2, 3, 4, 5]),
    PA10: (pa10, 10, [0, 1, 2, 3, 4]),
    PA11: (pa11, 11, [0, 1, 2, 3, 4, 5]),
    PA12: (pa12, 12, [0, 1, 2, 3, 4, 5]),
    PA13: (pa13, 13, [0, 1, 2], super::Debugger),
    PA14: (pa14, 14, [0, 1], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 5], super::Debugger),
]);

#[cfg(feature = "gpio-f042")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3]),
    PB1: (pb1, 1, [0, 1, 2, 3]),
    PB2: (pb2, 2, [3]),
    PB3: (pb3, 3, [0, 1, 2, 3], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 5], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3]),
    PB8: (pb8, 8, [0, 1, 2, 3, 4]),
    PB9: (pb9, 9, [0, 1, 2, 3, 4, 5]),
    PB10: (pb10, 10, [0, 1, 2, 3, 5]),
    PB11: (pb11, 11, [0, 1, 2]),
    PB12: (pb12, 12, [0, 1, 2]),
    PB13: (pb13, 13, [0, 2, 5]),
    PB14: (pb14, 14, [0, 2, 5]),
    PB15: (pb15, 15, [0, 2]),
]);

#[cfg(feature = "gpio-f042")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f042")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [0, 1]),
    PF1: (pf1, 1, [1]),
    PF11: (pf11, 11, []),
]);

#[cfg(feature = "gpio-f051")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [0, 1, 2, 3, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 7]),
    PA2: (pa2, 2, [0, 1, 2, 3, 7]),
    PA3: (pa3, 3, [0, 1, 2, 3, 7]),
    PA4: (pa4, 4, [0, 1, 3, 4, 7]),
    PA5: (pa5, 5, [0, 1, 2, 3, 7]),
    PA6: (pa6, 6, [0, 1, 2, 3, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA8: (pa8, 8, [0, 1, 2, 3]),
    PA9: (pa9, 9, [0, 1, 2, 3]),
    PA10: (pa10, 10, [0, 1, 2, 3]),
    PA11: (pa11, 11, [0, 1, 2, 3, 7]),
    PA12: (pa12, 12, [0, 1, 2, 3, 7]),
    PA13: (pa13, 13, [1], super::Debugger),
    PA14: (pa14, 14, [0, 1], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3], super::Debugger),
]);

#[cfg(feature = "gpio-f051")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3]),
    PB1: (pb1, 1, [0, 1, 2, 3]),
    PB2: (pb2, 2, [3]),
    PB3: (pb3, 3, [0, 1, 2, 3], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3]),
    PB8: (pb8, 8, [0, 1, 2, 3]),
    PB9: (pb9, 9, [0, 1, 2, 3]),
    PB10: (pb10, 10, [0, 1, 2, 3]),
    PB11: (pb11, 11, [0, 1, 2, 3]),
    PB12: (pb12, 12, [0, 1, 2, 3]),
    PB13: (pb13, 13, [0, 2, 3]),
    PB14: (pb14, 14, [0, 1, 2, 3]),
    PB15: (pb15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-f051")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, []),
    PC1: (pc1, 1, []),
    PC2: (pc2, 2, []),
    PC3: (pc3, 3, []),
    PC4: (pc4, 4, []),
    PC5: (pc5, 5, []),
    PC6: (pc6, 6, [0]),
    PC7: (pc7, 7, [0]),
    PC8: (pc8, 8, [0]),
    PC9: (pc9, 9, [0]),
    PC10: (pc10, 10, []),
    PC11: (pc11, 11, []),
    PC12: (pc12, 12, []),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f051")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD2: (pd2, 2, [0]),
]);

#[cfg(feature = "gpio-f051")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, []),
    PF1: (pf1, 1, []),
    PF4: (pf4, 4, []),
    PF5: (pf5, 5, []),
    PF6: (pf6, 6, []),
    PF7: (pf7, 7, []),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 4, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5]),
    PA2: (pa2, 2, [0, 1, 2, 3, 7]),
    PA3: (pa3, 3, [0, 1, 2, 3]),
    PA4: (pa4, 4, [0, 1, 3, 4]),
    PA5: (pa5, 5, [0, 1, 2, 3]),
    PA6: (pa6, 6, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4]),
    PA9: (pa9, 9, [0, 1, 2, 3]),
    PA10: (pa10, 10, [0, 1, 2, 3]),
    PA11: (pa11, 11, [0, 1, 2, 3, 4, 7]),
    PA12: (pa12, 12, [0, 1, 2, 3, 4, 7]),
    PA13: (pa13, 13, [0, 1, 2], super::Debugger),
    PA14: (pa14, 14, [0, 1], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4], super::Debugger),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3, 4]),
    PB1: (pb1, 1, [0, 1, 2, 3, 4]),
    PB2: (pb2, 2, [3]),
    PB3: (pb3, 3, [0, 1, 2, 3], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 5], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3, 4]),
    PB8: (pb8, 8, [0, 1, 2, 3, 4]),
    PB9: (pb9, 9, [0, 1, 2, 3, 4, 5]),
    PB10: (pb10, 10, [0, 1, 2, 3, 4, 5]),
    PB11: (pb11, 11, [0, 1, 2, 3, 4]),
    PB12: (pb12, 12, [0, 1, 2, 3, 4, 5]),
    PB13: (pb13, 13, [0, 2, 3, 4, 5]),
    PB14: (pb14, 14, [0, 1, 2, 3, 4, 5]),
    PB15: (pb15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 7]),
    PC1: (pc1, 1, [0, 7]),
    PC2: (pc2, 2, [0, 1, 7]),
    PC3: (pc3, 3, [0, 1, 7]),
    PC4: (pc4, 4, [0, 1]),
    PC5: (pc5, 5, [0, 1, 7]),
    PC6: (pc6, 6, [0, 7]),
    PC7: (pc7, 7, [0, 7]),
    PC8: (pc8, 8, [0, 7]),
    PC9: (pc9, 9, [0, 7]),
    PC10: (pc10, 10, [0, 1, 7]),
    PC11: (pc11, 11, [0, 1, 7]),
    PC12: (pc12, 12, [0, 1, 7]),
    PC13: (pc13, 13, [7]),
    PC14: (pc14, 14, [7]),
    PC15: (pc15, 15, [7]),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [0, 1]),
    PD1: (pd1, 1, [0, 1]),
    PD2: (pd2, 2, [0, 1]),
    PD3: (pd3, 3, [0, 1, 4]),
    PD4: (pd4, 4, [0, 1, 4]),
    PD5: (pd5, 5, [0]),
    PD6: (pd6, 6, [0]),
    PD7: (pd7, 7, [0]),
    PD8: (pd8, 8, [0, 4]),
    PD9: (pd9, 9, [0]),
    PD10: (pd10, 10, [0]),
    PD11: (pd11, 11, [0]),
    PD12: (pd12, 12, [0, 1]),
    PD13: (pd13, 13, [1]),
    PD14: (pd14, 14, [1]),
    PD15: (pd15, 15, [0, 1]),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [0, 1]),
    PE1: (pe1, 1, [0, 1]),
    PE2: (pe2, 2, [0, 1]),
    PE3: (pe3, 3, [0, 1]),
    PE4: (pe4, 4, [0, 1]),
    PE5: (pe5, 5, [0, 1]),
    PE6: (pe6, 6, [0]),
    PE7: (pe7, 7, [0]),
    PE8: (pe8, 8, [0]),
    PE9: (pe9, 9, [0]),
    PE10: (pe10, 10, [0]),
    PE11: (pe11, 11, [0]),
    PE12: (pe12, 12, [0, 1]),
    PE13: (pe13, 13, [0, 1]),
    PE14: (pe14, 14, [0, 1]),
    PE15: (pe15, 15, [0, 1]),
]);

#[cfg(feature = "gpio-f052")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [0]),
    PF1: (pf1, 1, []),
    PF2: (pf2, 2, [0]),
    PF3: (pf3, 3, [0]),
    PF6: (pf6, 6, []),
    PF9: (pf9, 9, [0]),
    PF10: (pf10, 10, [0]),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 4, 7]),
    PA1: (pa1, 1, [0, 1, 2, 3, 4, 5]),
    PA2: (pa2, 2, [0, 1, 2, 3, 7]),
    PA3: (pa3, 3, [0, 1, 2, 3]),
    PA4: (pa4, 4, [0, 1, 3, 4, 5]),
    PA5: (pa5, 5, [0, 1, 2, 3, 5]),
    PA6: (pa6, 6, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA7: (pa7, 7, [0, 1, 2, 3, 4, 5, 6, 7]),
    PA8: (pa8, 8, [0, 1, 2, 3, 4]),
    PA9: (pa9, 9, [0, 1, 2, 3, 4, 5]),
    PA10: (pa10, 10, [0, 1, 2, 3, 4]),
    PA11: (pa11, 11, [0, 1, 2, 3, 4, 5, 7]),
    PA12: (pa12, 12, [0, 1, 2, 3, 4, 5, 7]),
    PA13: (pa13, 13, [0, 1, 2], super::Debugger),
    PA14: (pa14, 14, [0, 1], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 4], super::Debugger),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 2, 3, 4]),
    PB1: (pb1, 1, [0, 1, 2, 3, 4]),
    PB2: (pb2, 2, [3]),
    PB3: (pb3, 3, [0, 1, 2, 3, 4], super::Debugger),
    PB4: (pb4, 4, [0, 1, 2, 3, 4, 5], super::Debugger),
    PB5: (pb5, 5, [0, 1, 2, 3, 4]),
    PB6: (pb6, 6, [0, 1, 2, 3]),
    PB7: (pb7, 7, [0, 1, 2, 3, 4]),
    PB8: (pb8, 8, [0, 1, 2, 3, 4]),
    PB9: (pb9, 9, [0, 1, 2, 3, 4, 5]),
    PB10: (pb10, 10, [0, 1, 2, 3, 4, 5]),
    PB11: (pb11, 11, [0, 1, 2, 3, 4]),
    PB12: (pb12, 12, [0, 1, 2, 3, 4, 5]),
    PB13: (pb13, 13, [0, 2, 3, 4, 5]),
    PB14: (pb14, 14, [0, 1, 2, 3, 4, 5]),
    PB15: (pb15, 15, [0, 1, 2, 3]),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 1, 2]),
    PC1: (pc1, 1, [0, 1, 2]),
    PC2: (pc2, 2, [0, 1, 2]),
    PC3: (pc3, 3, [0, 1, 2]),
    PC4: (pc4, 4, [0, 1]),
    PC5: (pc5, 5, [0, 1]),
    PC6: (pc6, 6, [0, 1]),
    PC7: (pc7, 7, [0, 1]),
    PC8: (pc8, 8, [0, 1]),
    PC9: (pc9, 9, [0, 1]),
    PC10: (pc10, 10, [0, 1]),
    PC11: (pc11, 11, [0, 1]),
    PC12: (pc12, 12, [0, 1, 2]),
    PC13: (pc13, 13, []),
    PC14: (pc14, 14, []),
    PC15: (pc15, 15, []),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [0, 1]),
    PD1: (pd1, 1, [0, 1]),
    PD2: (pd2, 2, [0, 1, 2]),
    PD3: (pd3, 3, [0, 1]),
    PD4: (pd4, 4, [0, 1]),
    PD5: (pd5, 5, [0]),
    PD6: (pd6, 6, [0]),
    PD7: (pd7, 7, [0]),
    PD8: (pd8, 8, [0]),
    PD9: (pd9, 9, [0]),
    PD10: (pd10, 10, [0]),
    PD11: (pd11, 11, [0]),
    PD12: (pd12, 12, [0, 1, 2]),
    PD13: (pd13, 13, [0, 1]),
    PD14: (pd14, 14, [0, 1]),
    PD15: (pd15, 15, [0, 1, 2]),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [0, 1]),
    PE1: (pe1, 1, [0, 1]),
    PE2: (pe2, 2, [0, 1]),
    PE3: (pe3, 3, [0, 1]),
    PE4: (pe4, 4, [0, 1]),
    PE5: (pe5, 5, [0, 1]),
    PE6: (pe6, 6, [0]),
    PE7: (pe7, 7, [0, 1]),
    PE8: (pe8, 8, [0, 1]),
    PE9: (pe9, 9, [0, 1]),
    PE10: (pe10, 10, [0, 1]),
    PE11: (pe11, 11, [0, 1]),
    PE12: (pe12, 12, [0, 1]),
    PE13: (pe13, 13, [0, 1]),
    PE14: (pe14, 14, [0, 1]),
    PE15: (pe15, 15, [0, 1]),
]);

#[cfg(feature = "gpio-f091")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [0, 1]),
    PF1: (pf1, 1, [1]),
    PF2: (pf2, 2, [0, 1, 2]),
    PF3: (pf3, 3, [0, 1, 2]),
    PF6: (pf6, 6, []),
    PF9: (pf9, 9, [0, 1]),
    PF10: (pf10, 10, [0, 1]),
]);
