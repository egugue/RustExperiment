mod _1_thread;
mod _2_channel;
mod _3_mutex;
mod _4_sync_send_trait;

pub fn main() {
    _1_thread::main();
    // _2_channel::main();
    _3_mutex::main();
    _4_sync_send_trait::main();
}
