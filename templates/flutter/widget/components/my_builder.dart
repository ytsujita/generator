import 'package:flutter/material.dart';

/// builderによるウィジェットの生成
///
/// ローディング中にオーバーレイを表示する
Widget myBuilder(BuildContext context, Widget? child, bool loading) {
  return Stack(
    children: [
      if (child != null) Positioned.fill(child: child),
      if (loading)
        Overlay(
          initialEntries: [
            OverlayEntry(
              builder: (context) => Scaffold(
                backgroundColor: Colors.black.withOpacity(0.5),
                body: const Center(
                  child: CircularProgressIndicator(),
                ),
              ),
            ),
          ],
        ),
    ],
  );
}
