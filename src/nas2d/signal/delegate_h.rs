use std::collections::HashMap;
use std::io::empty;
use std::process::Output;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DelegateMemento {
    // TODO:
    // DelegateMemento& operator=(const DelegateMemento& right)
    // {
    //     SetMementoFrom(right);
    //     return *this;
    // }

    // inline bool operator<(const DelegateMemento& right) { return IsLess(right); }
    // inline bool operator>(const DelegateMemento& right) { return right.IsLess(*this); }
    pub m_pthis: i32,
    pub m_pfunction: Option<i32>,
}

impl DelegateMemento {
    pub fn set_memento_from(&mut self, right: &DelegateMemento) {
        self.m_pFunction = right.m_pFunction;
        self.m_pthis = right.m_pthis;
    }

    pub fn new(right: &DelegateMemento) -> Self {
        Self {
            m_pthis: right.m_pthis,
            m_pfunction: right.m_pfunction,

        }
    }

    pub fn copy_from(&mut self, right: &DelegateMemento) {
        self.set_memento_from(right)
    }

    pub fn bind_static_func(&mut self,
                            p_parent: DerivedClass,
                            static_function_invoker: ParentInvokerSig,
                            function_to_bind: Option<fn()>,
    ) {
        if !function_to_bind.is_some() {
            self.m_pfunction = None;
        } else {
            self.bind_mem_func(p_parent, static_function_invoker);
        }
        // self.m_pthis = function_to_bind;
    }

    pub fn get_static_function(&mut self) -> fn() {
        unimplemented!()
    }

    pub fn is_equal_to_static_func_ptr(&mut self, func_ptr: Option<fn()>) -> bool {
        if func_ptr.is_none() {
            //return empty()
        } else {
            // return funcptr == reinterpret_cast<StaticFuncPtr>(GetStaticFunction());
        }
        todo!()
    }
}

// using DesiredRetType = typename detail::DefaultVoidToVoid<RetType>::type;
pub struct DesiredRetType {}

// using StaticFunctionPtr = DesiredRetType (*)(Params...);
// using UnvoidStaticFunctionPtr = RetType (*)(Params...);
// using GenericMemFn = RetType (detail::GenericClass::*)(Params...);
// using ClosureType = detail::ClosurePtr<GenericMemFn, StaticFunctionPtr, UnvoidStaticFunctionPtr>;
pub struct ClosureType {}


pub struct SafeBoolStruct {
    pub a_data_pointer_to_this_is_0_on_buggy_compilers: i32,
    pub m_nonzero: fn(),
}

type UselessTypeDef = SafeBoolStruct;
type UnspecifiedBoolType = fn() -> SafeBoolStruct;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DelegateX {
    pub closure: ClosureType,
}

impl DelegateX {
    pub fn new() -> Self {
        // clear()
        Self {
            closure: ClosureType {}
        }
    }

    pub fn new2(x: &DelegateX) -> Self {
        Self {
            closure: x.closure.clone(),
        }
    }

    // DelegateX& operator=(const DelegateX& x)
    // {
    //     m_Closure.CopyFrom(this, x.m_Closure);
    //     return *this;
    // }

    // bool operator==(const DelegateX& x) const { return m_Closure.IsEqual(x.m_Closure); }
    // bool operator!=(const DelegateX& x) const { return !m_Closure.IsEqual(x.m_Closure); }
    // bool operator<(const DelegateX& x) const { return m_Closure.IsLess(x.m_Closure); }
    // bool operator>(const DelegateX& x) const { return x.m_Closure.IsLess(m_Closure); }
    // TODO:
    // pub fn new3(Y* pthis, DesiredRetType (X::*function_to_bind)(Params...)) -> Self
    // {
    //     m_Closure.bindmemfunc(static_cast<X*>(pthis), function_to_bind);
    // }

    pub fn bind(&mut self, function_to_bind: fn() -> DesiredRetType) {
        self.closure.bind_mem_func(self, function_to_bind)
    }

    // template <typename X, typename Y>
    // DelegateX(const Y* pthis, DesiredRetType (X::*function_to_bind)(Params...) const)
    // {
    //     m_Closure.bindconstmemfunc(static_cast<const X*>(pthis), function_to_bind);
    // }

    // template <typename X, typename Y>
    // inline void Bind(const Y* pthis, DesiredRetType (X::*function_to_bind)(Params...) const)
    // {
    //     m_Closure.bindconstmemfunc(static_cast<const X*>(pthis), function_to_bind);
    // }
    pub fn bind2(&mut self, function_to_bind: fn() -> DesiredRetType) {
        self.closure.bind_const_mem_func(self, function_to_bind)
    }

