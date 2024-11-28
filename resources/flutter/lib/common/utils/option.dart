sealed class Option<T> {
  const Option();

  T? get toNullable => switch (this) {
        Some(:final value) => value,
        None() => null,
      };
}

class Some<T> extends Option<T> {
  Some(this.value);
  final T value;
}

class None<T> extends Option<T> {
  const None();
}
