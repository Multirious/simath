use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn map<F, N>(self, with: F) -> Vec2<N>
    where
        F: FnOnce(T, T) -> (N, N),
    {
        let v = with(self.x, self.y);
        Vec2::new(v.0, v.1)
    }

    pub fn map_all<F, N>(self, mut with: F) -> Vec2<N>
    where
        F: FnMut(T) -> N,
    {
        let x = with(self.x);
        let y = with(self.y);
        Vec2::new(x, y)
    }
}

impl<T: Copy> Vec2<T> {
    pub const fn splat(value: T) -> Vec2<T> {
        Vec2::new(value, value)
    }
}

impl<T: ConstZero> Vec2<T> {
    pub const ZERO: Vec2<T> = Vec2 {
        x: T::ZERO,
        y: T::ZERO,
    };
}

impl<T: ConstOne> Vec2<T> {
    pub const ONE: Vec2<T> = Vec2 {
        x: T::ONE,
        y: T::ONE,
    };
}

macro_rules! ops_impl {
(@self $trait_name:ident $fn_name:ident $operator:tt) => {
    impl<A: $trait_name<Output = B>, B> $trait_name for Vec2<A> {
        type Output = Vec2<B>;

        fn $fn_name(self, rhs: Self) -> Self::Output {
            Vec2::new(self.x $operator rhs.x, self.y $operator rhs.y)
        }
    }
};
(@self @assign $trait_name:ident $fn_name:ident $operator:tt) => {

    impl<T: $trait_name<T>> $trait_name for Vec2<T> {
        fn $fn_name(&mut self, rhs: Self) {
            self.x $operator rhs.x;
            self.y $operator rhs.y;
        }
    }
};

(@scalar $trait_name:ident $fn_name:ident $operator:tt) => {
    impl<A: $trait_name<Output = B> + Copy, B> $trait_name<A> for Vec2<A> {
        type Output = Vec2<B>;

        fn $fn_name(self, rhs: A) -> Self::Output {
            Vec2::new(self.x $operator rhs, self.y $operator rhs)
        }
    }
};
(@scalar @assign $trait_name:ident $fn_name:ident $operator:tt) => {

    impl<T: $trait_name<T> + Copy> $trait_name<T> for Vec2<T> {
        fn $fn_name(&mut self, rhs: T) {
            self.x $operator rhs;
            self.y $operator rhs;
        }
    }
};
}

ops_impl! {@self Add add +}
ops_impl! {@self Sub sub -}
ops_impl! {@self Mul mul *}
ops_impl! {@self Div div /}
ops_impl! {@self Rem rem %}
ops_impl! {@self BitAnd bitand &}
ops_impl! {@self BitOr bitor |}
ops_impl! {@self BitXor bitxor ^}
ops_impl! {@self Shl shl <<}
ops_impl! {@self Shr shr >>}

impl<A: Not<Output = B>, B> Not for Vec2<A> {
    type Output = Vec2<B>;

    fn not(self) -> Self::Output {
        Vec2::new(!self.x, !self.y)
    }
}

impl<A: Neg<Output = B>, B> Neg for Vec2<A> {
    type Output = Vec2<B>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

ops_impl! {@self @assign AddAssign add_assign +=}
ops_impl! {@self @assign SubAssign sub_assign -=}
ops_impl! {@self @assign MulAssign mul_assign *=}
ops_impl! {@self @assign DivAssign div_assign /=}
ops_impl! {@self @assign RemAssign rem_assign %=}
ops_impl! {@self @assign BitAndAssign bitand_assign &=}
ops_impl! {@self @assign BitOrAssign bitor_assign |=}
ops_impl! {@self @assign BitXorAssign bitxor_assign ^=}
ops_impl! {@self @assign ShlAssign shl_assign <<=}
ops_impl! {@self @assign ShrAssign shr_assign >>=}

ops_impl! {@scalar Mul mul *}
ops_impl! {@scalar Div div /}
ops_impl! {@scalar @assign MulAssign mul_assign *=}
ops_impl! {@scalar @assign DivAssign div_assign /=}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(value: Vec2<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2::new(x, y)
    }
}
