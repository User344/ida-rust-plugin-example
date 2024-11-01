#include "bridge.hpp"

extern "C" void bridge_msg(const std::string &format) { msg(format.c_str()); }
