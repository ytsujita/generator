import 'package:flutter/material.dart';

enum NotificationCategory {
  success,
  error,
  ;

  Color get color {
    switch (this) {
      case success:
        return Colors.green;
      case error:
        return Colors.red;
    }
  }

  IconData get icon {
    switch (this) {
      case success:
        return Icons.check;
      case error:
        return Icons.error;
    }
  }

  String get name {
    switch (this) {
      case success:
        return 'Success';
      case error:
        return 'Error';
    }
  }
}
