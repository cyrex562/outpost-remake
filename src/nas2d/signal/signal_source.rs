use crate::nas2d::signal::delegate::make_delegate;

pub type Source = SignalSource;
//pub type DelegateType = DelegateX<void, Params, HashMap<K,V>>

pub struct SignalSource {
    pub delegate_list: Set<DelegateType>
}

impl SignalSource {
    pub fn empty(&self) -> bool {
        return self.delegate_list.is_empty()
    }

    pub fn clear(&self) {
        self.delegate_list.clear()
    }

    pub fn connect(&self, delegate: DelegateType) {
        self.insert(delegate)
    }

    pub fn connect2<T, U>(&mut self, obj: &T, func: U) {
        self.insert(make_delegate(obj, func))
    }

    pub fn disconnect(&mut self, delegate: &DelegateType) {
        self.delegate_list.erase(delegate)
    }

    pub fn disconnect2<T, U>(&mut self, obj: &T, func: U) {
        self.delegate_list.erase(make_delegate(obj, func))
    }
}

