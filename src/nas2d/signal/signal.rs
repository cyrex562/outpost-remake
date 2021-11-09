

/**
 * Signal with preset number of parameters
 *
 * See https://github.com/lairworks/nas2d-core/wiki/Signal-&-Slots for usage documentation.
 */
// template <typename... Params>
// class Signal : public SignalSource<Params...>
pub struct Signal {}

impl Signal {
    pub fn emit(&self, params: &Params) {
        for delegate in self.delegate_list {
            delegate(params)
        }
    }
}
