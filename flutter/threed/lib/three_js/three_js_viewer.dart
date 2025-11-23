import 'package:flutter/material.dart';
import './three_js_controller.dart';

class ThreeJsViewer extends StatefulWidget {
  final ThreeJsController controller;
  final void Function()? onLoad;

  const ThreeJsViewer({super.key, required this.controller, this.onLoad});

  @override
  State<ThreeJsViewer> createState() => _ThreeJsViewerState();
}

class _ThreeJsViewerState extends State<ThreeJsViewer> {
  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (BuildContext context, BoxConstraints constraints) {
        if (constraints.maxWidth > 0 && constraints.maxHeight > 0) {
          if (widget.controller.threeJs == null) {
            widget.controller.init(
              onSetupComplete: () {
                widget.onLoad?.call();
                if (mounted) {
                  setState(() {});
                }
              },
            );
          }
          return widget.controller.threeJs!.build();
        }
        return const Center(child: CircularProgressIndicator());
      },
    );
  }
}
