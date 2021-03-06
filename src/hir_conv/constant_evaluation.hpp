/*
 */
#include <hir/hir.hpp>

namespace HIR {


struct Evaluator
{
    class Newval
    {
    public:
        virtual ::HIR::Path new_static(::HIR::TypeRef type, ::HIR::Literal value) = 0;
    };

    Span    root_span;
    StaticTraitResolve  resolve;
    Newval& nvs;

    Evaluator(const Span& sp, const ::HIR::Crate& crate, Newval& nvs):
        root_span(sp),
        resolve(crate),
        nvs( nvs )
    {
    }

    ::HIR::Literal evaluate_constant(const ::HIR::ItemPath& ip, const ::HIR::ExprPtr& expr, ::HIR::TypeRef exp, MonomorphState ms={});

    ::HIR::Literal evaluate_constant_mir(const ::HIR::ItemPath& ip, const ::MIR::Function& fcn, MonomorphState ms, ::HIR::TypeRef exp, ::std::vector< ::HIR::Literal> args);
};

} // namespace HIR

