import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../i18n/translations.g.dart';

final localeProvider = NotifierProvider.autoDispose<LocaleNotifier, AppLocale>(
  LocaleNotifier.new,
);

class LocaleNotifier extends AutoDisposeNotifier<AppLocale> {
  @override
  AppLocale build() {
    return AppLocale.jaJp;
  }

  Future<void> setLocale(AppLocale locale) async {
    state = locale;
    await LocaleSettings.setLocale(locale);
  }
}
