pub mod angle;
pub mod mpls;
pub mod plist;
pub mod stream_clip;

pub trait FromBinary
where
    Self: Sized + Default,
{
    fn read_from_binary<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let size_of_struct = std::mem::size_of::<Self>();

        let mut buf: Vec<u8> = vec![0u8; size_of_struct];
        reader.read_exact(buf.as_mut_slice())?;
        let mut obj: Self = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
        obj.sanitize();
        Ok(obj)
    }

    fn sanitize(&mut self);
}
