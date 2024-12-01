import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../navigation/route_path.dart';
import '../../components/not_found_page.dart';

final {{ shell_name|camel }}ShellNavigatorKeyProvider =
    Provider.family<GlobalKey<NavigatorState>, int>((ref, index) {
  return GlobalKey<NavigatorState>();
});


class {{ shell_name|pascal }}Widget extends ConsumerWidget {
  const {{ shell_name|pascal }}Widget({
    super.key,
    required this.selectedIndex,
    required this.pathStack,
  });
  final {{ shell_name|pascal }}ShellIndex selectedIndex;
  final Map<{{ shell_name|pascal }}ShellIndex, List<BaseRoutePath>> pathStack;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      body: HeroControllerScope(
        controller:
            MaterialApp.createMaterialHeroController(),
        child: Navigator(
          key: ref.read({{ shell_name|camel }}ShellNavigatorKeyProvider(selectedIndex)),
          pages: pathStack[selectedIndex]?.map((e) => e.buildPage()).toList() ?? [const NotFoundPage()],
          onDidRemovePage: (poppedPage) {},
        ),
      ),
    );
  }
}

