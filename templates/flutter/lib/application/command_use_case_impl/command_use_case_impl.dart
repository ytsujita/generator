import 'package:flutter/foundation.dart';
import 'package:{{ application_name }}/common/utils/result.dart';
import 'package:{{ application_name }}/application/use_case/{{ file_name }}';

class {{ name|pascal }}UseCaseImpl extends {{ name|pascal }}UseCase {
  @override
  Result<void, {{ name|pascal }}UseCaseException> call() {
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

