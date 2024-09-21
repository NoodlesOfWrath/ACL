//! translates a ```Circuit``` to a spice netlist

use crate::translator::Circuit;

struct SpiceTranslator {
    circuit: Circuit,
    spice: String,
}
