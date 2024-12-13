import 'package:flutter/widgets.dart';

import '../../i18n/translations.g.dart';

extension InternationalizationBuildContext on BuildContext {
  Translations get i18n => Translations.of(this);
}
