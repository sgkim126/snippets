import 'dart:async';
import 'package:three_js/three_js.dart' as three;

class ThreeJsController {
  three.ThreeJS? threeJs;
  three.OrbitControls? controls;
  three.AnimationMixer? mixer;
  final Map<String, three.AnimationAction> _actions = {};

  void init({required void Function() onSetupComplete}) {
    threeJs = three.ThreeJS(
      onSetupComplete: onSetupComplete,
      setup: () => setup(),
    );
  }

  Future<void> setup() async {
    if (threeJs == null) {
      return;
    }

    threeJs!.camera =
        three.PerspectiveCamera(45, threeJs!.width / threeJs!.height, 1, 2200);
    threeJs!.camera.position.setValues(3, 6, 10);

    threeJs!.scene = three.Scene();

    final ambientLight = three.AmbientLight(0xffffff, 0.3);
    threeJs!.scene.add(ambientLight);

    final pointLight = three.PointLight(0xffffff, 0.1);
    pointLight.position.setValues(0, 0, 0);
    threeJs!.camera.add(pointLight);
    threeJs!.scene.add(threeJs!.camera);

    threeJs!.camera.lookAt(threeJs!.scene.position);

    three.GLTFLoader loader = three.GLTFLoader(flipY: true).setPath('assets/');
    final dash = (await loader.fromAsset('dash.glb'))!;

    mixer = three.AnimationMixer(dash.scene);

    for (final clip in dash.animations!) {
      final action = mixer!.clipAction(clip)!;
      _actions[clip.name] = action;
      action.setEffectiveWeight(0);
      action.enabled = true;
      action.play();
    }

    threeJs!.scene.add(dash.scene);

    controls = three.OrbitControls(threeJs!.camera, threeJs!.globalKey);
    controls!.target = dash.scene.position;
    controls!.enableDamping = true;
    controls!.dampingFactor = 0.5;

    threeJs!.addAnimationEvent((dt) {
      controls?.update();
      mixer?.update(dt);
    });
  }

  void dispose() {
    threeJs?.dispose();
    controls?.dispose();
    three.loading.clear();
  }

  void playAnimation({required String animationName}) {
    for (final action in _actions.keys) {
      _actions[action]!.setEffectiveWeight(action == animationName ? 1 : 0);
    }
  }
}