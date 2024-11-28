import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../common/utils/result.dart';

final {{ name|camel }}UseCaseProvider =
    Provider.autoDispose<{{ name }}UseCase>(
  (final ref) => throw UnimplementedError(),
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

