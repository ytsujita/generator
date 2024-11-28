class UnhandledError extends Error {
  UnhandledError({required this.exception});
  final dynamic exception;
  @override
  String toString() => 'UnhandledError occurred: $exception';
}

/// sealed classを使ってエラーハンドリングを強制する
///
/// sealed classについては https://zenn.dev/never_inc_dev/articles/c11a809b9dfedf
sealed class Result<T, U extends Exception> {
  const Result();

  T unwrap() {
    switch (this) {
      case Success<T, U>(:final data):
        return data;
      case Failure<T, U>(:final exception):
        throw exception;
    }
  }
}

class Success<T, U extends Exception> extends Result<T, U> {
  const Success({required this.data});
  final T data;
}

class Failure<T, U extends Exception> extends Result<T, U> {
  const Failure({required this.exception});
  final U exception;
}
