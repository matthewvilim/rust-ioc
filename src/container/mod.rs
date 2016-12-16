mod impls;
mod brw_scope;

pub use self::impls::*;
pub use self::brw_scope::*;

/// A container that can resolve dependencies.
pub trait Container
    where Self: Sized
{
    fn resolve<D, R>(&self) -> R
        where R: Resolvable<Self, Dependency = D>,
              D: ResolvableFromContainer<Self>
    {
        let d = D::resolve_from_container(self);

        R::resolve(d)
    }
}

/// A trait for creating a new scope and using it within a closure.
pub trait Scope<'scope> {
    type Container: ScopedContainer<'scope>;

    fn scope<F>(&self, f: F) where F: FnOnce(Self::Container) -> ();
}

/// A scoped container that can resolve shared dependencies.
pub trait ScopedContainer<'scope>
    where Self: Container
{
    fn get_or_add<T, D>(&self) -> T
        where T: Resolvable<Self, Dependency = D> + Clone + 'static,
              D: ResolvableFromContainer<Self>;
}

/// A container that can resolve shared dependencies through borrowing.
pub trait BrwScopedContainer<'scope>
    where Self: ScopedContainer<'scope> + Container
{
    fn brw_or_add<T, D>(&self) -> &'scope T
        where T: Resolvable<Self, Dependency = D>,
              D: ResolvableFromContainer<Self>;
}

/// A dependency that can be resolved directly from the container.
///
/// This trait is different from `Resolvable` because it doesn't declare
/// the type of the dependency the implementor requires.
pub trait ResolvableFromContainer<C>
    where C: Container
{
    fn resolve_from_container(container: &C) -> Self;
}

/// A dependency that can be resolved.
pub trait Resolvable<C> {
    type Dependency;

    fn resolve(dependency: Self::Dependency) -> Self;
}

/// A basic implementation of a container.
#[derive(Default)]
pub struct BasicContainer;

impl Container for BasicContainer {}

impl<'scope> Scope<'scope> for BasicContainer {
    type Container = Scoped<'scope>;

    fn scope<F>(&self, f: F)
        where F: FnOnce(Self::Container) -> ()
    {
        let scope = Scoped::new();

        f(scope);
    }
}