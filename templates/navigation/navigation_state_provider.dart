import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../provider/notifier/auth/sign_in_user_provider.dart';
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
    // signInUserの状態変更を監視して、その状態に応じた画面遷移状態を返す
    // AsyncData -> AsyncData などの画面遷移の必要のない状態変化は無視するようにしている
    ref.watch(
      signInUserProvider.select(
        (val) => switch (val) {
          AsyncData(value: null) => 0,
          AsyncData() => 1,
          AsyncError() => 2,
          _ => 3,
        },
      ),
    );
    return switch (ref.read(signInUserProvider)) {
      AsyncData(value: null) =>
        NavigationState.fromRoutePath(routePath: const SignInRoutePath()),
      AsyncData() => NavigationState.fromRoutePath(
          routePath: const IncidentLogSearchRoutePath(),
        ),
      AsyncError() =>
        NavigationState.fromRoutePath(routePath: const SignInRoutePath()),
      _ =>
        NavigationState.fromRoutePath(routePath: const FetchLoadingRoutePath()),
    };
  }

  /// 完全に画面遷移状態を置き換える
  ///
  /// これまでの遷移情報が失われるため、極力使用しないようにする
  set navigationState(NavigationState newState) {
    state = newState;
  }

  /// [RoutePath]から[NavigationState]を生成して遷移する
  void navigate(RoutePath myRoutePath) {
    state = NavigationState.fromRoutePath(
      routePath: myRoutePath,
      previousState: state,
    ).copyWith();
  }

  /// [MainTab]から[RoutePath]を生成して遷移する
  void navigateMainTab(MainTab mainTab) {
    final targetRoutePath = switch (mainTab) {
      MainTab.incidentLog => const IncidentLogSearchRoutePath(),
      MainTab.account => const AccountSearchRoutePath(),
      MainTab.adminAccount => const AdminAccountSearchRoutePath(),
      MainTab.group => const GroupSearchRoutePath(),
      MainTab.groupAdminAccount => const GroupAdminAccountSearchRoutePath(),
      MainTab.groupBelongingAccount =>
        const GroupBelongingAccountSearchRoutePath(),
      MainTab.monitoringAppSettings => const MonitoringAppSettingsRoutePath(),
    };
    navigate(targetRoutePath);
  }

  /// 現在の画面からポップできるページをポップする
  void pop() {
    state = state.pop();
  }
}
