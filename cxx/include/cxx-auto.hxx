#pragma once

#include "rust/cxx.h"
#include "sys/types.h"

#include <compare>
#include <concepts>
#include <iterator>
#include <limits>
#include <memory>
#include <ranges>
#include <sstream>
#include <type_traits>

// NOLINTBEGIN(google-runtime-int)
using c_char = char;
using c_int = int;
using c_long = long;
using c_longlong = long long;
using c_schar = signed char;
using c_short = short;
using c_uchar = unsigned char;
using c_uint = unsigned int;
using c_ulong = unsigned long;
using c_ulonglong = unsigned long long;
using c_ushort = unsigned short;
using c_void = void;
using c_off_t = off_t;
using c_time_t = time_t;
// NOLINTEND(google-runtime-int)

namespace cxx_auto::detection {
template<typename T, typename... U>
concept same_as_any_of = (std::same_as<T, U> or ...);

template<typename T>
concept has_operator_equal = requires(T const& lhs, T const& rhs) { //
  {
    lhs == rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_not_equal = requires(T const& lhs, T const& rhs) { //
  {
    lhs != rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than = requires(T const& lhs, T const& rhs) { //
  {
    lhs < rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_less_than_or_equal = requires(T const& lhs, T const& rhs) { //
  {
    lhs <= rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than = requires(T const& lhs, T const& rhs) { //
  {
    lhs > rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_greater_than_or_equal = requires(T const& lhs, T const& rhs) { //
  {
    lhs >= rhs
  } -> std::same_as<bool>;
};

template<typename T>
concept has_operator_three_way_comparison = requires(T const& lhs, T const& rhs) { //
  requires same_as_any_of<decltype(lhs <=> rhs), std::partial_ordering, std::strong_ordering>;
};

template<typename T>
concept is_std_hashable = requires(T const& arg) { //
  {
    std::hash<T>{}(arg)
  } -> std::same_as<std::size_t>;
};

template<typename T>
concept has_operator_std_string = requires(T const& arg) { //
  {
    arg.operator std::string()
  } -> std::same_as<std::string>;
};

template<typename T>
concept has_operator_std_string_view = requires(T const& arg) { //
  {
    arg.operator std::string_view()
  } -> std::same_as<std::string_view>;
};

template<typename T>
concept has_to_string = requires(T const& arg) { //
  {
    std::to_string(arg)
  } -> std::same_as<std::string>;
};

template<typename T>
concept has_operator_ostream_left_shift = requires(T const& arg, std::ostream& os) { //
  {
    os << arg
  } -> std::same_as<std::ostream&>;
};

template<typename T, typename It>
concept is_constructible_from_iterator = requires(It first, It last) { //
  requires std::input_iterator<It>;
  {
    T{ first, last }
  } -> std::same_as<T>;
};

template<typename T>
concept is_iterable = std::ranges::range<T>;

template<typename T, typename V>
concept is_input_iterable = requires { //
  requires std::ranges::input_range<T>;
  requires std::same_as<std::iter_value_t<std::ranges::iterator_t<T>>, std::remove_reference_t<V>>;
};

template<typename T>
concept is_input_copy_iterator = requires { //
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_lvalue_reference_t<std::iter_value_t<T>>>;
};

template<typename T>
concept is_input_move_iterator = requires { //
  requires std::input_iterator<T>;
  requires std::same_as<std::iter_reference_t<T>, std::add_rvalue_reference_t<std::iter_value_t<T>>>;
};
} // namespace cxx_auto::detection

namespace cxx_auto {
template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_align() noexcept -> size_t
{
  return alignof(T);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_abi_size() noexcept -> size_t
{
  return sizeof(T);
}

template<typename T, typename... Args>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_constructible() noexcept -> bool
{
  return std::is_constructible_v<T, Args...>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_default_constructible() noexcept -> bool
{
  return std::is_default_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_copy_constructible() noexcept -> bool
{
  return std::is_copy_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_move_constructible() noexcept -> bool
{
  return std::is_move_constructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_destructible() noexcept -> bool
{
  return std::is_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_copyable() noexcept -> bool
{
  return std::is_trivially_copyable_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_movable() noexcept -> bool
{
  return std::is_trivially_move_constructible_v<T> and std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_trivially_destructible() noexcept -> bool
{
  return std::is_trivially_destructible_v<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_equality_comparable() noexcept -> bool
{
  return std::equality_comparable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_equal() noexcept -> bool
{
  return detection::has_operator_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_not_equal() noexcept -> bool
{
  return detection::has_operator_not_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_less_than() noexcept -> bool
{
  return detection::has_operator_less_than<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_less_than_or_equal() noexcept -> bool
{
  return detection::has_operator_less_than_or_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_greater_than() noexcept -> bool
{
  return detection::has_operator_greater_than<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_greater_than_or_equal() noexcept -> bool
{
  return detection::has_operator_greater_than_or_equal<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_has_operator_three_way_comparison() noexcept -> bool
{
  return detection::has_operator_three_way_comparison<T> or
         (not detection::has_operator_three_way_comparison<T> and detection::has_operator_less_than<T> and
          detection::has_operator_equal<T>);
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_partially_ordered() noexcept -> bool
{
  return cxx_has_operator_three_way_comparison<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_totally_ordered() noexcept -> bool
{
  return std::totally_ordered<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_hashable() noexcept -> bool
{
  return detection::is_std_hashable<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_debuggable() noexcept -> bool
{
  return detection::has_operator_ostream_left_shift<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
cxx_is_displayable() noexcept -> bool
{
  return detection::has_to_string<T> or detection::has_operator_std_string<T> or
         detection::has_operator_std_string_view<T>;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_cxx_extern_type_trivial() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_unpin() noexcept -> bool
{
  return cxx_is_trivially_movable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_send() noexcept -> bool
{
  return false;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_sync() noexcept -> bool
{
  return false;
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_drop() noexcept -> bool
{
  return cxx_is_destructible<T>() and not cxx_is_trivially_destructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_copy() noexcept -> bool
{
  return cxx_is_trivially_copyable<T>() and cxx_is_trivially_movable<T>() and not rust_should_impl_drop<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_default() noexcept -> bool
{
  return cxx_is_default_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_moveref_copy_new() noexcept -> bool
{
  return cxx_is_copy_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_moveref_move_new() noexcept -> bool
{
  return cxx_is_move_constructible<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_eq() noexcept -> bool
{
  return cxx_is_equality_comparable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_partial_eq() noexcept -> bool
{
  return cxx_has_operator_equal<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_partial_ord() noexcept -> bool
{
  return cxx_has_operator_three_way_comparison<T>() or
         (cxx_has_operator_less_than<T>() and cxx_has_operator_equal<T>());
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_ord() noexcept -> bool
{
  return cxx_is_totally_ordered<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_hash() noexcept -> bool
{
  return cxx_is_hashable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_debug() noexcept -> bool
{
  return cxx_is_debuggable<T>();
}

template<typename T>
[[nodiscard]] [[gnu::always_inline]] [[gnu::const]]
constexpr static inline auto
rust_should_impl_display() noexcept -> bool
{
  return cxx_is_displayable<T>();
}

} // namespace cxx_auto

namespace cxx_auto {
template<typename T, typename... Args>
requires(cxx_is_constructible<T, Args...>())
[[gnu::always_inline]]
static inline auto
cxx_placement_new(T* This [[clang::lifetimebound]], Args&&... args) noexcept -> void
{
  new (This) T(std::forward<Args>(args)...);
}

template<typename T>
requires(cxx_is_default_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void
{
  cxx_placement_new(This);
}

template<typename T>
requires(cxx_is_copy_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_copy_new(T* This [[clang::lifetimebound]], T const& that [[clang::lifetimebound]]) noexcept -> void
requires std::is_lvalue_reference_v<decltype(that)>
{
  new (This) T(that);
}

template<typename T>
requires(cxx_is_move_constructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_move_new(T* This [[clang::lifetimebound]], T&& that [[clang::lifetimebound]]) noexcept -> void
requires std::is_rvalue_reference_v<decltype(that)>
{
  new (This) T(std::forward<T>(that));
}

template<typename T>
requires(cxx_is_destructible<T>())
[[gnu::always_inline]]
static inline auto
cxx_destruct(T* This [[clang::lifetimebound]]) -> void
{
  std::destroy_at(This);
}

template<typename T>
requires(cxx_has_operator_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_equal(T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]) noexcept -> bool
{
  return (This == That);
}

template<typename T>
requires(cxx_has_operator_not_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_not_equal(T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]) noexcept -> bool
{
  return (This != That);
}

template<typename T>
requires(cxx_has_operator_less_than<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_less_than(T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]) noexcept -> bool
{
  return (This < That);
}

template<typename T>
requires(cxx_has_operator_less_than_or_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_less_than_or_equal(T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]) noexcept
  -> bool
{
  return (This <= That);
}

template<typename T>
requires(cxx_has_operator_greater_than<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_greater_than(T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]) noexcept
  -> bool
{
  return (This > That);
}

template<typename T>
requires(cxx_has_operator_greater_than_or_equal<T>())
[[gnu::always_inline]]
static inline auto
cxx_operator_greater_than_or_equal(
  T const& This [[clang::lifetimebound]],
  T const& That [[clang::lifetimebound]]
) noexcept -> bool
{
  return (This >= That);
}

template<typename T>
requires(detection::has_operator_three_way_comparison<T>)
[[gnu::always_inline]]
static inline auto
cxx_operator_three_way_comparison(
  T const& This [[clang::lifetimebound]],
  T const& That [[clang::lifetimebound]]
) noexcept -> int8_t
{
  auto result = (This <=> That);
  if (result < 0) {
    return -1;
  } else if (result > 0) { // NOLINT(llvm-else-after-return, readability-else-after-return)
    return 1;
  } else if (result == 0) {
    return 0;
  } else {
    return std::numeric_limits<int8_t>::max();
  }
}

template<typename T>
requires(not detection::has_operator_three_way_comparison<T> and detection::has_operator_less_than<T> and detection::has_operator_equal<T>)
[[gnu::always_inline]]
static inline auto
cxx_operator_three_way_comparison(
  T const& This [[clang::lifetimebound]],
  T const& That [[clang::lifetimebound]]
) noexcept -> int8_t
{
  auto le = (This < That);
  auto eq = (This == That);

  if (le and not eq) {
    return -1;
  } else if (not le and not eq) { // NOLINT(llvm-else-after-return, readability-else-after-return)
    return 1;
  } else if (not le and eq) {
    return 0;
  } else {
    return std::numeric_limits<int8_t>::max();
  }
}

template<typename T>
requires(detection::is_std_hashable<T>)
[[gnu::always_inline]]
static inline auto
cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t
{
  return std::hash<T>{}(This);
}

template<typename T>
requires(detection::has_operator_ostream_left_shift<T>)
[[gnu::always_inline]]
static inline auto
cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  std::ostringstream os;
  os << This;
  return rust::String::lossy(os.str());
}

template<typename T>
requires(detection::has_to_string<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  return rust::String::lossy(std::to_string(This));
}

template<typename T>
requires(not detection::has_to_string<T> and detection::has_operator_std_string<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  return rust::String::lossy(This.operator std::string());
}

// FIXME: optimize this to use `&str` instead of `String`
template<typename T>
requires(not detection::has_to_string<T> and not detection::has_operator_std_string<T> and detection::has_operator_std_string_view<T>)
[[gnu::always_inline]]
static inline auto
cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::String
{
  return rust::String::lossy(std::string{ This.operator std::string_view() });
}

}; // namespace cxx_auto

// NOLINTBEGIN(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
#define CXX_AUTO_PRELUDE_SELECT_MACRO(_0, _1, _2, NAME, ...) NAME

#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_1(TY_CON) using TyCon = TY_CON;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_2(TY_CON, TY_ARG0)                                                              \
  template<typename TyArg0>                                                                                            \
  using TyCon = TY_CON<TyArg0>;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                                     \
  template<typename TyArg0, typename TyArg1>                                                                           \
  using TyCon = TY_CON<TyArg0, TyArg1>;
#define CXX_AUTO_PRELUDE_TY_CON_DEFINE(...)                                                                            \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__, CXX_AUTO_PRELUDE_TY_CON_DEFINE_3, CXX_AUTO_PRELUDE_TY_CON_DEFINE_2, CXX_AUTO_PRELUDE_TY_CON_DEFINE_1, \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_1(TY_CON)
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_2(TY_CON, TY_ARG0) using TyArg0 = TY_ARG0;
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1)                                                    \
  using TyArg0 = TY_ARG0;                                                                                              \
  using TyArg1 = TY_ARG1;
#define CXX_AUTO_PRELUDE_TY_ARGS_DEFINE(...)                                                                           \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__,                                                                                                       \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_3,                                                                                 \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_2,                                                                                 \
    CXX_AUTO_PRELUDE_TY_ARGS_DEFINE_1,                                                                                 \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE_TYPE_DEFINE_1(TY_CON) using Self = TY_CON;
#define CXX_AUTO_PRELUDE_TYPE_DEFINE_2(TY_CON, TY_ARG0) using Self = TY_CON<TY_ARG0>;
#define CXX_AUTO_PRELUDE_TYPE_DEFINE_3(TY_CON, TY_ARG0, TY_ARG1) using Self = TY_CON<TY_ARG0, TY_ARG1>;

#define CXX_AUTO_PRELUDE_TYPE_DEFINE(...)                                                                              \
  CXX_AUTO_PRELUDE_SELECT_MACRO(                                                                                       \
    __VA_ARGS__, CXX_AUTO_PRELUDE_TYPE_DEFINE_3, CXX_AUTO_PRELUDE_TYPE_DEFINE_2, CXX_AUTO_PRELUDE_TYPE_DEFINE_1,       \
  )                                                                                                                    \
  (__VA_ARGS__)

#define CXX_AUTO_PRELUDE(CXX_NAME, ...)                                                                                \
  CXX_AUTO_PRELUDE_TY_CON_DEFINE(__VA_ARGS__)                                                                          \
  CXX_AUTO_PRELUDE_TY_ARGS_DEFINE(__VA_ARGS__)                                                                         \
  CXX_AUTO_PRELUDE_TYPE_DEFINE(__VA_ARGS__)                                                                            \
  using CXX_NAME = Self;                                                                                               \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_abi_align() noexcept -> size_t                                                      \
  {                                                                                                                    \
    return ::cxx_auto::cxx_abi_align<Self>();                                                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_abi_size() noexcept -> size_t                                                       \
  {                                                                                                                    \
    return ::cxx_auto::cxx_abi_size<Self>();                                                                           \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_default_constructible() noexcept -> bool                                         \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_default_constructible<Self>();                                                           \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_copy_constructible() noexcept -> bool                                            \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_copy_constructible<Self>();                                                              \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_move_constructible() noexcept -> bool                                            \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_move_constructible<Self>();                                                              \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_destructible() noexcept -> bool                                                  \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_destructible<Self>();                                                                    \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_copyable() noexcept -> bool                                            \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_trivially_copyable<Self>();                                                              \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_movable() noexcept -> bool                                             \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_trivially_movable<Self>();                                                               \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_trivially_destructible() noexcept -> bool                                        \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_trivially_destructible<Self>();                                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_equality_comparable() noexcept -> bool                                           \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_equality_comparable<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_equal() noexcept -> bool                                               \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_equal<Self>();                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_not_equal() noexcept -> bool                                           \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_not_equal<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_less_than() noexcept -> bool                                           \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_less_than<Self>();                                                             \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_less_than_or_equal() noexcept -> bool                                  \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_less_than_or_equal<Self>();                                                    \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_greater_than() noexcept -> bool                                        \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_greater_than<Self>();                                                          \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_greater_than_or_equal() noexcept -> bool                               \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_greater_than_or_equal<Self>();                                                 \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_has_operator_three_way_comparison() noexcept -> bool                                \
  {                                                                                                                    \
    return ::cxx_auto::cxx_has_operator_three_way_comparison<Self>();                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_partially_ordered() noexcept -> bool                                             \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_partially_ordered<Self>();                                                               \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_totally_ordered() noexcept -> bool                                               \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_totally_ordered<Self>();                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_hashable() noexcept -> bool                                                      \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_hashable<Self>();                                                                        \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_debuggable() noexcept -> bool                                                    \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_debuggable<Self>();                                                                      \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto cxx_is_displayable() noexcept -> bool                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_is_displayable<Self>();                                                                     \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_cxx_extern_type_trivial() noexcept -> bool                             \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_cxx_extern_type_trivial<Self>();                                               \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_unpin() noexcept -> bool                                               \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_unpin<Self>();                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_send() noexcept -> bool                                                \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_send<Self>();                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_sync() noexcept -> bool                                                \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_sync<Self>();                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_drop() noexcept -> bool                                                \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_drop<Self>();                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_copy() noexcept -> bool                                                \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_copy<Self>();                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_default() noexcept -> bool                                             \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_default<Self>();                                                               \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_moveref_copy_new() noexcept -> bool                                    \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_moveref_copy_new<Self>();                                                      \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_moveref_move_new() noexcept -> bool                                    \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_moveref_move_new<Self>();                                                      \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_eq() noexcept -> bool                                                  \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_eq<Self>();                                                                    \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_partial_eq() noexcept -> bool                                          \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_partial_eq<Self>();                                                            \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_partial_ord() noexcept -> bool                                         \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_partial_ord<Self>();                                                           \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_ord() noexcept -> bool                                                 \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_ord<Self>();                                                                   \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_hash() noexcept -> bool                                                \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_hash<Self>();                                                                  \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_debug() noexcept -> bool                                               \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_debug<Self>();                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  [[nodiscard]] [[gnu::always_inline]] [[gnu::const]]                                                                  \
  constexpr static inline auto rust_should_impl_display() noexcept -> bool                                             \
  {                                                                                                                    \
    return ::cxx_auto::rust_should_impl_display<Self>();                                                               \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_default_constructible<T>())                                  \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_default_new(T* This [[clang::lifetimebound]]) noexcept -> void                                \
  {                                                                                                                    \
    return ::cxx_auto::cxx_default_new(This);                                                                          \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_copy_constructible<T>())                                     \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_copy_new(T* This [[clang::lifetimebound]], T const& that [[clang::lifetimebound]]) noexcept   \
    -> void                                                                                                            \
  {                                                                                                                    \
    return ::cxx_auto::cxx_copy_new(This, that);                                                                       \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_move_constructible<T>())                                     \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_move_new(T* This [[clang::lifetimebound]], T* that [[clang::lifetimebound]]) noexcept -> void \
  {                                                                                                                    \
    /* NOLINTNEXTLINE(hicpp-move-const-arg, performance-move-const-arg) */                                             \
    return ::cxx_auto::cxx_move_new(This, ::std::move(*that));                                                         \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_destructible<T>())                                           \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_destruct(T* This [[clang::lifetimebound]]) noexcept -> void                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_destruct(This);                                                                             \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_equal<T>())                                        \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_equal(                                                                               \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_equal(This, That);                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_not_equal<T>())                                    \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_not_equal(                                                                           \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_not_equal(This, That);                                                             \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_less_than<T>())                                    \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_less_than(                                                                           \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_less_than(This, That);                                                             \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_less_than_or_equal<T>())                           \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_less_than_or_equal(                                                                  \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_less_than_or_equal(This, That);                                                    \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_greater_than<T>())                                 \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_greater_than(                                                                        \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_greater_than(This, That);                                                          \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_greater_than_or_equal<T>())                        \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_greater_than_or_equal(                                                               \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> bool                                                                                                   \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_greater_than_or_equal(This, That);                                                 \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_has_operator_three_way_comparison<T>())                         \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_operator_three_way_comparison(                                                                \
    T const& This [[clang::lifetimebound]], T const& That [[clang::lifetimebound]]                                     \
  ) noexcept -> int8_t                                                                                                 \
  {                                                                                                                    \
    return ::cxx_auto::cxx_operator_three_way_comparison(This, That);                                                  \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_hashable<T>())                                               \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_hash(T const& This [[clang::lifetimebound]]) noexcept -> size_t                               \
  {                                                                                                                    \
    return ::cxx_auto::cxx_hash(This);                                                                                 \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_debuggable<T>())                                             \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_debug(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                        \
  {                                                                                                                    \
    return ::cxx_auto::cxx_debug(This);                                                                                \
  }                                                                                                                    \
                                                                                                                       \
  template<typename T>                                                                                                 \
  requires(::std::same_as<T, Self> and ::cxx_auto::cxx_is_displayable<T>())                                            \
  [[gnu::always_inline]]                                                                                               \
  static inline auto cxx_display(T const& This [[clang::lifetimebound]]) noexcept -> rust::string                      \
  {                                                                                                                    \
    return ::cxx_auto::cxx_display(This);                                                                              \
  }

// NOLINTEND(cppcoreguidelines-macro-usage, bugprone-macro-parentheses)
