#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt;
use std::mem;
use std::slice;
use std::str;

use cql_ffi::value::CassValueType;
use cql_ffi::row::CassRow;
use cql_ffi::types::cass_size_t;
use cql_ffi::iterator::result_iterator::ResultIterator;
use cql_bindgen::CassResult as _CassResult;

use cql_bindgen::cass_result_free;
use cql_bindgen::cass_result_row_count;
use cql_bindgen::cass_result_column_count;
use cql_bindgen::cass_result_column_name;
use cql_bindgen::cass_result_column_type;
use cql_bindgen::cass_result_first_row;
use cql_bindgen::cass_result_has_more_pages;
use cql_bindgen::cass_iterator_from_result;

pub struct CassResult(pub *const _CassResult);

impl Debug for CassResult {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        try!(write!(f, "Result row count: {:?}\n", self.row_count()));
        for row in self.iter() {
            try!(write!(f, "{:?}\n",row));
        }
        Ok(())
    }
}

impl Drop for CassResult {
    fn drop(&mut self) {unsafe{
        self.free()
    }}
}

impl CassResult {
    unsafe fn free(&mut self) {
        cass_result_free(self.0)
    }

    pub fn row_count(&self) -> u64 {unsafe{
        cass_result_row_count(self.0) as u64
    }}

    pub fn column_count(&self) -> u64 {unsafe{
        cass_result_column_count(self.0) as u64
    }}

                                    //~ result: *const CassResult, index: size_t,
                                   //~ name: *mut *const ::libc::c_char,
                                   //~ name_length: *mut size_t
    

    pub fn column_name(&self, index: cass_size_t) -> String {unsafe{
        let name = mem::zeroed();
        let name_length = mem::zeroed();
        cass_result_column_name(self.0, index, name, name_length);
        let slice = slice::from_raw_parts(name as *const u8,name_length as usize);
        str::from_utf8(slice).unwrap().to_string()
    }}

    pub fn column_type(&self, index: cass_size_t) -> CassValueType {unsafe{
        CassValueType::build(cass_result_column_type(self.0, index))
    }}

    pub fn first_row(&self) -> Option<CassRow> {unsafe{
        match self.row_count() {
            0 => None,
            _ => Some(CassRow(cass_result_first_row(self.0)))
        }
    }}

    pub fn has_more_pages(&self) -> bool {unsafe{
        if cass_result_has_more_pages(self.0) > 0 {true} else {false}
    }}
    
    pub fn iter(&self) -> ResultIterator {unsafe{
        ResultIterator(cass_iterator_from_result(self.0))
    }}

}
