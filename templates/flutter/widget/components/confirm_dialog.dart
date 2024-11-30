import 'package:flutter/material.dart';

import '../../common/extension/l10n_loc.dart';
import 'button.dart';

class ConfirmDialog extends StatelessWidget {
  const ConfirmDialog({required this.action, super.key});
  final String action;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(action),
      content: Text(context.loc.confirmMessage(action)),
      actions: [
        CancelButton(
          onPressed: () => Navigator.pop(context, false),
          child: Text(context.loc.cancel),
        ),
        ActionButton(
          onPressed: () => Navigator.pop(context, true),
          child: Text(context.loc.signOut),
        ),
      ],
    );
  }
}
