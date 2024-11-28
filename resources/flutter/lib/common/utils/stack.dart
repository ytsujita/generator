class Stack<T> {
  Stack() : _storage = <T>[];
  final List<T> _storage;
  void push(T val) {
    _storage.add(val);
  }

  T? pop() {
    if (_storage.isEmpty) {
      return null;
    }
    final val = _storage.removeLast();
    return val;
  }

  T? top() {
    if (_storage.isEmpty) {
      return null;
    }
    return _storage.last;
  }

  void clear() {
    _storage.clear();
  }
}
