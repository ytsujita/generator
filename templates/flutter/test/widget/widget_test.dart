import 'package:flutter_test/flutter_test.dart';

import 'package:{{ application_name }}/{{ file_name }}';

void main() {
  testWidgets(
    '{{ widget_name|pascal }}WidgetTest',
    (WidgetTester tester) async {
      await tester.pumpWidget(
        const {{ widget_name|pascal }}Widget(),
      );
    },
  );
}
