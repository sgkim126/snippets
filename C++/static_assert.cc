template <typename T>
void Something(T t) {
  static_assert(T == char || T == short, msg);
}
