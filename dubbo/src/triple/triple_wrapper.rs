use bytes::BufMut;
use prost::Message;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleRequestWrapper {
    /// hessian4
    /// json
    #[prost(string, tag = "1")]
    pub serialize_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "3")]
    pub arg_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleResponseWrapper {
    #[prost(string, tag = "1")]
    pub serialize_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripleExceptionWrapper {
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub serialization: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub class_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}


impl TripleRequestWrapper {
    pub fn new(strs: Vec<String>) -> Self {
        let mut trip = TripleRequestWrapper::default();
        trip.serialize_type = "fastjson".to_string();
        trip.args = strs.iter().map(|e| e.as_bytes().to_vec()).collect();
        return trip;
    }
    pub fn get_buf(strs: Vec<String>) -> Vec<u8> {
        let mut trip = TripleRequestWrapper::default();
        trip.serialize_type = "fastjson".to_string();
        trip.args = strs.iter().map(|e| e.as_bytes().to_vec()).collect();
        return trip.encode_to_vec();
    }
    pub fn get_req(self) -> Vec<String> {
        let mut res = vec![];
        for str in self.args {
            res.push(String::from_utf8(str).unwrap());
        }
        return res;
    }
}

impl TripleResponseWrapper {
    pub fn new(strs: String) -> TripleResponseWrapper {
        let mut trip = TripleResponseWrapper::default();
        trip.serialize_type = "fastjson".to_string();
        trip.data = strs.as_bytes().to_vec();
        return trip;
    }
    pub fn get_buf(strs: String) -> Vec<u8> {
        let mut trip = TripleResponseWrapper::default();
        trip.serialize_type = "fastjson".to_string();
        trip.data = strs.as_bytes().to_vec();
        return trip.encode_to_vec();
    }
    pub fn is_empty_body(&self) -> bool {
        return self.data.starts_with("null".as_bytes());
    }
}

impl TripleExceptionWrapper {
    pub fn get_buf(strs: String) -> Vec<u8> {
        let mut trip = TripleExceptionWrapper::default();
        trip.serialization = "fastjson".to_string();
        trip.data = strs.as_bytes().to_vec();
        return trip.encode_to_vec();
    }
    pub fn get_err_info(&self) -> String {
        return serde_json::from_slice(&self.data[..]).unwrap_or("rpc error".to_owned());
    }
}