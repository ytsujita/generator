import 'package:flutter/material.dart';

class {{ widget_name|pascal }}Widget extends StatelessWidget {
  const {{ widget_name|pascal }}Widget({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(body: Center(child: Text('{{ widget_name|pascal }}')));
  }
}

