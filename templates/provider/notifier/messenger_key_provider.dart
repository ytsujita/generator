import 'dart:math';

import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../common/utils/notification_category.dart';

final messengerKeyProvider =
    NotifierProvider<MessengerKeyNotifier, GlobalKey<ScaffoldMessengerState>>(
  MessengerKeyNotifier.new,
);

class MessengerKeyNotifier extends Notifier<GlobalKey<ScaffoldMessengerState>> {
  @override
  GlobalKey<ScaffoldMessengerState> build() {
    return GlobalKey<ScaffoldMessengerState>();
  }

  /// Show snackbar
  void showSnackBar({
    required NotificationCategory category,
    required String Function(BuildContext context) message,
    bool showCloseIcon = true,
    Duration? duration,
    bool hideCurrentSnackBar = true,
  }) {
    if (hideCurrentSnackBar) {
      state.currentState?.hideCurrentSnackBar();
    }
    final context = state.currentContext;
    if (context == null) {
      return;
    }
    state.currentState?.showSnackBar(
      SnackBar(
        width: min(
          MediaQuery.of(context).size.width * 0.8,
          _getTextSize(message(context)).width + 200,
        ),
        behavior: SnackBarBehavior.floating,
        showCloseIcon: showCloseIcon,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(10),
        ),
        content: Row(
          children: [
            Icon(category.icon, color: category.color),
            const SizedBox(width: 8),
            Text(message(context)),
          ],
        ),
        duration: duration ?? const Duration(milliseconds: 4000),
      ),
    );
  }

  Size _getTextSize(String text) {
    const style = TextStyle(
      fontSize: 12,
      letterSpacing: 0.67,
      fontWeight: FontWeight.w300,
    );
    final textPainter = TextPainter(
      text: TextSpan(text: text, style: style),
      textDirection: TextDirection.ltr,
    )..layout(maxWidth: 300);
    return textPainter.size;
  }
}
