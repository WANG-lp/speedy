use {
    crate::{Context, Readable, Reader, Writable, Writer},
    bytes::{Bytes, BytesMut},
};

impl<'a, C> Readable<'a, C> for Bytes
where
    C: Context,
{
    #[inline]
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let length = crate::private::read_length(reader)?;
        // TODO: This can be more efficient if we directly read into the `Bytes`.
        let value = reader.read_vec(length)?;
        Ok(value.into())
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        4
    }
}

impl<C> Writable<C> for Bytes
where
    C: Context,
{
    #[inline]
    fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
        self.as_ref().write_to(writer)
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Writable::<C>::bytes_needed(self.as_ref())
    }
}

impl<'a, C> Readable<'a, C> for BytesMut
where
    C: Context,
{
    #[inline]
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let length = crate::private::read_length(reader)?;
        // TODO: This can be more efficient if we directly read into the `Bytes`.
        let value = reader.read_vec(length)?;
        Ok(Bytes::from(value).into())
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        4
    }
}

impl<C> Writable<C> for BytesMut
where
    C: Context,
{
    #[inline]
    fn write_to<T: ?Sized + Writer<C>>(&self, writer: &mut T) -> Result<(), C::Error> {
        self.as_ref().write_to(writer)
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Writable::<C>::bytes_needed(self.as_ref())
    }
}
