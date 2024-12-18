import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'package:{{ application_name }}/common/utils/result.dart';
{% match use_case_type %}
{%- when UseCaseType::Command -%}
import 'package:{{ application_name }}/application/command_use_case_impl/{{ file_name }}';
{%- when UseCaseType::Query -%}
import 'package:{{ application_name }}/infrastructure/query_use_case_impl/{{ file_name }}';
{% endmatch %}

final {{ name|camel }}UseCaseProvider =
    Provider.autoDispose<{{ name|pascal }}UseCase>(
  (final ref) => {{ name|pascal }}UseCaseImpl(),
);

abstract class {{ name|pascal }}UseCase {
  Result<void, {{ name|pascal }}UseCaseException> call();
}

sealed class {{ name|pascal }}UseCaseException implements Exception {}
{% for exception in exceptions %}
class {{ exception.name|pascal }}Exception extends {{ name|pascal }}UseCaseException {}
{% endfor %}

