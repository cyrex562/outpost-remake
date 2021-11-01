use std::collections::HashMap;
use std::process::Output;

#[derive(Debug,Clone,PartialOrd, PartialEq)]
pub struct DelegateMemento
{
    // TODO:
    // DelegateMemento& operator=(const DelegateMemento& right)
    // {
    //     SetMementoFrom(right);
    //     return *this;
    // }

    // inline bool operator<(const DelegateMemento& right) { return IsLess(right); }
    // inline bool operator>(const DelegateMemento& right) { return right.IsLess(*this); }
    pub m_pthis: i32,
    pub m_pfunction: i32,
}

impl DelegateMemento {

    pub fn set_memento_from(&mut self, right: &DelegateMemento)
    {
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

    pub fn bind_static_func(&mut self, p_parent: DerivedClass, static_function_invoker: ParentInvokerSig, function_to_bind: Option<fn()>) {
        if !function_to_bind.is_some() {
            
        }
    }

        template <typename DerivedClass, typename ParentInvokerSig>
        inline void bindstaticfunc(DerivedClass* pParent, ParentInvokerSig static_function_invoker, StaticFuncPtr function_to_bind)
        {
            if (!function_to_bind)
            {
                m_pFunction = nullptr;
            }
            else
            {
                bindmemfunc(pParent, static_function_invoker);
            }
            static_assert(sizeof(GenericClass*) != sizeof(function_to_bind), "Can't use evil method");
            m_pthis = horrible_cast<GenericClass*>(function_to_bind);
        }

        inline UnvoidStaticFuncPtr GetStaticFunction() const
        {
            static_assert(sizeof(UnvoidStaticFuncPtr) != sizeof(this), "Can't use evil method");
            return horrible_cast<UnvoidStaticFuncPtr>(this);
        }


        inline bool IsEqualToStaticFuncPtr(StaticFuncPtr funcptr)
        {
            if (!funcptr)
            {
                return empty();
            }
            else
            {
                return funcptr == reinterpret_cast<StaticFuncPtr>(GetStaticFunction());
            }
        }
    };

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

#[derive(Debug,Clone,PartialOrd,PartialEq)]
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

    template <typename X, typename Y>
    inline void Bind(Y* pthis, DesiredRetType (X::*function_to_bind)(Params...))
    {
        m_Closure.bindmemfunc(static_cast<X*>(pthis), function_to_bind);
    }

    pub fn bind(&mut self, function_to_bind: fn() -> DesiredRetType {
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


    pub fn invoke_static_function(&self, params: &HashMap<String, (String, String)>) -> RetType
    {
        self.closure.get_static_function(params)
    }
}

// template <typename RetType, typename... Params>
pub struct Delegate<RetType(Params...)> : public DelegateX<RetType, Params...>
{

    using BaseType = DelegateX<RetType, Params...>;
    using SelfType = Delegate;

    Delegate() = default;

    template <typename X, typename Y>
    Delegate(Y* pthis, RetType (X::*function_to_bind)(Params...)) :
        BaseType(pthis, function_to_bind)
    {}

    template <typename X, typename Y>
    Delegate(const Y* pthis, RetType (X::*function_to_bind)(Params...) const) :
        BaseType(pthis, function_to_bind)
    {}

    Delegate(RetType (*function_to_bind)(Params...)) :
        BaseType(function_to_bind)
    {}

    Delegate& operator=(const BaseType& x)
    {
        *static_cast<BaseType*>(this) = x;
        return *this;
    }
}



template <typename X, typename Y, typename RetType, typename... Params>
DelegateX<RetType, Params...> MakeDelegate(Y* x, RetType (X::*func)(Params...))
{
    return DelegateX<RetType, Params...>(x, func);
}

template <typename X, typename Y, typename RetType, typename... Params>
DelegateX<RetType, Params...> MakeDelegate(Y* x, RetType (X::*func)(Params...) const)
{
    return DelegateX<RetType, Params...>(x, func);
}

// template <typename GenericMemFunc, typename StaticFuncPtr, typename UnvoidStaticFuncPtr>
pub struct ClosurePtr : public DelegateMemento
{

template <typename X, typename XMemFunc>
inline void bindmemfunc(X* pthis, XMemFunc function_to_bind)
{
m_pthis = SimplifyMemFunc<sizeof(function_to_bind)>::Convert(pthis, function_to_bind, m_pFunction);

m_pStaticFunction = nullptr;

}

template <typename X, typename XMemFunc>
inline void bindconstmemfunc(const X* pthis, XMemFunc function_to_bind)
{
m_pthis = SimplifyMemFunc<sizeof(function_to_bind)>::Convert(const_cast<X*>(pthis), function_to_bind, m_pFunction);

m_pStaticFunction = nullptr;

}


template <typename X, typename XMemFunc>
inline void bindmemfunc(const X* pthis, XMemFunc function_to_bind)
{
bindconstmemfunc(pthis, function_to_bind);

m_pStaticFunction = nullptr;

}


inline GenericClass* GetClosureThis() const
{
return m_pthis;
}

inline GenericMemFunc GetClosureMemPtr() const
{
return CastMemFuncPtr<GenericMemFunc>(m_pFunction);
}



template <typename DerivedClass>
inline void CopyFrom(DerivedClass* pParent, const DelegateMemento& x)
{
SetMementoFrom(x);
if (m_pStaticFunction) m_pthis = reinterpret_cast<GenericClass*>(pParent);
}

template <typename DerivedClass, typename ParentInvokerSig>
inline void bindstaticfunc(DerivedClass* pParent, ParentInvokerSig static_function_invoker, StaticFuncPtr function_to_bind)
{
if (!function_to_bind)
{
m_pFunction = nullptr;
}
else
{
bindmemfunc(pParent, static_function_invoker);
}
m_pStaticFunction = reinterpret_cast<GenericFuncPtr>(function_to_bind);
}
inline UnvoidStaticFuncPtr GetStaticFunction() const { return reinterpret_cast<UnvoidStaticFuncPtr>(m_pStaticFunction); }
