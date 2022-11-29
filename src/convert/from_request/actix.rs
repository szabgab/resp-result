use std::{
    marker::PhantomPinned,
    pin::Pin,
    task::{Context, Poll}, future::Future,
};

use actix_web::FromRequest;


use crate::{FromRequestFamily, MapReject, Nil, RespError, RespResult, ToInner};

impl<T, E> FromRequest for MapReject<T, E>
where
    T: FromRequestFamily<E>,
    T::Payload: FromRequest,
    E: From<<T::Payload as FromRequest>::Error>,
    E: RespError + 'static,
{
    type Error = RespResult<Nil, E>;

    type Future = MapRejectFuture<T, E>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        MapRejectFuture {
            inner: <T::Payload as FromRequest>::from_request(req, payload),
            __pinned: PhantomPinned,
        }
    }
}

pub struct MapRejectFuture<F, E>
where
    F: FromRequestFamily<E>,
    F::Payload: FromRequest,
{
    inner: <F::Payload as FromRequest>::Future,
    __pinned: PhantomPinned,
}

impl<F, E> Future for MapRejectFuture<F, E>
where
    F: FromRequestFamily<E>,
    F::Payload: FromRequest,
    E: From<<F::Payload as FromRequest>::Error>,
{
    type Output = Result<MapReject<F, E>, RespResult<Nil, E>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().inner) };

        let Poll::Ready(ready) = fut.poll(cx) else{
            return Poll::Pending;
        };
        Poll::Ready(
            ready
                .map_err(|err| RespResult::Err(E::from(err)))
                .map(|data| MapReject(data.to_inner())),
        )
    }
}

mod from_request_families {
    use std::{clone::Clone, sync::Arc};

    use actix_web::web::{Data, Form, Header, Json, Path, Query, ReqData};

    use crate::ToInner;

    impl<T> ToInner for Data<T> {
        type Inner = Arc<T>;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }

    impl<T> ToInner for Form<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }

    impl<T> ToInner for Header<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }

    impl<T> ToInner for Json<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }
    impl<T> ToInner for Path<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }

    impl<T> ToInner for Query<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }

    impl<T: Clone> ToInner for ReqData<T> {
        type Inner = T;

        fn to_inner(self) -> Self::Inner {
            self.into_inner()
        }
    }
}