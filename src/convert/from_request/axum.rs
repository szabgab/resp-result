use core::{future::Future, marker::Send, pin::Pin};

use axum::extract::FromRequestParts;
use futures::TryFutureExt;

use crate::{Nil, RespError, RespResult};

use super::{FromRequestFamily, MapReject, ToInner};

impl<S, T, E> FromRequestParts<S> for MapReject<T, E>
where
    S: Sync,
    E: Send + From<<T::Payload as FromRequestParts<S>>::Rejection> + RespError,
    T: FromRequestFamily<E>,
    T::Payload: FromRequestParts<S>,
{
    type Rejection = RespResult<Nil, E>;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut http::request::Parts,
        state: &'life1 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            <T::Payload as FromRequestParts<S>>::from_request_parts(parts, state)
                .map_err(|err| RespResult::Err(E::from(err)))
                .map_ok(|data| Self(data.to_inner()))
                .await
        })
    }
}
mod from_request_families {
    use axum::extract::{Extension, Form, Json, Path, Query, State};

    use crate::convert::from_request::ToInner;

    impl<T> ToInner for Extension<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl<T> ToInner for Form<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl<T> ToInner for Json<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl<T> ToInner for Path<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl<T> ToInner for Query<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }

    impl<T> ToInner for State<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.0
        }
    }
}
