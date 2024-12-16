{% for table_name in table_names %}
pub mod {{ table_name|snake }}_table_accessor;
{% endfor %}
