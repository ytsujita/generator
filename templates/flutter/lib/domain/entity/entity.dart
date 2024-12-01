{%- if immutable -%}
import 'package:flutter/foundation.dart';

@immutable
{% endif %}
class {{ entity_name }} {
  {% if immutable %}const{% endif %} {{ entity_name }}({
    {% for field in fields %}
    {% if field.is_final %}required{% endif %} this.{{ field.name|camel }},
    {{ field.name|camel }};
    {% endfor %}
  });

  {% for field in fields %}
  {% if field.is_final %}final {% endif %}
  {%- match field.dart_type -%}
  {% when DartType::Literal with (val) %} {{ val }} 
  {% when DartType::Class with (val) %} {{ val.name }} 
  {%- endmatch -%}
  {{ field.name|camel }};
  {% endfor %}
}
