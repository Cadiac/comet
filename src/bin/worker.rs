use comet::simulator::Simulator;

use gloo_worker::Registrable;


fn main() {
    Simulator::registrar().register();
}
