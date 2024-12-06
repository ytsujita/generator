import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:url_strategy/url_strategy.dart';

import 'provider/notifier/application_name_provider.dart';
import 'provider/notifier/messenger_key_provider.dart';
import 'widget/components/my_builder.dart';
import 'widget/theme/my_theme.dart';

void main() async {
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
    const ProviderScope(
      // overrides: [],
      child: MyApp(),
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
      localizationsDelegates: L10n.localizationsDelegates,
      supportedLocales: L10n.supportedLocales,
      debugShowCheckedModeBanner: false,
      locale: const Locale('ja'),
      localeListResolutionCallback: (locales, supportedLocales) {
        return supportedLocales.firstWhere(
          (element) => element.languageCode == locales?.first.languageCode,
        );
      },
      title: ref.watch(titleProvider),
      theme: lightTheme(context),
      darkTheme: darkTheme(context),
      themeMode: ThemeMode.dark,
      home: const Scaffold(body: Center(child: Text('sample widget'))),
    );
  }
}
