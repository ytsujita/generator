import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final backButtonDispatcherProvider = Provider.autoDispose(
  (final ref) => RootBackButtonDispatcher(),
);
