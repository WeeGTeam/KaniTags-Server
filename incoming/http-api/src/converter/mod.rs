pub mod image;
pub mod image_list;
pub mod image_tag_converter;
pub mod import;
pub mod tag_converter;
pub mod collection_converter;
pub mod parse;

pub trait ToDomain<D> {
    fn to_domain(self) -> D;
}

pub trait TryToDomain<D> {
    type Error: std::error::Error;

    fn try_to_domain(self) -> Result<D, Self::Error>;
}

pub trait FromDomain<D> {
    fn from_domain(value: D) -> Self;
}

pub trait TryFromDomain<D>: Sized {
    type Error: std::error::Error;

    fn try_from_domain(value: D) -> Result<Self, Self::Error>;
}

// impl

impl<A, D> ToDomain<Vec<D>> for Vec<A>
where
    A: ToDomain<D>,
{
    fn to_domain(self) -> Vec<D> {
        self.into_iter().map(A::to_domain).collect()
    }
}

impl<A, D> ToDomain<Option<D>> for Option<A>
where
    A: ToDomain<D>,
{
    fn to_domain(self) -> Option<D> {
        self.map(A::to_domain)
    }
}

impl<A, D> TryToDomain<Vec<D>> for Vec<A>
where
    A: TryToDomain<D>,
{
    type Error = A::Error;

    fn try_to_domain(self) -> Result<Vec<D>, Self::Error> {
        self.into_iter().map(A::try_to_domain).collect()
    }
}

impl<'a, A, D> TryToDomain<Vec<D>> for &'a Vec<A>
where
    &'a A: TryToDomain<D>,
{
    type Error = <&'a A as TryToDomain<D>>::Error;

    fn try_to_domain(self) -> Result<Vec<D>, Self::Error> {
        self.into_iter().map(<&'a A>::try_to_domain).collect()
    }
}

impl<A, D> TryToDomain<Option<D>> for Option<A>
where
    A: TryToDomain<D>,
{
    type Error = A::Error;

    fn try_to_domain(self) -> Result<Option<D>, Self::Error> {
        self.map(A::try_to_domain).transpose()
    }
}

impl<A, D> FromDomain<Vec<D>> for Vec<A>
where
    A: FromDomain<D>,
{
    fn from_domain(value: Vec<D>) -> Self {
        value.into_iter().map(A::from_domain).collect()
    }
}

impl<A, D> FromDomain<Option<D>> for Option<A>
where
    A: FromDomain<D>,
{
    fn from_domain(value: Option<D>) -> Self {
        value.map(A::from_domain)
    }
}

impl<A, D> TryFromDomain<Vec<D>> for Vec<A>
where
    A: TryFromDomain<D>,
{
    type Error = A::Error;

    fn try_from_domain(value: Vec<D>) -> Result<Self, Self::Error> {
        value.into_iter().map(A::try_from_domain).collect()
    }
}

impl<A, D> TryFromDomain<Option<D>> for Option<A>
where
    A: TryFromDomain<D>,
{
    type Error = A::Error;

    fn try_from_domain(value: Option<D>) -> Result<Self, Self::Error> {
        value.map(A::try_from_domain).transpose()
    }
}
