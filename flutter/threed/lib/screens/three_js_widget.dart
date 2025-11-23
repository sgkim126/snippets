import 'package:flutter/material.dart';
import '../three_js/three_js_controller.dart';
import '../three_js/three_js_viewer.dart';

class ThreeJsWidget extends StatefulWidget {
  const ThreeJsWidget({super.key, required this.title});

  final String title;

  @override
  State<ThreeJsWidget> createState() => _ThreeJsWidgetState();
}

class _ThreeJsWidgetState extends State<ThreeJsWidget> {
  late final ThreeJsController _controller;
  bool _isRunning = false;
  PointerDownEvent? _downEvent;
  bool _isLoaded = false;

  @override
  void initState() {
    super.initState();
    _controller = ThreeJsController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

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
            if (_isLoaded && dx < 18 && dy < 18 && time < 200) {
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
        child: ThreeJsViewer(
          controller: _controller,
          onLoad: () {
            _controller.playAnimation(animationName: "Walk");
            setState(() {
              _isLoaded = true;
            });
          },
        ),
      ),
    );
  }
}
