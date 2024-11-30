import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'navigation_state.dart';
import 'route_path.dart';

/// 画面遷移状態を管理する
final navigationStateProvider =
    NotifierProvider<NavigationStateNotifier, NavigationState>(
  NavigationStateNotifier.new,
);

class NavigationStateNotifier extends Notifier<NavigationState> {
  @override
  NavigationState build() {
    return NavigationState.init();
  }

  /// 完全に画面遷移状態を置き換える
  ///
  /// これまでの遷移情報が失われるため、極力使用しないようにする
  set navigationState(NavigationState newState) {
    state = newState;
  }

  /// [RoutePath]から[NavigationState]を生成して遷移する
  void navigate(RoutePath routePath) {
    state = NavigationState.fromRoutePath(
      routePath: routePath,
      previousState: state,
    );
  }

  /// 現在の画面からポップできるページをポップする
  void pop() {
    state = state.pop();
  }
}
