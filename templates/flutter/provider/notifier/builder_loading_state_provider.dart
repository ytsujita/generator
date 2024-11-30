import 'dart:async';

import 'package:hooks_riverpod/hooks_riverpod.dart';

final loadingStateProvider = NotifierProvider<LoadingStateNotifier, bool>(
  LoadingStateNotifier.new,
);

class LoadingStateNotifier extends Notifier<bool> {
  @override
  bool build() {
    return false;
  }

  Future<T> showAsyncDialog<T>(Future<T> Function() function) async {
    state = true;
    final result = await function();
    state = false;
    return result;
  }
}
