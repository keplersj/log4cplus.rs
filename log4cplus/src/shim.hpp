#pragma once

#include <memory>
#include "log4cplus/tstring.h"
#include "log4cplus/logger.h"
#include <string>

namespace log4cplus
{
    template <typename T, typename... Args>
    std::unique_ptr<T> construct_unique(Args... args)
    {
        // C++14
        return std::make_unique<T>(args...);
        // C++11
        // return std::unique_ptr<T>(new T(std::forward<Args>(args)...));
    }

    // `cxx` Doesn't currently support static class members, so we wrap it and expose it for easy Rust consumption
    inline std::unique_ptr<Logger> Logger_getInstance(const log4cplus::tstring &name)
    {
        Logger logger = log4cplus::Logger::getInstance(name);

        // C++14
        return std::make_unique<Logger>(logger);
        // C++11
        // return std::unique_ptr<Logger>(new Logger(logger));
    }

    inline std::unique_ptr<log4cplus::tstring> string_to_tstring(const std::string &str)
    {
        log4cplus::tstring tstr = LOG4CPLUS_STRING_TO_TSTRING(str);

        return std::make_unique<log4cplus::tstring>(tstr);
    }

    inline std::unique_ptr<log4cplus::tstring> cstr_to_tstring(char const *str)
    {
        log4cplus::tstring tstr = LOG4CPLUS_C_STR_TO_TSTRING(str);

        return std::make_unique<log4cplus::tstring>(tstr);
    }
}