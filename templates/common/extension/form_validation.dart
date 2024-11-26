import 'package:flutter/material.dart';

extension FormValidation on GlobalKey<FormState> {
  bool isValid() {
    if (currentState?.validate() ?? false) {
      return true;
    }
    return false;
  }

  bool isInvalid() {
    return !isValid();
  }
}
