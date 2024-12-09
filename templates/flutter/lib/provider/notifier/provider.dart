import 'package:hooks_riverpod/hooks_riverpod.dart';
{% match state_path %}
  {% when Some with (val) %}
import '{{ val }}';
  {% when None %}
{% endmatch %}


{% match provider_type %}

  {%- when Provider -%}
final {{ name|camel }}Provider = Provider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ state_name }}>
    {%- endmatch -%}
    ((final ref) { throw UnimplementedError(); });
  {%- when NotifierProvider -%}
final {{ name|camel }}Provider = NotifierProvider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ name|pascal }}Notifier, {{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ name|pascal }}Notifier, {{ state_name }}>
    {%- endmatch -%}
    ({{ name|pascal }}Notifier.new);
class {{name|pascal}}Notifier extends {% if auto_dispose -%}
AutoDispose
{%- endif -%}
  {%- match family_type -%}
    {%- when Some with (val) -%}
      FamilyNotifier<{{ state_name }}, {{ val }}>
    {%- when None -%}
      Notifier<{{ state_name }}>
  {%- endmatch -%}
    {
      @override
      {{ state_name }} build(
        {%- match family_type -%}
          {%- when Some with (val) -%}
            {{ val }} arg
          {%- when None -%}
        {%- endmatch -%}
      ) {
        // TODO: implement
        throw UnimplementedError();
      }
    }
  {% when FutureProvider %}
final {{ name|camel }}Provider = FutureProvider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ state_name }}>
    {%- endmatch -%}
    ((final ref) { throw UnimplementedError(); });
  {% when StreamProvider %}
final {{ name|camel }}Provider = StreamProvider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ state_name }}>
    {%- endmatch -%}
    ((final ref) { throw UnimplementedError(); });
  {% when AsyncNotifierProvider %}
final {{ name|camel }}Provider = AsyncNotifierProvider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ name|pascal }}AsyncNotifier, {{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ name|pascal }}AsyncNotifier, {{ state_name }}>
    {%- endmatch -%}
    ({{ name|pascal }}Notifier.new);
class {{ name|pascal }}AsyncNotifier extends {% if auto_dispose -%}
AutoDispose
{%- endif -%}
  {%- match family_type -%}
    {%- when Some with (val) -%}
      FamilyAsyncNotifier<{{ state_name }}, {{ val }}>
    {%- when None -%}
      AsyncNotifier<{{ state_name }}>
  {%- endmatch -%}
    {
      @override
      Future<{{ state_name }}> build(
        {%- match family_type -%}
          {%- when Some with (val) -%}
            {{ val }} arg
          {%- when None -%}
        {%- endmatch -%}
      ) {
        // TODO: implement
        throw UnimplementedError();
      }
    }
  {% when StreamNotifierProvider %}
final {{ name|camel }}Provider = StreamNotifierProvider
    {%- if auto_dispose -%}.autoDispose{%- endif -%}
    {%- match family_type -%}
      {%- when Some with (val) -%}
        .family<{{ name|pascal }}StreamNotifier, {{ state_name }}, {{ val }}>
      {%- when None -%}
        <{{ name|pascal }}StreamNotifier, {{ state_name }}>
    {%- endmatch -%}
    ({{ name|pascal }}Notifier.new);
class {{ name|pascal }}StreamNotifier extends {% if auto_dispose -%}
AutoDispose
{%- endif -%}
  {%- match family_type -%}
    {%- when Some with (val) -%}
      FamilyStreamNotifier<{{ state_name }}, {{ val }}>
    {%- when None -%}
      StreamNotifier<{{ state_name }}>
  {%- endmatch -%}
    {
      @override
      Stream<{{ state_name }}> build(
        {%- match family_type -%}
          {%- when Some with (val) -%}
            {{ val }} arg
          {%- when None -%}
        {%- endmatch -%}
      ) {
        // TODO: implement
        throw UnimplementedError();
      }
    }
{% endmatch %}
