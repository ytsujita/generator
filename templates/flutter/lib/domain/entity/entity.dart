{%- if dart_class.is_immutable -%}
import 'package:flutter/foundation.dart';
{% endif %}
{%- match dart_class.fields -%}
  {%- when Some with (fields) -%}
    {%- for field in fields -%}
      {%- match field.dart_type -%}
        {% when DartType::RefClass with (val) %}
          import 'package:{{ application_name }}/{{ val.path }}';
        {% when _ %}
      {%- endmatch -%}
    {%- endfor -%}
  {%- when None -%}
{%- endmatch %}

{% if dart_class.is_immutable -%}
@immutable
{%- endif %}
class {{ dart_class.name|pascal }} {
  {% if dart_class.is_immutable -%}const{%- endif %} {{ dart_class.name }}(
    {%- match dart_class.fields %}
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

  {% match dart_class.fields %}
    {%- when Some with (val) -%}
      {%- for field in val -%}
        {%- if field.is_final -%}final {% endif -%}
        {{ field.dart_type }}
        {%- if field.nullable -%}
        ?
        {%- endif %} {{ field.name|camel }};
      {%- endfor -%}
    {%- when None -%}
  {%- endmatch %}
}

