import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../navigation/route_path.dart';

final {{ shell_name|camel }}ShellNavigatorKeyProvider =
    Provider<List<GlobalKey<NavigatorState>>>((ref) {
  final length = {{ shell_name|pascal }}ShellIndex.values.length;
  return [
    for (var i = 0; i < length; i++) ...{
      GlobalKey<NavigatorState>(),
    },
  ];
});


class {{ shell_name|pascal }}Widget extends ConsumerWidget {
  const {{ shell_name|pascal }}Widget({
    super.key,
    required this.selectedIndex,
    required this.pathStack,
  });
  final {{ shell_name|pascal }}ShellIndex selectedIndex;
  final List<BaseRoutePath> pathStack;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      body: HeroControllerScope(
        controller:
            MaterialApp.createMaterialHeroController(),
        child: Navigator(
          key: ref.read({{ shell_name|camel }}ShellNavigatorKeyProvider)[selectedIndex.index],
          pages: pathStack.map(
            (e) {
              return switch (e) {
                ShellRoutePath() => e.buildPage(),
                RoutePath() => e.buildPage(),
              };
            },
          ).toList(),
          onDidRemovePage: (poppedPage) {},
        ),
      ),
    );
  }
}
