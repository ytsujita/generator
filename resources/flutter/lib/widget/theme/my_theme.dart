import 'package:flutter/material.dart';

const Color _white = Color.fromRGBO(255, 255, 255, 1);
const Color _black = Color.fromRGBO(0, 0, 0, 1);

ThemeData lightTheme(BuildContext context) {
  return ThemeData(
    useMaterial3: true,
    brightness: Brightness.light,
    fontFamily: 'NotoSansJP',
    snackBarTheme: const SnackBarThemeData(
      backgroundColor: _black,
      contentTextStyle: TextStyle(color: _white),
      showCloseIcon: true,
      closeIconColor: _black,
    ),
    scrollbarTheme: ScrollbarThemeData(
      thumbVisibility: WidgetStateProperty.all(true),
      trackVisibility: WidgetStateProperty.all(true),
    ),
  );
}

ThemeData darkTheme(BuildContext context) {
  return ThemeData(
    useMaterial3: true,
    brightness: Brightness.dark,
    fontFamily: 'NotoSansJP',
    snackBarTheme: const SnackBarThemeData(
      backgroundColor: _black,
      contentTextStyle: TextStyle(color: _white),
      showCloseIcon: true,
      closeIconColor: _white,
    ),
    scrollbarTheme: ScrollbarThemeData(
      thumbVisibility: WidgetStateProperty.all(true),
      trackVisibility: WidgetStateProperty.all(true),
    ),
  );
}
