import 'package:flutter/material.dart';

class NotFoundPage extends MaterialPage<void> {
  const NotFoundPage() : super(child: const NotFoundWidget());
}

class NotFoundWidget extends StatelessWidget {
  const NotFoundWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text('Page Not Found'),
      ),
    );
  }
}
