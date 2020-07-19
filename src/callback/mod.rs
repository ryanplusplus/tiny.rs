#[cfg(test)]
mod test;

macro_rules! make_callback {
    ($Name:ident$([$($arg:ident: $Arg:ident),*])?$(,)? $(-> $Ret:ident)?) => {
        pub struct $Name<'life, $($($Arg,)*)*$($Ret)*> {
            context: *const (),
            callback: fn(*const (), $($(&$Arg,)*)*) $(-> $Ret)*,
            _life: ::core::marker::PhantomData<&'life ()>,
        }

        impl<'life, $($($Arg,)*)*$($Ret)*> $Name<'life, $($($Arg,)*)*$($Ret)*> {
            #[inline]
            pub fn new<Context>(context: &'life Context, callback: fn(&Context, $($(&$Arg,)*)*) $(-> $Ret)*) -> Self {
                Self {
                    context: unsafe { ::core::intrinsics::transmute(context) },
                    callback: unsafe { ::core::intrinsics::transmute(callback) },
                    _life: ::core::marker::PhantomData,
                }
            }

            #[inline]
            pub fn call(&self, $($($arg: &$Arg,)*)*) $(-> $Ret)* {
                (self.callback)(self.context, $($($arg,)*)*)
            }
        }

        impl<'life, $($($Arg,)*)*$($Ret)*> ::core::clone::Clone for $Name<'life, $($($Arg,)*)*$($Ret)*> {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<'life, $($($Arg,)*)*$($Ret)*> ::core::marker::Copy for $Name<'life, $($($Arg,)*)*$($Ret)*> {}
    };
}

make_callback!(Callback);
make_callback!(CallbackWithReturn -> Ret);

make_callback!(CallbackWith1Argument[a: Arg]);
make_callback!(CallbackWith1ArgumentAndReturn[a: Arg] -> Ret);

make_callback!(CallbackWith2Arguments[a: Arg1, b: Arg2]);
make_callback!(CallbackWith2ArgumentsAndReturn[a: Arg1, b: Arg2] -> Ret);

make_callback!(CallbackWith3Arguments[a: Arg1, b: Arg2, c: Arg3]);
make_callback!(CallbackWith3ArgumentsAndReturn[a: Arg1, b: Arg2, c: Arg3] -> Ret);
