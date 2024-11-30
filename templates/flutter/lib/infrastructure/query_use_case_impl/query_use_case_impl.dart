import 'package:flutter/foundation.dart';

import '../../{% for i in 0..file_nest_size -%}
../
{%- endfor -%}common/utils/result.dart';
import '../../{% for i in 0..file_nest_size -%}
../
{%- endfor -%}application/use_case/{{ file_name }}';


abstract class {{ name }}UseCaseImpl extends {{ name }}SampleUseCaes {
{%- if is_future_call %}
  Future<Result<{{ return_type }}, {{ name }}UseCaseException>> call() {
{%- else %}
  Result<{{ return_type }}, {{ name }}UseCaseException> call() {
{%- endif %}
    try {
      throw UnimplementedError();
    } on Exception {
      throw UnimplementedError();
      // return Failure(exception: UnknownException());
    } catch (e) {
      if (kDebugMode) {
        print(e);
        rethrow;
      }
      throw UnimplementedError();
      // return Failure(exception: UnknownException());
    }
  }
}

