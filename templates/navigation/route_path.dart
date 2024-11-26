import 'package:flutter/material.dart';

sealed class BaseRoutePath {
  const BaseRoutePath();
}

sealed class ShellRoutePath extends BaseRoutePath {
  const ShellRoutePath({
    required this.pathStack,
  });
  final List<BaseRoutePath> pathStack;
  Page<dynamic> buildPage();
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
  Page<dynamic> buildPage();
}

{% for shell_route_path in shell_route_paths -%}
enum {{ shell_route_path.name }}ShellIndex {
  {%- for enum_name in shell_route_path.shell_index_enum_names %}
  {{ enum_name|camel }},
  {%- endfor %}
  ;
}

class {{ shell_route_path.name }}ShellRoutePath extends ShellRoutePath {
  const {{ shell_route_path.name }}ShellRoutePath({
    required super.pathStack,
    required this.selectedIndex,
  });
  final {{ shell_route_path.name }}ShellIndex selectedIndex;

  @override
  Page<dynamic> buildPage() {
    return const MaterialPage(
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