    // DelegateX(DesiredRetType (*function_to_bind)(Params...))
    // {
    //     Bind(function_to_bind);
    // }

    // DelegateX& operator=(DesiredRetType (*function_to_bind)(Params...))
    // {
    //     Bind(function_to_bind);
    //     return *this;
    // }

    // inline void Bind(DesiredRetType (*function_to_bind)(Params...))
    // {
    //     m_Closure.bindstaticfunc(this, &DelegateX::InvokeStaticFunction, function_to_bind);
    // }
    pub fn bind3(&mut self, function_to_bind: fn() -> DesiredRetType) {
        self.closure.bind_static_func(self, function_to_bind)
    }

    // RetType operator()(Params... params) const
    // {
    //     return (m_Closure.GetClosureThis()->*(m_Closure.GetClosureMemPtr()))(params...);
    // }

    pub fn empty(&self) -> ClosureType {
        !self.closure.clone()
    }

    pub fn clear(&mut self) {
        self.closure.clear()
    }

    pub fn get_memento(&self) -> ClosureType {
        self.closure.clone()
    }

    pub fn set_memento(&mut self, any: &DelegateMemento) {
        self.closure = any.clone().into();
    }


    pub fn invoke_static_function(&self, params: &HashMap<String, (String, String)>) -> RetType {
        self.closure.get_static_function(params)
    }
}

// using BaseType = DelegateX<RetType, Params...>; type BaseType = DelegateX;
// using SelfType = Delegate; type SelfType = Delegate;

// template <typename RetType, typename... Params>
// <RetType(Params...)> : public DelegateX<RetType, Params...>
pub struct Delegate {}

impl Delegate {
    pub fn new() -> Self {
        Self {}
    }

    // template <typename X, typename Y>
    // Delegate(Y* pthis, RetType (X::*function_to_bind)(Params...)) :
    //     BaseType(pthis, function_to_bind)
    // {}

    // template <typename X, typename Y>
    // Delegate(const Y* pthis, RetType (X::*function_to_bind)(Params...) const) :
    //     BaseType(pthis, function_to_bind)
    // {}

    // Delegate(RetType (*function_to_bind)(Params...)) :
    //     BaseType(function_to_bind)
    // {}

    // Delegate& operator=(const BaseType& x)
    // {
    //     *static_cast<BaseType*>(this) = x;
    //     return *this;
    // }
}

// template <typename X, typename Y, typename RetType, typename... Params>
// DelegateX<RetType, Params...> MakeDelegate(Y* x, RetType (X::*func)(Params...))
// {
// return DelegateX<RetType, Params...>(x, func);
// }
pub fn make_delegate() -> DelegateX {
    DelegateX::new()
}

// template <typename X, typename Y, typename RetType, typename... Params>
// DelegateX<RetType, Params...> MakeDelegate(Y* x, RetType (X::*func)(Params...) const)
// {
//     return DelegateX<RetType, Params...>(x, func);
// }

// template <typename GenericMemFunc, typename StaticFuncPtr, typename UnvoidStaticFuncPtr>
pub struct ClosurePtr {}

impl ClosurePtr {
    pub fn bind_mem_func(&mut self, function_to_bind: fn()) {
        self.m_pthis = simplify_mem_func(self.p_this, function_to_bind, self.m_p_function);
        self.m_p_static_function = None;
    }

    pub fn get_closure_this(&self) -> GenericClass {
        self.m_p_this
    }

    pub fn ge_closure_mem_ptr(&self) -> GenericMemFunc {
        self.m_p_function
    }

    pub fn copy_from(&self, p_parent: &DerivedClass, x: &DelegateMemento) {
        set_memento_from(x);
        if self.m_p_static_function.is_some() {
            self.m_p_this = p_parent;
        }
    }

    pub fn bind_static_func(&self, p_parent: &DerivedClass, static_function_inovker: &ParentInvokerSig, function_to_bind: &StaticFuncPtr) {
        if function_to_bind.is_none() {
            self.m_p_function = None;
        } else {
            // self.bind_mem_func(p_parent, static_function_inovker);
        }

        self.m_p_static_function = function_to_bind;
    }

    pub fn get_static_function(&self) -> UnvoidStaticFuncPtr {
        self.m_p_static_function
    }
}
