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

use dubbo_base::Url;
use std::{
    fmt::{Debug, Formatter},
    str::FromStr,
};
use tower_service::Service;

use crate::triple::transport::{connection::Connection, self};

#[derive(Clone)]
pub struct TripleInvoker {
    url: Url,
    conn: Connection,
}

impl TripleInvoker {
    pub fn new(url: Url) -> TripleInvoker {
        let uri = http::Uri::from_str(&url.to_url()).unwrap();
        Self {
            url,
            conn: Connection::new().with_host(uri),
        }
    }
}

impl Debug for TripleInvoker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.url).as_str())
    }
}

impl<B> Service<http::Request<B>> for TripleInvoker 
where
    B: http_body::Body + Unpin + Send + 'static,
    B::Error: Into<crate::Error>,
    B::Data: Send + Unpin,
{
    type Response = http::Response<crate::BoxBody>;

    type Error = crate::Error;

    type Future = crate::BoxFuture<Self::Response, Self::Error>;

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        self.conn.call(req)
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        <transport::connection::Connection as Service<http::Request<B>>>::poll_ready(&mut self.conn, cx)
    }
}

