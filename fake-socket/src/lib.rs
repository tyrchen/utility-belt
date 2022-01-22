use pin_project::pin_project;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, Stream};
use tokio::sync::mpsc;

#[pin_project]
pub struct ReceiverStream<T, E> {
    #[pin]
    inner: mpsc::UnboundedReceiver<T>,
    _error: PhantomData<E>,
}

pub struct SenderSink<T, E> {
    inner: mpsc::UnboundedSender<T>,
    _error: PhantomData<E>,
}

#[pin_project]
pub struct FakeSocket<T, E> {
    #[pin]
    sender: SenderSink<T, E>,
    #[pin]
    receiver: ReceiverStream<T, E>,
}

impl<T, E> ReceiverStream<T, E> {
    pub fn new(inner: mpsc::UnboundedReceiver<T>) -> Self {
        Self {
            inner,
            _error: PhantomData::default(),
        }
    }
}

impl<T, E> SenderSink<T, E> {
    pub fn new(inner: mpsc::UnboundedSender<T>) -> Self {
        Self {
            inner,
            _error: PhantomData::default(),
        }
    }
}

impl<T, E> FakeSocket<T, E> {
    pub fn new(rx: mpsc::UnboundedReceiver<T>, tx: mpsc::UnboundedSender<T>) -> Self {
        Self {
            sender: SenderSink::new(tx),
            receiver: ReceiverStream::new(rx),
        }
    }
}

impl<T, E> Stream for ReceiverStream<T, E> {
    type Item = Result<T, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let data = futures::ready!(self.inner.poll_recv(cx));
        Poll::Ready(Ok(data).transpose())
    }
}

impl<T, E> Stream for FakeSocket<T, E> {
    type Item = Result<T, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().receiver.poll_next(cx)
    }
}

impl<T, E> Sink<T> for SenderSink<T, E> {
    type Error = E;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let _ = self.inner.send(item);
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl<T, E> Sink<T> for FakeSocket<T, E> {
    type Error = E;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sender.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.project().sender.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sender.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().sender.poll_close(cx)
    }
}
