import 'package:email_validator/email_validator.dart';

extension EmailValidatorExtension on String? {
  bool get isEmail {
    if (this == null) {
      return false;
    }
    return EmailValidator.validate(this!);
  }
}
