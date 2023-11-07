use std::{io::{self, Read, Write}, hash::Hash, collections::{HashMap, HashSet}};

pub trait Transcode: Sized {
    fn to_bytes(&self, _writer: &mut dyn Write) -> io::Result<usize>;
    fn from_bytes(_reader: &mut dyn Read) -> io::Result<Self>;
}

impl Transcode for u8 {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        writer.write(&[*self])
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let mut buf = [0u8];
        reader.read_exact(&mut buf).map(|_| buf[0])
    }
}

impl Transcode for i8 {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        writer.write(&[*self as u8])
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let mut buffer = [0u8];
        reader.read(&mut buffer)?;
        Ok(buffer[0] as i8)
    }
}

macro_rules! impl_transcode_int {
    ($($type: ty),+) => {
        $(
        impl Transcode for $type {
            fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
                writer.write(&self.to_le_bytes())
            }
            fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
                let mut buf = [0u8; std::mem::size_of::<$type>()];
                reader.read_exact(&mut buf).map(|_| <$type>::from_le_bytes(buf))
            }
        }
        )+
    };
}

impl_transcode_int!(u16, u32, u64, u128, i16, i32, i64, i128);

impl Transcode for usize {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        (*self as u64).to_bytes(writer)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        u64::from_bytes(reader).map(|v| v as usize)
    }
}

impl Transcode for isize {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        (*self as i64).to_bytes(writer)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        i64::from_bytes(reader).map(|v| v as isize)
    }
}

impl<T: Transcode> Transcode for Box<[T]> {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        let mut bytes = self.len().to_bytes(writer)?;
        for value in self.iter() {
            bytes += value.to_bytes(writer)?;
        }
        Ok(bytes)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let len = usize::from_bytes(reader)?;
        let mut data = Vec::new();
        for _ in 0..len {
            data.push(T::from_bytes(reader)?);
        }
        Ok(data.into_boxed_slice())
    }
}

impl<T: Transcode> Transcode for Vec<T> {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        let mut bytes = self.len().to_bytes(writer)?;
        for value in self.iter() {
            bytes += value.to_bytes(writer)?;
        }
        Ok(bytes)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let len = usize::from_bytes(reader)?;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(T::from_bytes(reader)?);
        }
        Ok(data)
    }
}

impl<K: Transcode + Hash + Eq,V: Transcode> Transcode for HashMap<K, V> {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        let mut bytes = self.len().to_bytes(writer)?;
        for (key,value) in self.iter() {
            bytes += key.to_bytes(writer)?;
            bytes += value.to_bytes(writer)?;
        }
        Ok(bytes)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let len = usize::from_bytes(reader)?;
        let mut data = HashMap::with_capacity(len);
        for _ in 0..len {
            data.insert(K::from_bytes(reader)?, V::from_bytes(reader)?);
        }
        Ok(data)
    }
}

impl<T: Transcode + Hash + Eq> Transcode for HashSet<T> {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        let mut bytes = self.len().to_bytes(writer)?;
        for value in self.iter() {
            bytes += value.to_bytes(writer)?;
        }
        Ok(bytes)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        let len = usize::from_bytes(reader)?;
        let mut data = HashSet::with_capacity(len);
        for _ in 0..len {
            data.insert(T::from_bytes(reader)?);
        }
        Ok(data)
    }
}

macro_rules! impl_transcode_array {
    ($($num: literal),+) => {
        $(
        impl<T: Transcode + Sized> Transcode for [T; $num] where [T; $num]: for<'a >TryFrom<&'a [T]> {
            fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
                let mut bytes = 0;
                for value in self.into_iter() {
                    bytes+=value.to_bytes(writer)?;
                }
                Ok(bytes)
            }
            fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
                let mut data = vec![];
                for _ in 0..$num {
                    data.push(T::from_bytes(reader)?);
                } 
                Ok(unsafe { data.as_slice().try_into().unwrap_unchecked() })

            }
        }
        )+
    };
}

impl_transcode_array!(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32);

impl Transcode for () {
    fn to_bytes(&self, _writer: &mut dyn Write) -> io::Result<usize> {
        Ok(0)
    }

    fn from_bytes(_reader: &mut dyn Read) -> io::Result<Self> {
        Ok(())
    }
}

impl<T: Transcode> Transcode for [T; 0] {
    fn to_bytes(&self, _writer: &mut dyn Write) -> io::Result<usize> {
        Ok(0)
    }

    fn from_bytes(_reader: &mut dyn Read) -> io::Result<Self> {
        Ok([])
    }
}

impl<T: Transcode> Transcode for (T,) {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        self.0.to_bytes(writer)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        Ok((T::from_bytes(reader)?,))
    }
}

impl<T: Transcode, U: Transcode> Transcode for (T,U) {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        Ok(self.0.to_bytes(writer)?+self.1.to_bytes(writer)?)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        Ok((T::from_bytes(reader)?,U::from_bytes(reader)?))
    }
}

impl<T: Transcode, U: Transcode, V: Transcode> Transcode for (T,U,V) {
    fn to_bytes(&self, writer: &mut dyn Write) -> io::Result<usize> {
        Ok(self.0.to_bytes(writer)?+self.1.to_bytes(writer)?+self.2.to_bytes(writer)?)
    }

    fn from_bytes(reader: &mut dyn Read) -> io::Result<Self> {
        Ok((T::from_bytes(reader)?,U::from_bytes(reader)?,V::from_bytes(reader)?))
    }
}