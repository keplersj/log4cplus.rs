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

    inline std::unique_ptr<std::string> tstring_to_string(const log4cplus::tstring &tstr)
    {
        std::string str = LOG4CPLUS_TSTRING_TO_STRING(tstr);

        return std::make_unique<std::string>(str);
    }
}