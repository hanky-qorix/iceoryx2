// Copyright (c) 2023 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Example
//!
//! ```
//! use iceoryx2::prelude::*;
//! use iceoryx2::service::port_factory::publisher::UnableToDeliverStrategy;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let service_name = ServiceName::new("My/Funk/ServiceName")?;
//! let pubsub = zero_copy::Service::new(&service_name)
//!     .publish_subscribe()
//!     .open_or_create::<u64>()?;
//!
//! let subscriber = pubsub.subscriber()
//!                     .create()?;
//!
//! # Ok(())
//! # }
//! ```

use std::fmt::Debug;

use iceoryx2_bb_log::fail;

use crate::{
    port::{
        port_identifiers::{UniquePublisherId, UniqueSubscriberId},
        subscriber::{Subscriber, SubscriberCreateError},
        DegrationAction, DegrationCallback,
    },
    service,
};

use super::publish_subscribe::PortFactory;

#[derive(Debug)]
pub(crate) struct SubscriberConfig {
    pub(crate) degration_callback: Option<DegrationCallback<'static>>,
}

/// Factory to create a new [`Subscriber`] port/endpoint for
/// [`MessagingPattern::PublishSubscribe`](crate::service::messaging_pattern::MessagingPattern::PublishSubscribe) based
/// communication.
#[derive(Debug)]
pub struct PortFactorySubscriber<'factory, Service: service::Service, MessageType: Debug> {
    config: SubscriberConfig,
    pub(crate) factory: &'factory PortFactory<Service, MessageType>,
}

impl<'factory, Service: service::Service, MessageType: Debug>
    PortFactorySubscriber<'factory, Service, MessageType>
{
    pub(crate) fn new(factory: &'factory PortFactory<Service, MessageType>) -> Self {
        Self {
            config: SubscriberConfig {
                degration_callback: None,
            },
            factory,
        }
    }

    /// Sets the [`DegrationCallback`] of the [`Subscriber`]. Whenever a connection to a
    /// [`crate::port::subscriber::Subscriber`] is corrupted or it seems to be dead, this callback
    /// is called and depending on the returned [`DegrationAction`] measures will be taken.
    pub fn set_degration_callback<
        F: Fn(
                service::static_config::StaticConfig,
                UniquePublisherId,
                UniqueSubscriberId,
            ) -> DegrationAction
            + 'static,
    >(
        mut self,
        callback: Option<F>,
    ) -> Self {
        match callback {
            Some(c) => self.config.degration_callback = Some(DegrationCallback::new(c)),
            None => self.config.degration_callback = None,
        }

        self
    }

    /// Creates a new [`Subscriber`] or returns a [`SubscriberCreateError`] on failure.
    pub fn create(self) -> Result<Subscriber<Service, MessageType>, SubscriberCreateError> {
        let origin = format!("{:?}", self);
        Ok(
            fail!(from origin, when Subscriber::new(&self.factory.service, self.factory.service.state().static_config.publish_subscribe(), self.config),
                "Failed to create new Subscriber port."),
        )
    }
}
