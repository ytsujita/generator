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
      <{{ state_name }}>
    {%- match family_type -%}
      {% when Some with (val) %}
    .family
      {% when None %}
    {% endmatch %}

  {%- when NotifierProvider -%}
  {% when FutureProvider %}
  {% when StreamProvider %}
  {% when AsyncNotifierProvider %}
  {% when StreamNotifierProvider %}
{% endmatch %}
