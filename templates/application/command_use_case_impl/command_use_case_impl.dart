import 'package:flutter/foundation.dart';

import '../../../common/utils/result.dart';
import '../../use_case/auth/refresh_sign_in_user_use_case.dart';

class {{ name }}UseCaseImpl extends {{ name }}UseCase {
  {{ name }}UseCaseImpl();

  @override
{% if is_future_call %}
  Future<Result<{{ return_type }}, {{ name }}UseCaseException>> call() async {
{%- else %}
  Result<{{ return_type }}, {{ name }}UseCaseException> call() {
{%- endif %}
    try {
      throw UnimplementedError();
    } on Exception {
      return Failure(exception: UnknownException());
    } catch (e) {
      if (kDebugMode) {
        print(e);
        rethrow;
      }
      return Failure(exception: UnknownException());
    }
  }
}

