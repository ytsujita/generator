import 'package:flutter/material.dart';

/// [Drawer]メニューなどをタブのように扱うとき
/// 状態を維持しつつ他のタブに移動するためのウィジェット
class FadeIndexedStack extends StatefulWidget {
  const FadeIndexedStack({
    required this.index,
    required this.children,
    this.duration = const Duration(milliseconds: 100),
    super.key,
  });
  final int index;
  final List<Widget> children;
  final Duration duration;

  @override
  State<StatefulWidget> createState() => _FadeIndexedStackState();
}

class _FadeIndexedStackState extends State<FadeIndexedStack>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;

  @override
  Widget build(BuildContext context) {
    return FadeTransition(
      opacity: _controller,
      // alwaysIncludeSemantics: false,
      child: IndexedStack(
        index: widget.index,
        children: widget.children,
      ),
    );
  }

  @override
  void didUpdateWidget(covariant FadeIndexedStack oldWidget) {
    if (widget.index != oldWidget.index) {
      _controller.forward(from: 0);
    }
    super.didUpdateWidget(oldWidget);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  void initState() {
    _controller = AnimationController(vsync: this, duration: widget.duration);
    _controller.forward();
    super.initState();
  }
}
