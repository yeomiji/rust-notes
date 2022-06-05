fn everyone_can_access() {}

pub mod hosting;

mod serving {
    fn take_order() {
        super::everyone_can_access();
    }

    fn serve_order() {}

    fn take_payment() {}
}