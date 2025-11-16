import 'package:flutter/material.dart';
import 'package:flutter_3d_controller/flutter_3d_controller.dart';

class Flutter3DControllerWidget extends StatefulWidget {
  const Flutter3DControllerWidget({super.key, required this.title});

  final String title;

  @override
  State<Flutter3DControllerWidget> createState() => Flutter3DControllerState();
}

class Flutter3DControllerState extends State<Flutter3DControllerWidget> {
  final Flutter3DController _controller = Flutter3DController();
  bool _isRunning = false;
  PointerDownEvent? _downEvent;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Listener(
        onPointerDown: (event) {
          _downEvent = event;
        },
        onPointerUp: (event) {
          if (_downEvent != null) {
            final dx = (event.position.dx - _downEvent!.position.dx).abs();
            final dy = (event.position.dy - _downEvent!.position.dy).abs();
            final time = event.timeStamp.inMilliseconds - _downEvent!.timeStamp.inMilliseconds;

            // Detect tap based on position and time delta
            if (dx < 18 && dy < 18 && time < 200) {
              if (_isRunning) {
                _controller.playAnimation(animationName: "Run");
              } else {
                _controller.playAnimation(animationName: "Walk");
              }
              setState(() {
                _isRunning = !_isRunning;
              });
            }
          }
        },
        child: Flutter3DViewer(
          controller: _controller,
          src: 'assets/dash.glb',
          onLoad: (modelAddress) {
            if (_isRunning) {
              _controller.playAnimation(animationName: "Run");
            } else {
              _controller.playAnimation(animationName: "Walk");
            }
          }
        ),
      ),
    );
  }
}
