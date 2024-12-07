import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:{{ application_name }}/navigation/route_path.dart';
import 'package:{{ application_name }}/widget/components/not_found_page.dart';

final {{ shell_name|camel }}ShellNavigatorKeyProvider =
    Provider.family<GlobalKey<NavigatorState>, {% match index_type -%}
  {%- when ShellIndexType::Enum with (val) -%}
    {{ shell_name|pascal }}ShellIndex
  {%- when ShellIndexType::String -%}
    String
  {%- when ShellIndexType::Int -%}
    int
{%- endmatch -%}
>((ref, index) {
  return GlobalKey<NavigatorState>();
});


class {{ shell_name|pascal }}Widget extends ConsumerWidget {
  const {{ shell_name|pascal }}Widget({
    super.key,
    required this.selectedIndex,
    required this.pathStack,
  });
  final {% match index_type -%}
  {%- when ShellIndexType::Enum with (val) -%}
    {{ shell_name|pascal }}ShellIndex
  {%- when ShellIndexType::String -%}
    String
  {%- when ShellIndexType::Int -%}
    int
{%- endmatch %} selectedIndex;
  final Map<{% match index_type -%}
  {%- when ShellIndexType::Enum with (val) -%}
    {{ shell_name|pascal }}ShellIndex
  {%- when ShellIndexType::String -%}
    String
  {%- when ShellIndexType::Int -%}
    int
{%- endmatch %}, List<BaseRoutePath>> pathStack;

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

