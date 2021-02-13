#![no_main]
#![no_std]

use core::borrow::Borrow;

use aux8::{entry, gpioc::RegisterBlock};

fn clear_all(gpioe: &RegisterBlock)
{
gpioe.odr.write(|w| {
        w.odr8().clear_bit();
        w.odr9().clear_bit();
        w.odr10().clear_bit();
        w.odr11().clear_bit();
        w.odr12().clear_bit();
        w.odr13().clear_bit();
        w.odr14().clear_bit();
        w.odr15().clear_bit()
});
}

fn set_all(gpioe: &RegisterBlock)
{
gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
});
}


#[entry]
fn main() -> ! {
    let (gpioa, gpioe, rcc) = aux8::init();

  
  // enable the GPIOE peripheral
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // configure the pins as outputs
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output()
    });


    gpioa.moder.modify(|_, w| 
        w.moder0().input()
    );

    unsafe{
    gpioa.pupdr.modify(|_, w| 
        w.pupdr0().bits(2)
    );

}

    loop{
        clear_all(gpioe);
 
   
    while gpioa.idr.read().idr0().bit()
    {

    }
 
    set_all(gpioe);

    while !gpioa.idr.read().idr0().bit()
    {

    }


}
}
