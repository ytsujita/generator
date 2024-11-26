/// リストのインデックスを安全に取得するためのextension
extension IndexSafe<T> on List<T> {
  T? safeIndex(int index) {
    if (index < 0 || index >= length) {
      return null;
    }
    return this[index];
  }
}
