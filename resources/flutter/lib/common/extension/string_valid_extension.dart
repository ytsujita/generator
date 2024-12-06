extension EmailValidatorExtension on String? {
  bool get isEmail {
    if (this == null) {
      return false;
    }
    final regex = RegExp(
      r'''^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$''',
    );
    return regex.hasMatch(this!);
  }
}

extension PasswordValidatorExtension on String? {
  /// 8文字以上半角英字,数字,記号を含む文字列(cognito のデフォルトの条件)かどうか判定する
  bool get isValidPassword {
    if (this == null) {
      return false;
    }
    if (this!.length < 8) {
      return false;
    }
    final regex = RegExp(
      r'''^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[\^$*.\[\]{}()?!"!@#%&/\\,><':;|_~`=+-]).*$''',
    );
    return regex.hasMatch(this!);
  }
}
