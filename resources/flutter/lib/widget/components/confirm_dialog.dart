import 'package:flutter/material.dart';

import '../../common/extension/i18n_loc.dart';

class ConfirmDialog extends StatelessWidget {
  const ConfirmDialog({required this.action, super.key});
  final String action;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: Text(action),
      content: Text(context.i18n.components.confirmMessage(action: action)),
      actions: [
        ElevatedButton(
          onPressed: () => Navigator.pop(context, false),
          child: Text(context.i18n.components.cancel),
        ),
        ElevatedButton(
          onPressed: () => Navigator.pop(context, true),
          child: Text(context.i18n.components.signOut),
        ),
      ],
    );
  }
}
