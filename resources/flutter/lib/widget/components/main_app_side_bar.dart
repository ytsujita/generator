import 'package:flutter/material.dart';

/// [Drawer]メニューの開閉に応じたUIを表示するアイテム
class MainAppSideBar extends StatelessWidget {
  const MainAppSideBar({
    required this.width,
    required this.icon,
    required this.selected,
    required this.title,
    required this.onTap,
    this.enabled = true,
    this.color,
    super.key,
  });
  final double width;
  final IconData icon;
  final bool selected;
  final String title;
  final void Function()? onTap;
  final Color? color;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: enabled ? onTap : null,
      child: Container(
        padding: const EdgeInsets.all(4),
        color: selected ? Theme.of(context).colorScheme.surfaceContainer : null,
        width: width,
        height: 50,
        child: Row(
          children: [
            const SizedBox(width: 8),
            Tooltip(
              message: width >= 250 ? '' : title,
              child: Icon(
                icon,
                color: enabled
                    ? color ??
                        (selected
                            ? Theme.of(context).colorScheme.primary
                            : Theme.of(context).colorScheme.onSurface)
                    : Theme.of(context).disabledColor,
              ),
            ),
            const SizedBox(width: 8),
            if (width >= 250)
              Text(
                title,
                style: TextStyle(
                  overflow: TextOverflow.fade,
                  color: enabled
                      ? selected
                          ? Theme.of(context).colorScheme.primary
                          : Theme.of(context).colorScheme.onSurface
                      : Theme.of(context).disabledColor,
                ),
              ),
          ],
        ),
      ),
    );
  }
}
