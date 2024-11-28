import 'package:flutter/material.dart';

/// [InkWell]で[Checkbox]の複合ウィジェット
class InkWellCheckBox extends StatelessWidget {
  const InkWellCheckBox({
    required this.checkboxVal,
    required this.label,
    required this.onCheck,
    this.enabled = true,
    super.key,
  });
  final bool checkboxVal;
  final bool enabled;
  final String label;
  final void Function() onCheck;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: enabled ? onCheck : null,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Checkbox(
            value: checkboxVal,
            onChanged: enabled ? (val) => onCheck() : null,
          ),
          Text(
            label,
            style: TextStyle(
              color: enabled ? null : Theme.of(context).disabledColor,
            ),
          ),
        ],
      ),
    );
  }
}
