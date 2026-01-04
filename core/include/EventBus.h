#pragma once

#include <string>

namespace verseguy {

class EventBus {
public:
    static void publish(const std::string& topic, const std::string& message);
};

} // namespace verseguy