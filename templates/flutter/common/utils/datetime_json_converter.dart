import 'package:intl/intl.dart';
import 'package:json_annotation/json_annotation.dart';

import '../extension/datetime_extension.dart';

class DateTimeConverter implements JsonConverter<DateTime, String> {
  const DateTimeConverter();
  static final _dateFormatter = DateFormat('yyyy-MM-dd HH:mm:ss');

  @override
  DateTime fromJson(final String json) => _dateFormatter.parseStrict(json);

  @override
  String toJson(final DateTime object) {
    return object.formatString(format: 'yyyy-MM-dd HH:mm:ss');
  }
}
