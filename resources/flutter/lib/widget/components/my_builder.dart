import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../provider/notifier/builder_loading_state_provider.dart';

class MyBuilder extends StatelessWidget {
  const MyBuilder({
    super.key,
    required this.child,
    required this.context,
  });
  final Widget? child;
  final BuildContext context;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        if (child != null) Positioned.fill(child: child!),
        const Positioned.fill(child: _LoadingWidget()),
      ],
    );
  }
}

class _LoadingWidget extends ConsumerWidget {
  const _LoadingWidget();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Visibility(
      visible: ref.watch(loadingStateProvider),
      child: const SizedBox.expand(
        child: ColoredBox(
          color: Colors.black45,
          child: Center(child: CircularProgressIndicator()),
        ),
      ),
    );
  }
}
