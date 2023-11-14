use color_eyre::eyre::Result;
use heed::{BytesEncode, Database, RwTxn};

pub trait HeedExt<KC, DC> {
    fn put_no_overwrite<'a>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>;

    fn put_if_absent<'a>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<Option<()>>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>;
}

impl<KC, DC, C> HeedExt<KC, DC> for Database<KC, DC, C> {
    fn put_if_absent<'a>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<Option<()>>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        match self.put_with_flags(txn, heed::PutFlags::NO_OVERWRITE, key, data) {
            Err(heed::Error::Mdb(heed::MdbError::KeyExist)) => Ok(None),
            Err(e) => Err(e.into()),
            Ok(_) => Ok(Some(())),
        }
    }

    fn put_no_overwrite<'a>(
        &self,
        txn: &mut RwTxn,
        key: &'a KC::EItem,
        data: &'a DC::EItem,
    ) -> Result<()>
    where
        KC: BytesEncode<'a>,
        DC: BytesEncode<'a>,
    {
        Ok(self.put_with_flags(txn, heed::PutFlags::NO_OVERWRITE, key, data)?)
    }
}
