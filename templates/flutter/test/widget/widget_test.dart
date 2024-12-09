import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:{{ application_name }}/widget/page/{{ file_name }}';

void main() {
  testWidgets(
    '{{ widget_name|pascal }}WidgetTest',
    (WidgetTester tester) async {
      await tester.pumpWidget(
        const {{ widget_name|pascal}}(),
      );
    },
  );
}
