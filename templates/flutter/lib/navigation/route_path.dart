import 'package:flutter/material.dart';

{%- for route_path in route_paths -%}
{%- match route_path.dir_name -%}
{%- when Some with (dir) %}
import '../widget/page/{{ dir|snake }}/{{ route_path.name|snake }}.dart';
{%- when None -%}
import '../widget/page/{{ route_path.name|snake }}.dart';
{%- endmatch -%}
{% endfor %}
{%- for route_path in shell_route_paths -%}
{%- match route_path.dir_name -%}
{%- when Some with (dir) %}
import '../widget/page/{{ dir|snake }}/{{ route_path.name|snake }}.dart';
{%- when None -%}
import '../widget/page/{{ route_path.name|snake }}.dart';
{%- endmatch -%}
{% endfor %}

sealed class BaseRoutePath {
  const BaseRoutePath();
  Page<dynamic> buildPage();
}

sealed class ShellRoutePath<T> extends BaseRoutePath {
  const ShellRoutePath({
    required this.selectedIndex,
    required this.pathStack,
  });
  final T selectedIndex;
  final Map<T, List<BaseRoutePath>> pathStack;
}

sealed class RoutePath extends BaseRoutePath {
  const RoutePath();

  factory RoutePath.fromUri(Uri uri) {
    if (uri.path == '/' || uri.path.isEmpty) {
      return const {{ default_route_path_name }}RoutePath();
    }
    {%- for route_path in route_paths -%}
    {%- match route_path.path_reg_exp -%}
    {%- when Some with (uri) %}
    if (RegExp({{ route_path.name }}RoutePath.pathRegExp).hasMatch(uri.path)) {
      return const {{ route_path.name }}RoutePath();
    }
    {%- when None -%}
    {%- endmatch -%}
    {% endfor %}
    return const {{ default_route_path_name }}RoutePath();
  }
  Uri? get uri;
}

{% for shell_route_path in shell_route_paths -%}
enum {{ shell_route_path.name }}ShellIndex {
  {%- for enum_name in shell_route_path.shell_index_enum_names %}
  {{ enum_name|camel }},
  {%- endfor %}
  ;
}

class {{ shell_route_path.name }}ShellRoutePath extends ShellRoutePath<{{ shell_route_path.name }}ShellIndex> {
  const {{ shell_route_path.name }}ShellRoutePath({
    required super.pathStack,
    required super.selectedIndex,
  });
  final {{ shell_route_path.name }}ShellIndex selectedIndex;

  @override
  Page<dynamic> buildPage() {
    return MaterialPage(
      child: {{ shell_route_path.name }}Widget(
        selectedIndex: selectedIndex,
        pathStack: pathStack,
      ),
    );
  }
}
{%- endfor %}

{%- for route_path in route_paths %}
class {{ route_path.name }}RoutePath extends RoutePath {
  const {{ route_path.name }}RoutePath();
  {%- match route_path.path_reg_exp %}
  {% when Some with (uri) %}
  static const String pathRegExp = r'{{ uri }}';
  {%- when None -%}
  {%- endmatch %}

  @override
  Page<dynamic> buildPage() {
    return const MaterialPage(child: {{ route_path.name }}Widget());
  }

  @override
  {% match route_path.uri %}
  {%- when Some with (uri) -%}
  Uri get uri => Uri(path: '{{ uri }}');
  {%- when None -%}
  Uri? get uri => null;
  {%- endmatch %}
}
{% endfor %}

