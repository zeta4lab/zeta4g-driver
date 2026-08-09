// Copyright 2026 Zeta4Lab
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 라우팅 모듈
//!
//! 클러스터 환경에서 자동 라우팅을 지원합니다.
//!
//! # 개요
//!
//! 라우팅 드라이버는 `zeta4g://` 스킴을 사용하여 클러스터에 연결합니다.
//! 읽기 트랜잭션은 팔로워로, 쓰기 트랜잭션은 리더로 자동 라우팅됩니다.
//!
//! # 예시
//!
//! ```ignore
//! use zeta4g::driver::routing::{RoutingDriver, RoutingPolicy};
//! use zeta4g::driver::{AuthToken, SessionConfig, AccessMode};
//!
//! // 라우팅 드라이버 생성
//! let driver = RoutingDriver::new(
//!     "zeta4g://server1:7687,server2:7687",
//!     AuthToken::basic("zeta4g", "password")
//! )?;
//!
//! // 읽기 세션 (팔로워로 라우팅)
//! let read_session = driver.session(
//!     SessionConfig::builder()
//!         .with_default_access_mode(AccessMode::Read)
//!         .build()
//! )?;
//!
//! // 쓰기 세션 (리더로 라우팅)
//! let write_session = driver.session(
//!     SessionConfig::builder()
//!         .with_default_access_mode(AccessMode::Write)
//!         .build()
//! )?;
//!
//! driver.close().await?;
//! ```

mod driver;
mod policy;
mod table;

pub use driver::{is_routing_uri, parse_routing_uri, RoutingDriver, RoutingDriverMetrics};
pub use policy::{RoutingPolicy, ServerSelector};
pub use table::{RoutingTable, ServerRole};
