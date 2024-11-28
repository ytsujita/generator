import 'package:flutter/material.dart';

import '../../common/extension/l10n_loc.dart';

class CancelButton extends StatelessWidget {
  const CancelButton({
    required this.onPressed,
    this.child,
    super.key,
  });
  final VoidCallback onPressed;
  final Widget? child;

  @override
  Widget build(BuildContext context) {
    return OutlinedButton(
      onPressed: onPressed,
      child: child ?? Text(context.loc.cancel),
    );
  }
}

class ActionButton extends StatelessWidget {
  const ActionButton({
    required this.child,
    this.onPressed,
    super.key,
  });
  final VoidCallback? onPressed;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: onPressed,
      child: child,
    );
  }
}
