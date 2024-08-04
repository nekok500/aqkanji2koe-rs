use std::ffi::{c_void, CStr, CString};

use thiserror::Error;

pub mod ffi;

pub type Result<T> = std::result::Result<T, AqKanji2KoeError>;

#[derive(Error, Debug)]
pub enum AqKanji2KoeError {
    #[error("other error")]
    Other,
    #[error("input text too long")]
    InputTooLong,
    #[error("no system dictionary data specified")]
    NoSystemDictionary,
    #[error("invaild char code")]
    InvaildCharCode,
    #[error("invaild system dictionary: {0}")]
    InvaildSystemDictionary(i32),
    #[error("invaild user dictionary: {0}")]
    InvaildUserDictionary(i32),
    #[error("invaild license key")]
    InvaildLicenseKey,
    #[error("unknown error: {0}")]
    Unknown(i32),
}

impl From<i32> for AqKanji2KoeError {
    fn from(code: i32) -> Self {
        match code {
            100 => Self::Other,
            105 => Self::InputTooLong,
            106 => Self::NoSystemDictionary,
            107 => Self::InvaildCharCode,
            200..=299 => Self::InvaildSystemDictionary(code),
            300..=399 => Self::InvaildUserDictionary(code),
            _ => AqKanji2KoeError::Unknown(code),
        }
    }
}

pub struct AqKanji2Koe(*const c_void);

impl AqKanji2Koe {
    /// AqKanji2Koeのインスタンスを作成します。
    /// ReleaseはDrop時に自動で実行されます。
    ///
    /// # Arguments
    /// * `dic` -  システム辞書のパス
    pub fn create(dic: &str) -> Result<AqKanji2Koe> {
        use std::ffi::CString;

        let error: i32 = 0;
        let dic = CString::new(dic).unwrap();
        let handle: *const c_void =
            unsafe { ffi::AqKanji2Koe_Create(dic.as_ptr(), &error as *const i32) };

        if handle.is_null() {
            return Err(AqKanji2KoeError::from(error));
        }

        Ok(AqKanji2Koe(handle))
    }

    /// 入力を音声記号列に変換します。
    /// 評価版は「ナ行、マ行」の音韻がすべて「ヌ」になる制限があります。
    ///
    /// # Arguments
    /// * `kanji` - 変換する文字列
    pub fn convert(self, kanji: &str) -> Result<String> {
        use std::ffi::CString;

        let buffer_size = kanji.len() * 2 + 256;
        let koe = vec![0; buffer_size];
        let kanji = CString::new(kanji).unwrap();

        let error: i32 = unsafe {
            ffi::AqKanji2Koe_Convert(self.0, kanji.as_ptr(), koe.as_ptr(), buffer_size as i32)
        };

        if error != 0 {
            return Err(AqKanji2KoeError::from(error));
        }

        let koe = unsafe { CStr::from_ptr(koe.as_ptr()) };

        Ok(String::from(koe.to_str().unwrap()))
    }

    /// ライセンスキーを設定します。
    ///
    /// # Arguments
    /// * `key` - ライセンスキー
    pub fn set_dev_key(key: &str) -> Result<()> {
        let key = CString::new(key).unwrap();
        let error: i32 = unsafe { ffi::AqKanji2Koe_SetDevKey(key.as_ptr()) };

        if error != 0 {
            return Err(AqKanji2KoeError::InvaildLicenseKey);
        }

        Ok(())
    }
}

impl Drop for AqKanji2Koe {
    fn drop(&mut self) {
        unsafe { ffi::AqKanji2Koe_Release(self.0) }
    }
}

#[cfg(test)]
mod tests {

    use crate::AqKanji2Koe;

    #[test]
    fn test_crate_aqkanji2koe() {
        let kanji = AqKanji2Koe::create("../aq_dic").unwrap();
        let koe = kanji.convert("こんばんは").unwrap();
        println!("{}", koe);

        assert_eq!(koe, "コンバンワ'");
    }
}
