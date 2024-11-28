import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

/// パスワード入力フォーム
///
/// デフォルトでパスワードの入力内容を隠し、表示用のボタンを表示する
class PasswordFormField extends HookWidget {
  const PasswordFormField({
    required this.autofillHints,
    required this.hintText,
    this.onChanged,
    this.onSubmitted,
    this.onEditingComplete,
    this.validator,
    this.textInputAction,
    this.prefixIcon = const Icon(Icons.lock_outline),
    this.enabled = true,
    this.controller,
    super.key,
  });
  final void Function(String value)? onChanged;
  final void Function(String value)? onSubmitted;
  final void Function()? onEditingComplete;
  final Iterable<String>? autofillHints;
  final String hintText;
  final String? Function(String?)? validator;
  final TextInputAction? textInputAction;
  final Widget? prefixIcon;
  final bool enabled;
  final TextEditingController? controller;

  @override
  Widget build(BuildContext context) {
    final isVisible = useState(false);
    return TextFormField(
      enabled: enabled,
      controller: controller,
      autofillHints: autofillHints,
      onFieldSubmitted: onSubmitted,
      onEditingComplete: onEditingComplete,
      textInputAction: textInputAction,
      validator: validator,
      keyboardType: TextInputType.visiblePassword,
      obscureText: !isVisible.value,
      onChanged: onChanged,
      decoration: InputDecoration(
        errorMaxLines: 2,
        prefixIcon: prefixIcon,
        hintText: hintText,
        suffixIcon: IconButton(
          onPressed: () {
            isVisible.value = !isVisible.value;
          },
          icon: isVisible.value
              ? const Icon(Icons.visibility)
              : const Icon(Icons.visibility_off),
        ),
      ),
    );
  }
}
