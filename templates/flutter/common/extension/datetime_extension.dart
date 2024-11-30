import 'package:intl/intl.dart';

extension Format on DateTime {
  /// 任意のフォーマット変換
  String formatString({String format = 'yyyy/MM/dd HH:mm:ss'}) {
    return DateFormat(format).format(this);
  }
}
