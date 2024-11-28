import 'package:flutter/material.dart';

class BreadcrumbsItem {
  BreadcrumbsItem({
    required this.title,
    this.onPressed,
  });
  final String title;
  final void Function()? onPressed;
}

class BreadcrumbsWidget extends StatelessWidget {
  const BreadcrumbsWidget({required this.items, super.key});
  final List<BreadcrumbsItem> items;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8),
      child: Row(
        children: [
          for (final item in items)
            Row(
              children: [
                if (item.onPressed != null)
                  MouseRegion(
                    cursor: SystemMouseCursors.click,
                    child: GestureDetector(
                      onTap: item.onPressed,
                      child: Text(
                        item.title,
                        style: item.onPressed != null
                            ? Theme.of(context).textTheme.titleMedium?.copyWith(
                                  color: Theme.of(context).colorScheme.primary,
                                  decoration: TextDecoration.underline,
                                )
                            : Theme.of(context).textTheme.titleMedium,
                      ),
                    ),
                  )
                else
                  Text(
                    item.title,
                    style: Theme.of(context).textTheme.titleMedium?.copyWith(
                          decoration: TextDecoration.underline,
                        ),
                  ),
                if (item != items.last)
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 8),
                    child: Icon(
                      Icons.arrow_forward_ios_sharp,
                      size: Theme.of(context).textTheme.titleMedium?.fontSize,
                    ),
                  ),
              ],
            ),
        ],
      ),
    );
  }
}
