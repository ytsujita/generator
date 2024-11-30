import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../common/utils/result.dart';
{%- match use_case_type -%}
{%- when UseCaseType::Command -%}
import '../{% for i in 0..file_nest_size -%}
../
{%- endfor -%}command_use_case_impl/{{ file_name }}';
{%- when UseCaseType::Query -%}
import '../../{% for i in 0..file_nest_size -%}
../
{%- endfor -%}infrastructure/query_use_case_impl/{{ file_name }}';
{% endmatch %}


final {{ name|camel }}UseCaseProvider =
    Provider.autoDispose<{{ name }}UseCase>(
  (final ref) => {{ name }}UseCaseImpl(),
);

abstract class {{ name }}UseCase {
{%- if is_future_call %}
  Future<Result<{{ return_type }}, {{ name }}UseCaseException>> call();
{%- else %}
  Result<{{ return_type }}, {{ name }}UseCaseException> call();
{%- endif %}
}

sealed class {{ name }}UseCaseException implements Exception {}
{% for exception in exceptions %}
class {{ exception.name }}Exception extends {{ name }}UseCaseException {}
{%- endfor %}

