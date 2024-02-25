/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{f32::consts::E, marker::PhantomData};

use bytes::{Buf, BufMut};
use prost::Message;
use serde::{Deserialize, Serialize};

use crate::triple::triple_wrapper::{
    TripleExceptionWrapper, TripleRequestWrapper, TripleResponseWrapper,
};

use super::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

#[derive(Debug)]
pub struct TripleWrapperCodec;

impl Default for TripleWrapperCodec {
    fn default() -> Self {
        Self
    }
}

impl Codec for TripleWrapperCodec {
    type Encode = Result<String, crate::status::Status>;

    type Decode = Vec<String>;

    type Encoder = TripleWrapperEncoder<Self::Encode>;

    type Decoder = TripleWrapperDecoder<Self::Decode>;

    fn encoder(&mut self) -> Self::Encoder {
        TripleWrapperEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        TripleWrapperDecoder(PhantomData)
    }
}

#[derive(Debug, Clone)]
pub struct TripleWrapperEncoder<T>(PhantomData<T>);

impl<T> Encoder for TripleWrapperEncoder<T> {
    type Item = Result<String, Self::Error>;

    type Error = crate::status::Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let res_data = match item {
            Ok(data) => bytes::Bytes::from(TripleResponseWrapper::get_buf(data)),
            Err(err) => bytes::Bytes::from(TripleExceptionWrapper::get_buf(err.message())),
        };
        dst.put(res_data);
        Ok(())
    }
}

pub struct TripleWrapperDecoder<U>(PhantomData<U>);

impl<U> Decoder for TripleWrapperDecoder<U> {
    type Item = Vec<String>;

    type Error = crate::status::Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let value = src.chunk().to_owned();
        let mut msg = vec![0u8; value.len()];
        src.copy_to_slice(&mut msg);
        match TripleRequestWrapper::decode(&msg[5..0]) {
            Ok(req) => Ok(Some(req.get_req())),
            Err(err) => Err(crate::status::Status::new(
                crate::status::Code::InvalidArgument,
                err.to_string(),
            )),
        }
    }
}
