


pub type Source = SignalSource;
//pub type DelegateType = DelegateX<void, Params, HashMap<K,V>>

pub struct SignalSource {
    pub delegate_list: Set<DelegateType>
}

impl SignalSource {
    pub fn empty(&self) -> bool {
        return self.delegate_list.is_empty()
    }

    bool empty() const { return delegateList.empty(); }

    void clear() { delegateList.clear(); }

    void connect(DelegateType delegate) { delegateList.insert(delegate); }

    template <typename X, typename Y>
    void connect(Y* obj, void (X::*func)(Params...)) { delegateList.insert(MakeDelegate(obj, func)); }

    template <typename X, typename Y>
    void connect(Y* obj, void (X::*func)(Params...) const) { delegateList.insert(MakeDelegate(obj, func)); }

    void disconnect(DelegateType delegate) { delegateList.erase(delegate); }

    template <typename X, typename Y>
    void disconnect(Y* obj, void (X::*func)(Params...)) { delegateList.erase(MakeDelegate(obj, func)); }

    template <typename X, typename Y>
    void disconnect(Y* obj, void (X::*func)(Params...) const) { delegateList.erase(MakeDelegate(obj, func)); }


}

