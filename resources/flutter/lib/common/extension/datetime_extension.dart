import 'package:intl/intl.dart';

extension DateTimeFormat on DateTime {
  /// 任意のフォーマット変換
  String formatString({final String format = 'yyyy/MM/dd HH:mm:ss'}) {
    return DateFormat(format).format(this);
  }
}
