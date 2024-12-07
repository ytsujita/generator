import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import 'navigation_state.dart';
import 'navigation_state_provider.dart';
import 'route_path.dart';

/// [RouteInformationParser] は OS とのやり取りを行う
class MainRouteInformationParser
    extends RouteInformationParser<NavigationState> {
  MainRouteInformationParser(this._ref);
  final WidgetRef _ref;

  /// [RouteInformationProvider] から通知される [RouteInformation] を
  /// アプリの状態である [RoutePath] に変換する
  ///
  /// Web : URL に値を入力したとき、「戻る」「進む」が押されたときに呼び出される
  ///
  /// その他プラットフォーム : 初回アクセス時に呼び出される
  ///
  /// Async な理由は認証チェックなどを行うため
  /// async 処理をしないなら [SynchronousFuture] 返却を検討する
  ///
  /// [BuildContext]が必要な場合は[parseRouteInformationWithDependencies]が
  /// 代わりに実行する
  @override
  Future<NavigationState> parseRouteInformation(
    RouteInformation routeInformation,
  ) async {
    throw UnimplementedError();
  }

  @override
  Future<NavigationState> parseRouteInformationWithDependencies(
    RouteInformation routeInformation,
    BuildContext context,
  ) async {
    return NavigationState.fromUri(
      uri: routeInformation.uri,
      previousState: _ref.read(navigationStateProvider),
    );
  }

  /// アプリの状態からパスを更新するためのメソッド, webでは実装が必須
  ///
  /// [RouterDelegate.currentConfiguration] の後に呼び出され、
  /// 渡された [RoutePath] が持つ状態から [RouteInformation] に変換する
  @override
  RouteInformation? restoreRouteInformation(NavigationState configuration) {
    final res = configuration.currentUri;
    return res == null ? null : RouteInformation(uri: res);
  }
}
