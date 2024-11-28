import 'package:email_validator/email_validator.dart';

extension EmailValidatorExtension on String? {
  bool get isEmail {
    if (this == null) {
      return false;
    }
    return EmailValidator.validate(this!);
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
