import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../provider/notifier/auth/sign_in_user_provider.dart';
import 'navigation_state.dart';
import 'navigation_state_provider.dart';
import 'route_path.dart';

/// [RouterDelegate] は渡された設定に基づいて状態を復元する役割
class MainRouterDelegate extends RouterDelegate<NavigationState>
    with ChangeNotifier, PopNavigatorRouterDelegateMixin<NavigationState> {
  MainRouterDelegate(this._ref) : navigatorKey = GlobalKey<NavigatorState>() {
    // [navigationStateProvider]が変更されたことを[Router]へ通知
    _ref.listen(navigationStateProvider, (_, __) => notifyListeners());
  }

  @override
  final GlobalKey<NavigatorState> navigatorKey;
  final WidgetRef _ref;

  /// 戻るボタンが押された時の挙動、Web のブラウザバックは関係ない。
  ///
  /// 基本は [PopNavigatorRouterDelegateMixin] を with で mixin する。
  /// false を返すとアプリ全体をポップする。つまりアプリを閉じる。
  ///
  /// 非同期処理を行わない場合は [SynchronousFuture] で返却すべき
  @override
  Future<bool> popRoute() async {
    // 基本機能は PopNavigatorRouterDelegateMixin が提供するので override しなくてもいい
    // PopNavigatorRouterDelegateMixin が提供するのは以下
    final navigator = navigatorKey.currentState;
    if (navigator == null) {
      return SynchronousFuture<bool>(false);
    }
    return navigator.maybePop();
    // maybePop は Navigator の pages stack がなくなるまでポップする
    // stack がなくなったら false が返る
    // stack がなくても戻るボタンでアプリを閉じてほしくない場合
    // 例えば bottom navigation bar でタブ移動の履歴をたどるような場合には向かない
  }

  /// 現在のアプリの状態から [NavigationState] を返す
  ///
  /// [Router] が再構築によって経路情報が変更された可能性を検出したときに呼び出される.
  ///
  /// 現在のアプリの状態は _ref によって読み取る
  @override
  NavigationState? get currentConfiguration {
    final currentState = _ref.read(navigationStateProvider);
    final signInUserState = _ref.read(signInUserProvider);
    return signInUserState.when(
      data: (data) {
        if (data == null) {
          switch (currentState.currentRoutePath) {
            case SignInRoutePath():
            case FetchLoadingRoutePath():
            case SendOtpRoutePath():
            case ResetPasswordRoutePath():
            case InitializePasswordRoutePath():
            case VerifyEmailRoutePath():
              return currentState;
            default:
              return currentState.copyWith(
                rootRoutePathStack: [const SignInRoutePath()],
              );
          }
        }
        switch (currentState.currentRoutePath) {
          case SignInRoutePath():
          case FetchLoadingRoutePath():
          case SendOtpRoutePath():
          case ResetPasswordRoutePath():
            return currentState.copyWith(
              rootRoutePathStack: [const MainTabShellRoutePath()],
            );
          default:
            return currentState;
        }
      },
      error: (error, stackTrace) {
        return currentState.copyWith(
          rootRoutePathStack: [const SignInRoutePath()],
        );
      },
      loading: () {
        return currentState.copyWith(
          rootRoutePathStack: [
            const FetchLoadingRoutePath(),
          ],
        );
      },
    );
  }

  /// 状態によって[Widget]を切り替える[Navigator]を返す
  /// [notifyListeners]が呼び出された後に実行される
  @override
  Widget build(BuildContext context) {
    return Navigator(
      key: navigatorKey,
      pages: _ref
          .read(navigationStateProvider)
          .rootRoutePathStack
          .map(
            (e) => switch (e) {
              ShellRoutePath() => switch (e) {
                  MainTabShellRoutePath() => e.buildPage(),
                },
              RoutePath() => e.buildPage(),
            },
          )
          .toList(),
      onDidRemovePage: (poppedPage) {},
    );
  }

  /// [RouteInformationParser.parseRouteInformation] で解析された
  /// [NavigationState] から状態を更新する
  ///
  /// [setNewRoutePath] の後に notifyListeners を呼び出すべき
  /// 非同期処理を行わない場合 [SynchronousFuture] で返却すべき
  @override
  Future<void> setNewRoutePath(NavigationState configuration) async {
    _ref.read(navigationStateProvider.notifier).navigationState = configuration;
    return SynchronousFuture<void>(null);
  }

  /// 状態の復元中に Router によって呼び出される
  // @override
  // Future<void> setRestoredRoutePath(NavigationState configuration) {
  //   return setNewRoutePath(configuration); // デフォルト
  // }

  /// アプリ起動時にのみ呼び出される状態更新処理
  // @override
  // Future<void> setInitialRoutePath(NavigationState configuration) async {
  //   return super.setInitialRoutePath(configuration);
  // }
}
