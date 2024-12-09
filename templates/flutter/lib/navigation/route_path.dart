import 'package:flutter/material.dart';

{%- for route_path in route_paths -%}
{%- match route_path.dir_name -%}
{%- when Some with (dir) %}
import 'package:{{ application_name|snake }}/widget/page/{{ dir|snake }}/{{ route_path.name|snake }}.dart';
{%- when None -%}
import 'package:{{ application_name|snake }}/widget/page/{{ route_path.name|snake }}.dart';
{%- endmatch -%}
{% endfor %}
{%- for route_path in shell_route_paths -%}
{%- match route_path.dir_name -%}
{%- when Some with (dir) %}
import 'package:{{ application_name|snake }}/widget/page/{{ dir|snake }}/{{ route_path.name|snake }}.dart';
{%- when None -%}
import 'package:{{ application_name|snake }}/widget/page/{{ route_path.name|snake }}.dart';
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
  ShellRoutePath<T>? pop();
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
      {%- match route_path.fields -%}
        {%- when Some with (fields) -%}
          {%- for field in fields %}
      final {{ field.name|camel }} = uri.queryParameters['{{ field.name }}'];
          {%- endfor -%}
        {%- when None -%}
      {%- endmatch %}
      {%- match route_path.fields -%}
        {%- when Some with (fields) %}
      return {{ route_path.name|pascal }}RoutePath(
        {%- for field in fields %}
        {{ field.name|camel }}: {{ field.name|camel }}{% if !field.nullable %}!{% endif %},
        {%- endfor %}
      );
        {%- when None %}
      return const {{ route_path.name|pascal }}RoutePath();
      {%- endmatch %}
    }
        {%- when None -%}
      {%- endmatch -%}
    {% endfor %}
    return const {{ default_route_path_name }}RoutePath();
  }
  Uri? get uri;
}

{% for shell_route_path in shell_route_paths -%}
{% match shell_route_path.shell_index %}
  {% when ShellIndexType::Enum with (val) %}
enum {{ shell_route_path.name|pascal }}ShellIndex {
  {%- for enum_name in val %}
  {{ enum_name|camel }},
  {%- endfor %}
  ;
}
  {%- when _ -%}
{% endmatch %}

class {{ shell_route_path.name|pascal }}ShellRoutePath extends ShellRoutePath<
{%- match shell_route_path.shell_index -%}
  {%- when ShellIndexType::Enum with (val) -%}
    {{ shell_route_path.name|pascal }}ShellIndex
  {%- when ShellIndexType::String -%}
    String
  {%- when ShellIndexType::Int -%}
    int
{%- endmatch -%}
> {
  const {{ shell_route_path.name|pascal }}ShellRoutePath({
    required super.pathStack,
    required super.selectedIndex,
  });

  @override
  Page<dynamic> buildPage() {
    return MaterialPage(
      child: {{ shell_route_path.name }}Widget(
        selectedIndex: selectedIndex,
        pathStack: pathStack,
      ),
    );
  }

  @override
  ShellRoutePath<
{%- match shell_route_path.shell_index -%}
  {%- when ShellIndexType::Enum with (val) -%}
    {{ shell_route_path.name|pascal }}ShellIndex
  {%- when ShellIndexType::String -%}
    String
  {%- when ShellIndexType::Int -%}
    int
{%- endmatch -%}
>? pop() {
    final targetIndexStack = pathStack[selectedIndex];
    final newTargetIndexStack = targetIndexStack?..removeLast();
    if (newTargetIndexStack == null || newTargetIndexStack.isEmpty) {
      return null;
    }
    return {{ shell_route_path.name|pascal }}ShellRoutePath(
      pathStack: pathStack..update(selectedIndex, (val) => newTargetIndexStack),
      selectedIndex: selectedIndex,
    );
  }
}
{%- endfor %}
{% for route_path in route_paths %}
class {{ route_path.name|pascal }}RoutePath extends RoutePath {
  const {{ route_path.name|pascal }}RoutePath(
    {%- match route_path.fields %}
      {% when Some with (val) %}{
        {%- for field in val -%}
          {%- if field.is_final && !field.nullable %}
    required this.{{ field.name|camel }},
          {%- else %}
    this.{{ field.name|camel }}
          {%- endif -%}
        {%- endfor %}
  }
      {%- when None -%}
    {%- endmatch -%}
  );
  {%- match route_path.path_reg_exp -%}
    {%- when Some with (uri) %}
  static const String pathRegExp = r'{{ uri }}';
    {%- when None -%}
  {%- endmatch %}
  {%- match route_path.fields -%}
    {%- when Some with (fields) -%}
      {%- for field in fields %}
  {% if field.is_final %}final {% endif %}{{ field.dart_type }}{% if field.nullable %}?{% endif %} {{ field.name }};
      {%- endfor -%}
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

