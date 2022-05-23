// ==================================================================================
// = NAS2D
// = Copyright © 2008 - 2020 New Age Software
// ==================================================================================
// = NAS2D is distributed under the terms of the zlib license. You are free to copy,
// = modify and distribute the software under the terms of the zlib license.
// =
// = Acknowledgment of your use of NAS2D is appreciated but is not required.
// ==================================================================================

use crate::nas2d::signal::signal_source::SignalSource;

// using SignalType = SignalSource<Params...>;
type SignalType = SignalSource;
// using DelegateType = typename SignalType::DelegateType;

// template <typename... Params>
pub struct SignalConnection
{
    // No copy/move construction/assignment
    // This class is designed to handle a parent object's connection to a signal
    // The delegate parameter is likely tied to the address of the parent object
    // When parent object is copied/moved, a new updated connection must be formed
    // Disable copy/move to remove default copy/move from parent objects
    pub m_signal_source: SignalType,
    pub m_delegate: DelegateType,
}

impl SignalConnection {
    // SignalConnection(const SignalConnection&) = delete;
    // SignalConnection(SignalConnection&&) = delete;
    // SignalConnection& operator=(const SignalConnection&) = delete;
    // SignalConnection& operator=(SignalConnection&&) = delete;
    // SignalConnection(SignalType& signalSource, DelegateType delegate) :
    // mSignalSource{signalSource}, mDelegate{delegate}
    // {
    //     mSignalSource.connect(mDelegate);
    // }
    pub fn new(signal_source: &SignalType, delegate: DelegateType) -> Self {
        Self {
            m_signal_source: SignalSource { delegate_list: () },
            m_delegate: ()
        }
    }

// ~SignalConnection()
// {
// mSignalSource.disconnect(mDelegate);
// }
}

