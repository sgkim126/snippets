import 'package:flutter/material.dart';
import 'package:threed/screens/flutter_3d_controller_widget.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter 3D',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const Flutter3DControllerWidget(title: 'Flutter 3D Controller'),
    );
  }
}
