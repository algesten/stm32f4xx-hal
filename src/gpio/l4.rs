// auto-generated using codegen
// STM32CubeMX DB release: DB.6.0.50
pub use super::*;

pub use super::Analog as DefaultMode;

#[cfg(feature = "gpio-l41x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 7, 12, 14, 15]),
    PA1: (pa1, 1, [1, 4, 5, 7, 14, 15]),
    PA2: (pa2, 2, [1, 7, 8, 10, 14, 15]),
    PA3: (pa3, 3, [1, 7, 8, 10, 14, 15]),
    PA4: (pa4, 4, [5, 7, 14, 15]),
    PA5: (pa5, 5, [1, 2, 5, 14, 15]),
    PA6: (pa6, 6, [1, 5, 6, 7, 8, 10, 14, 15]),
    PA7: (pa7, 7, [1, 4, 5, 10, 15]),
    PA8: (pa8, 8, [0, 1, 7, 14, 15]),
    PA9: (pa9, 9, [1, 4, 7, 14, 15]),
    PA10: (pa10, 10, [1, 4, 7, 10, 15]),
    PA11: (pa11, 11, [1, 2, 5, 6, 7, 10, 12, 15]),
    PA12: (pa12, 12, [1, 5, 7, 10, 15]),
    PA13: (pa13, 13, [0, 1, 10, 15], super::Debugger),
    PA14: (pa14, 14, [0, 1, 4, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 5, 7, 9, 15], super::Debugger),
]);

#[cfg(feature = "gpio-l41x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [0, 1, 5, 7, 10, 12, 15]),
    PB1: (pb1, 1, [0, 1, 7, 8, 10, 14, 15]),
    PB2: (pb2, 2, [0, 1, 4, 15]),
    PB3: (pb3, 3, [0, 1, 5, 7, 15], super::Debugger),
    PB4: (pb4, 4, [0, 4, 5, 7, 9, 15], super::Debugger),
    PB5: (pb5, 5, [0, 1, 4, 5, 7, 9, 14, 15]),
    PB6: (pb6, 6, [0, 1, 4, 7, 9, 14, 15]),
    PB7: (pb7, 7, [0, 1, 4, 7, 9, 15]),
    PB8: (pb8, 8, [4, 14, 15]),
    PB9: (pb9, 9, [1, 4, 5, 15]),
    PB10: (pb10, 10, [1, 4, 5, 7, 8, 9, 10, 12, 15]),
    PB11: (pb11, 11, [1, 4, 7, 8, 10, 15]),
    PB12: (pb12, 12, [1, 4, 5, 7, 8, 9, 14, 15]),
    PB13: (pb13, 13, [1, 4, 5, 7, 8, 9, 14, 15]),
    PB14: (pb14, 14, [1, 4, 5, 7, 9, 14, 15]),
    PB15: (pb15, 15, [0, 1, 5, 9, 14, 15]),
]);

#[cfg(feature = "gpio-l41x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [0, 1, 4, 8, 14, 15]),
    PC1: (pc1, 1, [0, 1, 4, 8, 15]),
    PC2: (pc2, 2, [1, 5, 15]),
    PC3: (pc3, 3, [1, 5, 14, 15]),
    PC4: (pc4, 4, [7, 15]),
    PC5: (pc5, 5, [7, 15]),
    PC6: (pc6, 6, [9, 15]),
    PC7: (pc7, 7, [9, 15]),
    PC8: (pc8, 8, [9, 15]),
    PC9: (pc9, 9, [9, 10, 15]),
    PC10: (pc10, 10, [0, 7, 9, 15]),
    PC11: (pc11, 11, [7, 9, 15]),
    PC12: (pc12, 12, [0, 7, 9, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-l41x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD2: (pd2, 2, [0, 7, 9, 15]),
]);

