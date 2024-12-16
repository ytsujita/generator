{% for use_case_name in use_case_names %}
pub mod {{ use_case_name|snake }}_use_case;
{% endfor %}
