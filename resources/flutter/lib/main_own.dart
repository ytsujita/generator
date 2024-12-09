import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:url_strategy/url_strategy.dart';

import 'i18n/translations.g.dart';
import 'provider/notifier/application_name_provider.dart';
import 'provider/notifier/locale_provider.dart';
import 'provider/notifier/messenger_key_provider.dart';
import 'widget/components/my_builder.dart';
import 'widget/theme/my_theme.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  Logger.root.level = kReleaseMode ? Level.WARNING : Level.ALL;
  Logger.root.onRecord.listen((LogRecord rec) {
    debugPrint(
      '[${rec.loggerName}] ${rec.level.name}: ${rec.time}: ${rec.message}',
    );
  });
  if (kIsWeb) {
    setPathUrlStrategy();
  }
  runApp(
    ProviderScope(
      // overrides: [],
      child: TranslationProvider(child: const MyApp()),
    ),
  );
}

class MyApp extends ConsumerWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MaterialApp(
      scaffoldMessengerKey: ref.watch(messengerKeyProvider),
      builder: (context, child) {
        return MyBuilder(context: context, child: child);
      },
      debugShowCheckedModeBanner: false,
      locale: ref.watch(localeProvider).flutterLocale,
      supportedLocales: AppLocaleUtils.supportedLocales,
      localizationsDelegates: GlobalMaterialLocalizations.delegates,
      title: ref.watch(titleProvider),
      theme: lightTheme(context),
      darkTheme: darkTheme(context),
      themeMode: ThemeMode.dark,
      home: const Scaffold(body: Center(child: Text('sample widget'))),
    );
  }
}