#[cfg(feature = "gpio-l41x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
    PH3: (ph3, 3, [15]),
]);
/*
#[cfg(feature = "gpio-l41x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 7, 12, 13, 14, 15]),
    PA1: (pa1, 1, [1, 4, 5, 7, 11, 14, 15]),
    PA2: (pa2, 2, [1, 7, 8, 10, 11, 12, 14, 15]),
    PA3: (pa3, 3, [1, 7, 8, 10, 11, 13, 14, 15]),
    PA4: (pa4, 4, [5, 6, 7, 13, 14, 15]),
    PA5: (pa5, 5, [1, 2, 5, 14, 15]),
    PA6: (pa6, 6, [1, 5, 6, 7, 8, 10, 11, 12, 14, 15]),
    PA7: (pa7, 7, [1, 4, 5, 10, 11, 12, 15]),
    PA8: (pa8, 8, [0, 1, 7, 11, 12, 13, 14, 15]),
    PA9: (pa9, 9, [1, 4, 7, 11, 13, 14, 15]),
    PA10: (pa10, 10, [1, 4, 7, 10, 11, 13, 15]),
    PA11: (pa11, 11, [1, 2, 5, 6, 7, 9, 10, 12, 15]),
    PA12: (pa12, 12, [1, 5, 7, 9, 10, 15]),
    PA13: (pa13, 13, [0, 1, 10, 12, 13, 15], super::Debugger),
    PA14: (pa14, 14, [0, 1, 4, 12, 13, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 5, 6, 7, 9, 11, 12, 15], super::Debugger),
]);

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 5, 7, 10, 11, 12, 13, 15]),
    PB1: (pb1, 1, [1, 7, 8, 10, 11, 14, 15]),
    PB2: (pb2, 2, [0, 1, 4, 11, 15]),
    PB3: (pb3, 3, [0, 1, 5, 6, 7, 11, 13, 15], super::Debugger),
    PB4: (pb4, 4, [0, 4, 5, 6, 7, 9, 11, 13, 15], super::Debugger),
    PB5: (pb5, 5, [1, 4, 5, 6, 7, 9, 11, 12, 13, 14, 15]),
    PB6: (pb6, 6, [1, 4, 7, 9, 13, 14, 15]),
    PB7: (pb7, 7, [1, 4, 7, 9, 11, 15]),
    PB8: (pb8, 8, [4, 9, 11, 12, 13, 14, 15]),
    PB9: (pb9, 9, [1, 4, 5, 9, 11, 12, 13, 15]),
    PB10: (pb10, 10, [1, 4, 5, 7, 8, 9, 10, 11, 12, 13, 15]),
    PB11: (pb11, 11, [1, 4, 7, 8, 10, 11, 12, 15]),
    PB12: (pb12, 12, [1, 3, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB13: (pb13, 13, [1, 4, 5, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB14: (pb14, 14, [1, 4, 5, 7, 9, 11, 12, 13, 14, 15]),
    PB15: (pb15, 15, [0, 1, 5, 9, 11, 12, 13, 14, 15]),
]);

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 4, 8, 11, 14, 15]),
    PC1: (pc1, 1, [1, 4, 8, 11, 15]),
    PC2: (pc2, 2, [1, 5, 11, 15]),
    PC3: (pc3, 3, [1, 5, 11, 13, 14, 15]),
    PC4: (pc4, 4, [7, 11, 15]),
    PC5: (pc5, 5, [7, 11, 15]),
    PC6: (pc6, 6, [9, 11, 12, 15]),
    PC7: (pc7, 7, [9, 11, 12, 15]),
    PC8: (pc8, 8, [9, 11, 12, 15]),
    PC9: (pc9, 9, [9, 10, 11, 12, 15]),
    PC10: (pc10, 10, [6, 7, 9, 11, 12, 15]),
    PC11: (pc11, 11, [6, 7, 9, 11, 12, 15]),
    PC12: (pc12, 12, [6, 7, 9, 11, 12, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [5, 9, 15]),
    PD1: (pd1, 1, [5, 9, 15]),
    PD2: (pd2, 2, [7, 9, 11, 12, 15]),
    PD3: (pd3, 3, [5, 7, 10, 15]),
    PD4: (pd4, 4, [5, 7, 10, 15]),
    PD5: (pd5, 5, [7, 10, 15]),
    PD6: (pd6, 6, [7, 10, 13, 15]),
    PD7: (pd7, 7, [7, 10, 15]),
    PD8: (pd8, 8, [7, 11, 15]),
    PD9: (pd9, 9, [7, 11, 15]),
    PD10: (pd10, 10, [7, 9, 11, 15]),
    PD11: (pd11, 11, [7, 9, 11, 14, 15]),
    PD12: (pd12, 12, [7, 9, 11, 14, 15]),
    PD13: (pd13, 13, [9, 11, 14, 15]),
    PD14: (pd14, 14, [11, 15]),
    PD15: (pd15, 15, [11, 15]),
]);

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [11, 14, 15]),
    PE1: (pe1, 1, [11, 15]),
    PE2: (pe2, 2, [0, 9, 11, 13, 15]),
    PE3: (pe3, 3, [0, 9, 11, 13, 15]),
    PE4: (pe4, 4, [0, 9, 13, 15]),
    PE5: (pe5, 5, [0, 9, 13, 15]),
    PE6: (pe6, 6, [0, 13, 15]),
    PE7: (pe7, 7, [1, 13, 15]),
    PE8: (pe8, 8, [1, 13, 15]),
    PE9: (pe9, 9, [1, 13, 15]),
    PE10: (pe10, 10, [1, 9, 10, 13, 15]),
    PE11: (pe11, 11, [1, 9, 10, 15]),
    PE12: (pe12, 12, [1, 5, 9, 10, 15]),
    PE13: (pe13, 13, [1, 5, 9, 10, 15]),
    PE14: (pe14, 14, [1, 2, 3, 5, 10, 15]),
    PE15: (pe15, 15, [1, 3, 5, 10, 15]),
]);

#[cfg(feature = "gpio-l43x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
    PH3: (ph3, 3, [15]),
]);
/*
#[cfg(feature = "gpio-l43x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 7, 8, 12, 13, 14, 15]),
    PA1: (pa1, 1, [1, 4, 5, 7, 8, 14, 15]),
    PA2: (pa2, 2, [1, 7, 8, 10, 12, 14, 15]),
    PA3: (pa3, 3, [1, 7, 8, 10, 13, 14, 15]),
    PA4: (pa4, 4, [5, 6, 7, 13, 14, 15]),
    PA5: (pa5, 5, [1, 2, 5, 6, 14, 15]),
    PA6: (pa6, 6, [1, 2, 5, 6, 7, 8, 10, 12, 14, 15]),
    PA7: (pa7, 7, [1, 2, 4, 5, 6, 10, 12, 15]),
    PA8: (pa8, 8, [0, 1, 6, 7, 13, 14, 15]),
    PA9: (pa9, 9, [1, 4, 6, 7, 13, 14, 15]),
    PA10: (pa10, 10, [1, 4, 7, 10, 13, 15]),
    PA11: (pa11, 11, [1, 2, 5, 6, 7, 9, 10, 12, 15]),
    PA12: (pa12, 12, [1, 5, 7, 9, 10, 15]),
    PA13: (pa13, 13, [0, 1, 10, 13, 15], super::Debugger),
    PA14: (pa14, 14, [0, 1, 4, 5, 13, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 5, 6, 7, 8, 9, 15], super::Debugger),
]);

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 5, 6, 7, 10, 12, 13, 15]),
    PB1: (pb1, 1, [1, 2, 6, 7, 8, 10, 14, 15]),
    PB2: (pb2, 2, [0, 1, 4, 6, 15]),
    PB3: (pb3, 3, [0, 1, 5, 6, 7, 13, 15], super::Debugger),
    PB4: (pb4, 4, [0, 2, 4, 5, 6, 7, 9, 13, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 9, 12, 13, 14, 15]),
    PB6: (pb6, 6, [1, 4, 5, 7, 8, 9, 13, 14, 15]),
    PB7: (pb7, 7, [1, 4, 5, 7, 8, 9, 15]),
    PB8: (pb8, 8, [4, 9, 12, 13, 14, 15]),
    PB9: (pb9, 9, [1, 4, 5, 9, 12, 13, 15]),
    PB10: (pb10, 10, [1, 3, 4, 5, 7, 8, 9, 10, 12, 13, 15]),
    PB11: (pb11, 11, [1, 3, 4, 7, 8, 10, 12, 15]),
    PB12: (pb12, 12, [1, 3, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15]),
    PB13: (pb13, 13, [1, 4, 5, 6, 7, 8, 9, 10, 13, 14, 15]),
    PB14: (pb14, 14, [1, 4, 5, 6, 7, 9, 13, 14, 15]),
    PB15: (pb15, 15, [0, 1, 5, 6, 9, 13, 14, 15]),
]);

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2, 4, 8, 14, 15]),
    PC1: (pc1, 1, [0, 1, 2, 4, 8, 15]),
    PC2: (pc2, 2, [1, 5, 6, 15]),
    PC3: (pc3, 3, [1, 5, 13, 14, 15]),
    PC4: (pc4, 4, [7, 15]),
    PC5: (pc5, 5, [7, 15]),
    PC6: (pc6, 6, [2, 6, 9, 12, 15]),
    PC7: (pc7, 7, [2, 6, 9, 12, 15]),
    PC8: (pc8, 8, [2, 9, 12, 15]),
    PC9: (pc9, 9, [2, 9, 10, 12, 15]),
    PC10: (pc10, 10, [0, 6, 7, 8, 9, 12, 15]),
    PC11: (pc11, 11, [6, 7, 8, 9, 12, 15]),
    PC12: (pc12, 12, [0, 6, 7, 9, 12, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [5, 9, 15]),
    PD1: (pd1, 1, [5, 9, 15]),
    PD2: (pd2, 2, [0, 2, 7, 9, 12, 15]),
    PD3: (pd3, 3, [5, 6, 7, 10, 15]),
    PD4: (pd4, 4, [5, 6, 7, 10, 15]),
    PD5: (pd5, 5, [7, 10, 15]),
    PD6: (pd6, 6, [6, 7, 10, 13, 15]),
    PD7: (pd7, 7, [6, 7, 10, 15]),
    PD8: (pd8, 8, [7, 15]),
    PD9: (pd9, 9, [7, 15]),
    PD10: (pd10, 10, [7, 9, 15]),
    PD11: (pd11, 11, [4, 7, 9, 14, 15]),
    PD12: (pd12, 12, [4, 7, 9, 14, 15]),
    PD13: (pd13, 13, [4, 9, 14, 15]),
    PD14: (pd14, 14, [15]),
    PD15: (pd15, 15, [15]),
]);

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [14, 15]),
    PE1: (pe1, 1, [15]),
    PE2: (pe2, 2, [0, 2, 9, 13, 15]),
    PE3: (pe3, 3, [0, 2, 9, 13, 15]),
    PE4: (pe4, 4, [0, 2, 6, 9, 13, 15]),
    PE5: (pe5, 5, [0, 2, 6, 9, 13, 15]),
    PE6: (pe6, 6, [0, 2, 13, 15]),
    PE7: (pe7, 7, [1, 6, 13, 15]),
    PE8: (pe8, 8, [1, 6, 13, 15]),
    PE9: (pe9, 9, [1, 6, 13, 15]),
    PE10: (pe10, 10, [1, 9, 10, 13, 15]),
    PE11: (pe11, 11, [1, 9, 10, 15]),
    PE12: (pe12, 12, [1, 5, 9, 10, 15]),
    PE13: (pe13, 13, [1, 5, 9, 10, 15]),
    PE14: (pe14, 14, [1, 2, 3, 5, 10, 15]),
    PE15: (pe15, 15, [1, 3, 5, 10, 15]),
]);

#[cfg(feature = "gpio-l45x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
    PH3: (ph3, 3, [15]),
]);
/*
#[cfg(feature = "gpio-l45x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 13, 14, 15]),
    PA1: (pa1, 1, [1, 2, 7, 8, 11, 14, 15]),
    PA2: (pa2, 2, [1, 2, 7, 11, 13, 14, 15]),
    PA3: (pa3, 3, [1, 2, 7, 11, 14, 15]),
    PA4: (pa4, 4, [5, 6, 7, 13, 14, 15]),
    PA5: (pa5, 5, [1, 2, 3, 5, 14, 15]),
    PA6: (pa6, 6, [1, 2, 3, 5, 7, 10, 11, 12, 13, 14, 15]),
    PA7: (pa7, 7, [1, 2, 3, 5, 10, 11, 14, 15]),
    PA8: (pa8, 8, [0, 1, 7, 10, 11, 14, 15]),
    PA9: (pa9, 9, [1, 7, 11, 14, 15]),
    PA10: (pa10, 10, [1, 7, 10, 11, 14, 15]),
    PA11: (pa11, 11, [1, 2, 7, 9, 10, 12, 15]),
    PA12: (pa12, 12, [1, 7, 9, 10, 15]),
    PA13: (pa13, 13, [0, 1, 10, 15], super::Debugger),
    PA14: (pa14, 14, [0, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 5, 6, 8, 9, 11, 13, 15], super::Debugger),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 7, 10, 11, 12, 15]),
    PB1: (pb1, 1, [1, 2, 3, 6, 7, 10, 11, 14, 15]),
    PB2: (pb2, 2, [0, 1, 4, 6, 15]),
    PB3: (pb3, 3, [0, 1, 5, 6, 7, 11, 13, 15], super::Debugger),
    PB4: (pb4, 4, [0, 2, 5, 6, 7, 8, 9, 11, 13, 14, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB6: (pb6, 6, [1, 2, 3, 4, 6, 7, 9, 12, 13, 14, 15]),
    PB7: (pb7, 7, [1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB8: (pb8, 8, [2, 4, 6, 9, 11, 12, 13, 14, 15]),
    PB9: (pb9, 9, [1, 2, 4, 5, 6, 9, 11, 12, 13, 14, 15]),
    PB10: (pb10, 10, [1, 4, 5, 6, 7, 8, 10, 11, 12, 13, 15]),
    PB11: (pb11, 11, [1, 4, 6, 7, 8, 10, 11, 12, 15]),
    PB12: (pb12, 12, [1, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB13: (pb13, 13, [1, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14, 15]),
    PB14: (pb14, 14, [1, 3, 4, 5, 6, 7, 9, 11, 12, 13, 14, 15]),
    PB15: (pb15, 15, [0, 1, 3, 5, 6, 9, 11, 12, 13, 14, 15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 4, 6, 8, 11, 14, 15]),
    PC1: (pc1, 1, [1, 4, 6, 8, 11, 15]),
    PC2: (pc2, 2, [1, 5, 6, 11, 15]),
    PC3: (pc3, 3, [1, 5, 11, 13, 14, 15]),
    PC4: (pc4, 4, [7, 11, 15]),
    PC5: (pc5, 5, [7, 11, 15]),
    PC6: (pc6, 6, [2, 3, 6, 9, 11, 12, 13, 15]),
    PC7: (pc7, 7, [2, 3, 6, 9, 11, 12, 13, 15]),
    PC8: (pc8, 8, [2, 3, 9, 11, 12, 15]),
    PC9: (pc9, 9, [1, 2, 3, 9, 10, 11, 12, 13, 14, 15]),
    PC10: (pc10, 10, [6, 7, 8, 9, 11, 12, 13, 15]),
    PC11: (pc11, 11, [6, 7, 8, 9, 11, 12, 13, 15]),
    PC12: (pc12, 12, [6, 7, 8, 9, 11, 12, 13, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [5, 6, 9, 12, 15]),
    PD1: (pd1, 1, [5, 6, 9, 12, 15]),
    PD2: (pd2, 2, [2, 7, 8, 9, 11, 12, 15]),
    PD3: (pd3, 3, [5, 6, 7, 12, 15]),
    PD4: (pd4, 4, [5, 6, 7, 12, 15]),
    PD5: (pd5, 5, [7, 12, 15]),
    PD6: (pd6, 6, [6, 7, 12, 13, 15]),
    PD7: (pd7, 7, [6, 7, 12, 15]),
    PD8: (pd8, 8, [7, 11, 12, 15]),
    PD9: (pd9, 9, [7, 11, 12, 13, 15]),
    PD10: (pd10, 10, [7, 9, 11, 12, 13, 15]),
    PD11: (pd11, 11, [7, 9, 11, 12, 13, 14, 15]),
    PD12: (pd12, 12, [2, 7, 9, 11, 12, 13, 14, 15]),
    PD13: (pd13, 13, [2, 9, 11, 12, 14, 15]),
    PD14: (pd14, 14, [2, 11, 12, 15]),
    PD15: (pd15, 15, [2, 11, 12, 15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 11, 12, 14, 15]),
    PE1: (pe1, 1, [11, 12, 14, 15]),
    PE2: (pe2, 2, [0, 2, 9, 11, 12, 13, 15]),
    PE3: (pe3, 3, [0, 2, 9, 11, 12, 13, 15]),
    PE4: (pe4, 4, [0, 2, 6, 9, 12, 13, 15]),
    PE5: (pe5, 5, [0, 2, 6, 9, 12, 13, 15]),
    PE6: (pe6, 6, [0, 2, 12, 13, 15]),
    PE7: (pe7, 7, [1, 6, 12, 13, 15]),
    PE8: (pe8, 8, [1, 6, 12, 13, 15]),
    PE9: (pe9, 9, [1, 6, 12, 13, 15]),
    PE10: (pe10, 10, [1, 6, 9, 10, 12, 13, 15]),
    PE11: (pe11, 11, [1, 6, 9, 10, 12, 15]),
    PE12: (pe12, 12, [1, 5, 6, 9, 10, 12, 15]),
    PE13: (pe13, 13, [1, 5, 6, 9, 10, 12, 15]),
    PE14: (pe14, 14, [1, 2, 3, 5, 10, 12, 15]),
    PE15: (pe15, 15, [1, 3, 5, 10, 12, 15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12, 15]),
    PF1: (pf1, 1, [4, 12, 15]),
    PF2: (pf2, 2, [4, 12, 15]),
    PF3: (pf3, 3, [12, 15]),
    PF4: (pf4, 4, [12, 15]),
    PF5: (pf5, 5, [12, 15]),
    PF6: (pf6, 6, [1, 2, 13, 15]),
    PF7: (pf7, 7, [2, 13, 15]),
    PF8: (pf8, 8, [2, 13, 15]),
    PF9: (pf9, 9, [2, 13, 14, 15]),
    PF10: (pf10, 10, [14, 15]),
    PF11: (pf11, 11, [15]),
    PF12: (pf12, 12, [12, 15]),
    PF13: (pf13, 13, [6, 12, 15]),
    PF14: (pf14, 14, [6, 9, 12, 15]),
    PF15: (pf15, 15, [9, 12, 15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [9, 12, 15]),
    PG1: (pg1, 1, [9, 12, 15]),
    PG2: (pg2, 2, [5, 12, 13, 15]),
    PG3: (pg3, 3, [5, 12, 13, 15]),
    PG4: (pg4, 4, [5, 12, 13, 15]),
    PG5: (pg5, 5, [5, 8, 12, 13, 15]),
    PG6: (pg6, 6, [4, 8, 15]),
    PG7: (pg7, 7, [4, 8, 12, 15]),
    PG8: (pg8, 8, [4, 8, 15]),
    PG9: (pg9, 9, [6, 7, 12, 13, 14, 15]),
    PG10: (pg10, 10, [1, 6, 7, 12, 13, 14, 15]),
    PG11: (pg11, 11, [1, 6, 7, 13, 14, 15]),
    PG12: (pg12, 12, [1, 6, 7, 12, 13, 15]),
    PG13: (pg13, 13, [4, 7, 12, 15]),
    PG14: (pg14, 14, [4, 12, 15]),
    PG15: (pg15, 15, [1, 4, 15]),
]);

#[cfg(feature = "gpio-l47x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
]);
/*
#[cfg(feature = "gpio-l47x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI8: (pi8, 8, []),
]);
*/

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, [1, 2, 3, 7, 8, 13, 14, 15]),
    PA1: (pa1, 1, [1, 2, 4, 5, 7, 8, 11, 14, 15]),
    PA2: (pa2, 2, [1, 2, 7, 8, 10, 11, 13, 14, 15]),
    PA3: (pa3, 3, [1, 2, 7, 8, 10, 11, 13, 14, 15]),
    PA4: (pa4, 4, [5, 6, 7, 10, 13, 14, 15]),
    PA5: (pa5, 5, [1, 2, 3, 5, 14, 15]),
    PA6: (pa6, 6, [1, 2, 3, 4, 5, 7, 8, 10, 11, 12, 13, 14, 15]),
    PA7: (pa7, 7, [1, 2, 3, 4, 5, 10, 11, 14, 15]),
    PA8: (pa8, 8, [0, 1, 7, 10, 11, 12, 13, 14, 15]),
    PA9: (pa9, 9, [1, 3, 5, 7, 11, 13, 14, 15]),
    PA10: (pa10, 10, [1, 5, 7, 10, 11, 13, 14, 15]),
    PA11: (pa11, 11, [1, 2, 5, 7, 9, 10, 12, 15]),
    PA12: (pa12, 12, [1, 5, 7, 9, 10, 15]),
    PA13: (pa13, 13, [0, 1, 10, 12, 13, 15], super::Debugger),
    PA14: (pa14, 14, [0, 1, 4, 5, 10, 12, 13, 15], super::Debugger),
    PA15: (pa15, 15, [0, 1, 2, 3, 5, 6, 7, 8, 9, 11, 12, 13, 15], super::Debugger),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, [1, 2, 3, 5, 7, 10, 11, 12, 13, 15]),
    PB1: (pb1, 1, [1, 2, 3, 6, 7, 8, 10, 11, 14, 15]),
    PB2: (pb2, 2, [0, 1, 4, 6, 11, 15]),
    PB3: (pb3, 3, [0, 1, 5, 6, 7, 10, 11, 13, 15], super::Debugger),
    PB4: (pb4, 4, [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15], super::Debugger),
    PB5: (pb5, 5, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB6: (pb6, 6, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15]),
    PB7: (pb7, 7, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB8: (pb8, 8, [2, 4, 6, 9, 10, 11, 12, 13, 14, 15]),
    PB9: (pb9, 9, [1, 2, 4, 5, 6, 9, 10, 11, 12, 13, 14, 15]),
    PB10: (pb10, 10, [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
    PB11: (pb11, 11, [1, 3, 4, 6, 7, 8, 10, 11, 12, 15]),
    PB12: (pb12, 12, [1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB13: (pb13, 13, [1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
    PB14: (pb14, 14, [1, 3, 4, 5, 6, 7, 9, 11, 12, 13, 14, 15]),
    PB15: (pb15, 15, [0, 1, 3, 5, 6, 9, 11, 12, 13, 14, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOC, gpioc, PC, 'C', PCn, [
    PC0: (pc0, 0, [1, 2, 4, 6, 8, 11, 14, 15]),
    PC1: (pc1, 1, [0, 1, 2, 3, 4, 6, 8, 10, 11, 13, 15]),
    PC2: (pc2, 2, [1, 5, 6, 10, 11, 15]),
    PC3: (pc3, 3, [1, 5, 10, 11, 13, 14, 15]),
    PC4: (pc4, 4, [7, 10, 11, 15]),
    PC5: (pc5, 5, [7, 11, 15]),
    PC6: (pc6, 6, [2, 3, 6, 9, 10, 11, 12, 13, 15]),
    PC7: (pc7, 7, [2, 3, 6, 9, 10, 11, 12, 13, 15]),
    PC8: (pc8, 8, [2, 3, 9, 10, 11, 12, 15]),
    PC9: (pc9, 9, [1, 2, 3, 4, 6, 9, 10, 11, 12, 13, 14, 15]),
    PC10: (pc10, 10, [0, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
    PC11: (pc11, 11, [5, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
    PC12: (pc12, 12, [0, 6, 7, 8, 9, 10, 11, 12, 13, 15]),
    PC13: (pc13, 13, [15]),
    PC14: (pc14, 14, [15]),
    PC15: (pc15, 15, [15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOD, gpiod, PD, 'D', PDn, [
    PD0: (pd0, 0, [5, 6, 9, 12, 15]),
    PD1: (pd1, 1, [5, 6, 9, 12, 15]),
    PD2: (pd2, 2, [0, 2, 7, 8, 9, 10, 11, 12, 15]),
    PD3: (pd3, 3, [3, 4, 5, 6, 7, 10, 12, 15]),
    PD4: (pd4, 4, [5, 6, 7, 10, 12, 15]),
    PD5: (pd5, 5, [7, 10, 12, 15]),
    PD6: (pd6, 6, [4, 5, 6, 7, 10, 12, 13, 15]),
    PD7: (pd7, 7, [6, 7, 10, 12, 15]),
    PD8: (pd8, 8, [7, 10, 11, 12, 15]),
    PD9: (pd9, 9, [7, 10, 11, 12, 13, 15]),
    PD10: (pd10, 10, [7, 9, 11, 12, 13, 15]),
    PD11: (pd11, 11, [4, 7, 9, 11, 12, 13, 14, 15]),
    PD12: (pd12, 12, [2, 4, 7, 9, 11, 12, 13, 14, 15]),
    PD13: (pd13, 13, [2, 4, 9, 11, 12, 14, 15]),
    PD14: (pd14, 14, [2, 11, 12, 15]),
    PD15: (pd15, 15, [2, 11, 12, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOE, gpioe, PE, 'E', PEn, [
    PE0: (pe0, 0, [2, 10, 11, 12, 14, 15]),
    PE1: (pe1, 1, [10, 11, 12, 14, 15]),
    PE2: (pe2, 2, [0, 2, 9, 11, 12, 13, 15]),
    PE3: (pe3, 3, [0, 2, 9, 11, 12, 13, 15]),
    PE4: (pe4, 4, [0, 2, 6, 9, 10, 12, 13, 15]),
    PE5: (pe5, 5, [0, 2, 6, 9, 10, 12, 13, 15]),
    PE6: (pe6, 6, [0, 2, 10, 12, 13, 15]),
    PE7: (pe7, 7, [1, 6, 12, 13, 15]),
    PE8: (pe8, 8, [1, 6, 12, 13, 15]),
    PE9: (pe9, 9, [1, 6, 12, 13, 15]),
    PE10: (pe10, 10, [1, 6, 9, 10, 12, 13, 15]),
    PE11: (pe11, 11, [1, 6, 9, 10, 12, 15]),
    PE12: (pe12, 12, [1, 5, 6, 9, 10, 12, 15]),
    PE13: (pe13, 13, [1, 5, 6, 9, 10, 12, 15]),
    PE14: (pe14, 14, [1, 2, 3, 5, 10, 12, 15]),
    PE15: (pe15, 15, [1, 3, 5, 10, 12, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOF, gpiof, PF, 'F', PFn, [
    PF0: (pf0, 0, [4, 12, 15]),
    PF1: (pf1, 1, [4, 12, 15]),
    PF2: (pf2, 2, [4, 12, 15]),
    PF3: (pf3, 3, [12, 15]),
    PF4: (pf4, 4, [12, 15]),
    PF5: (pf5, 5, [12, 15]),
    PF6: (pf6, 6, [1, 2, 10, 13, 15]),
    PF7: (pf7, 7, [2, 10, 13, 15]),
    PF8: (pf8, 8, [2, 10, 13, 15]),
    PF9: (pf9, 9, [2, 10, 13, 14, 15]),
    PF10: (pf10, 10, [3, 10, 14, 15]),
    PF11: (pf11, 11, [10, 15]),
    PF12: (pf12, 12, [12, 15]),
    PF13: (pf13, 13, [4, 6, 12, 15]),
    PF14: (pf14, 14, [4, 6, 9, 12, 15]),
    PF15: (pf15, 15, [4, 9, 12, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOG, gpiog, PG, 'G', PGn, [
    PG0: (pg0, 0, [9, 12, 15]),
    PG1: (pg1, 1, [9, 12, 15]),
    PG2: (pg2, 2, [5, 12, 13, 15]),
    PG3: (pg3, 3, [5, 12, 13, 15]),
    PG4: (pg4, 4, [5, 12, 13, 15]),
    PG5: (pg5, 5, [5, 8, 12, 13, 15]),
    PG6: (pg6, 6, [4, 8, 15]),
    PG7: (pg7, 7, [4, 8, 12, 13, 15]),
    PG8: (pg8, 8, [4, 8, 15]),
    PG9: (pg9, 9, [6, 7, 12, 13, 14, 15]),
    PG10: (pg10, 10, [1, 6, 7, 12, 13, 14, 15]),
    PG11: (pg11, 11, [1, 6, 7, 13, 14, 15]),
    PG12: (pg12, 12, [1, 6, 7, 12, 13, 15]),
    PG13: (pg13, 13, [4, 7, 12, 15]),
    PG14: (pg14, 14, [4, 12, 15]),
    PG15: (pg15, 15, [1, 4, 10, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOH, gpioh, PH, 'H', PHn, [
    PH0: (ph0, 0, [15]),
    PH1: (ph1, 1, [15]),
    PH2: (ph2, 2, [3, 15]),
    PH3: (ph3, 3, [15]),
    PH4: (ph4, 4, [4, 15]),
    PH5: (ph5, 5, [4, 10, 15]),
    PH6: (ph6, 6, [4, 10, 15]),
    PH7: (ph7, 7, [4, 10, 15]),
    PH8: (ph8, 8, [4, 10, 15]),
    PH9: (ph9, 9, [4, 10, 15]),
    PH10: (ph10, 10, [2, 10, 15]),
    PH11: (ph11, 11, [2, 10, 15]),
    PH12: (ph12, 12, [2, 10, 15]),
    PH13: (ph13, 13, [3, 9, 15]),
    PH14: (ph14, 14, [3, 10, 15]),
    PH15: (ph15, 15, [3, 10, 15]),
]);

#[cfg(feature = "gpio-l49x")]
gpio!(GPIOI, gpioi, PI, 'I', PIn, [
    PI0: (pi0, 0, [2, 5, 10, 15]),
    PI1: (pi1, 1, [5, 10, 15]),
    PI2: (pi2, 2, [3, 5, 10, 15]),
    PI3: (pi3, 3, [3, 5, 10, 15]),
    PI4: (pi4, 4, [3, 10, 15]),
    PI5: (pi5, 5, [3, 10, 15]),
    PI6: (pi6, 6, [3, 10, 15]),
    PI7: (pi7, 7, [3, 10, 15]),
    PI8: (pi8, 8, [10, 15]),
    PI9: (pi9, 9, [9, 15]),
    PI10: (pi10, 10, [15]),
    PI11: (pi11, 11, [15]),
]);
