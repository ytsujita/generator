import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../navigation/navigation_state_provider.dart';

/// [Router]用の[AppBar]
///
/// [Router]を使用してページスタックを構築するとき、
/// 自動で生成される戻るボタンを[Router]へ命令するものに置き換える
class RouterAppBar extends StatelessWidget implements PreferredSizeWidget {
  const RouterAppBar({
    super.key,
    required this.title,
    this.height = kToolbarHeight,
    this.titleTextStyle,
    this.backgroundColor,
    this.actions,
    this.shadowColor,
    this.shape,
    this.actionsIconTheme,
    this.elevation = 0,
    this.scrolledUnderElevation = 0,
  });

  final Widget title;
  final double height;
  final TextStyle? titleTextStyle;
  final Color? backgroundColor;
  final List<Widget>? actions;
  final Color? shadowColor;
  final ShapeBorder? shape;
  final IconThemeData? actionsIconTheme;
  final double? elevation;
  final double? scrolledUnderElevation;

  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: title,
      automaticallyImplyLeading: false,
      backgroundColor: backgroundColor,
      titleTextStyle: titleTextStyle,
      actions: [
        ...?actions,
        const SizedBox(width: 8),
      ],
      bottomOpacity: 0,
      shadowColor: shadowColor,
      elevation: elevation,
      scrolledUnderElevation: scrolledUnderElevation,
      shape: shape,
      actionsIconTheme: actionsIconTheme,
      leading: ModalRoute.of(context)?.impliesAppBarDismissal ?? false
          ? Consumer(
              builder: (context, ref, child) {
                return IconButton(
                  onPressed: ref.read(navigationStateProvider.notifier).pop,
                  icon: const Icon(Icons.arrow_back),
                );
              },
            )
          : null,
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(height);
}
