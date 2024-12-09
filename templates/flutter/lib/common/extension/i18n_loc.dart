import 'package:flutter/widgets.dart';

import '../../i18n/translations.g.dart';

extension LocalizedBuildContext on BuildContext {
  Translations get i18n => Translations.of(this);
}
