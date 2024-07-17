// Copyright (c) 2024 Contributors to the Eclipse Foundation
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

#ifndef IOX2_SERVICE_HPP
#define IOX2_SERVICE_HPP

#include "callback_progression.hpp"
#include "config.hpp"
#include "iox/assertions_addendum.hpp"
#include "iox/expected.hpp"
#include "iox/function.hpp"
#include "iox/optional.hpp"
#include "iox2/service_details.hpp"
#include "messaging_pattern.hpp"
#include "service_name.hpp"
#include "service_type.hpp"

#include <cstdint>

namespace iox2 {
enum class ServiceDetailsError : uint8_t {
    /// The underlying static [`Service`] information could not be opened.
    FailedToOpenStaticServiceInfo,
    /// The underlying static [`Service`] information could not be read.
    FailedToReadStaticServiceInfo,
    /// The underlying static [`Service`] information could not be deserialized.
    /// Can be caused by
    /// version mismatch or a corrupted file.
    FailedToDeserializeStaticServiceInfo,
    /// Required [`Service`] resources are not available or corrupted.
    ServiceInInconsistentState,
    /// The [`Service`] was created with a different iceoryx2 version.
    VersionMismatch,
    /// Errors that indicate either an implementation issue or a wrongly
    /// configured system.
    InternalError,
    /// The [`NodeState`] could not be acquired.
    FailedToAcquireNodeState,
};

enum class ServiceListError : uint8_t {
    /// The process has insufficient permissions to list all [`Service`]s.
    InsufficientPermissions,
    /// Errors that indicate either an implementation issue or a wrongly
    /// configured system.
    InternalError,
};

template <ServiceType S>
class Service {
  public:
    static auto does_exist(const ServiceName& service_name,
                           const ConfigRef config,
                           const MessagingPattern messaging_pattern) -> iox::expected<bool, ServiceDetailsError> {
        IOX_TODO();
    }

    static auto
    details(const ServiceName& service_name, const ConfigRef config, const MessagingPattern messaging_pattern)
        -> iox::expected<iox::optional<ServiceDetails<S>>, ServiceDetailsError> {
        IOX_TODO();
    }

    static auto list(const ConfigRef config, const iox::function<CallbackProgression(ServiceDetails<S>)>& callback)
        -> iox::expected<void, ServiceListError> {
        IOX_TODO();
    }
};
} // namespace iox2

#endif
