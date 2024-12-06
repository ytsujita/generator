import 'route_path.dart';

class NavigationState {
  const NavigationState({required this.rootRoutePathStack});

  factory NavigationState.fromUri({
    required Uri uri,
    NavigationState? previousState,
  }) {
    final routePath = RoutePath.fromUri(uri);
    return NavigationState.fromRoutePath(
      routePath: routePath,
      previousState: previousState,
    );
  }

  /// 特定のルート[routePath]が表示されるときのページ構成を構築する
  factory NavigationState.fromRoutePath({
    required RoutePath routePath,
    NavigationState? previousState,
  }) {
    final _ = previousState ?? NavigationState.init();
    switch (routePath) {
      {% for name in route_path_names %}
        case {{ name|pascal }}RoutePath():
          // TODO: Handle this case.
      {%- endfor %}
    }
  }

  factory NavigationState.init() {
    return const NavigationState(
      rootRoutePathStack: [{{ default_route_path_name|pascal }}RoutePath()],
    );
  }

  // root
  final List<BaseRoutePath> rootRoutePathStack;

  RoutePath _getRouteFromStack(List<BaseRoutePath> pathStack) {
    if (pathStack.isEmpty) {
      throw Exception('Shellの中身を空にできません。');
    }
    final lastOne = pathStack.last;
    return switch (lastOne) {
      ShellRoutePath(:final pathStack, :final selectedIndex) =>
        _getRouteFromStack(pathStack[selectedIndex]!),
      RoutePath() => lastOne,
    };
  }

  RoutePath get currentRoutePath {
    return _getRouteFromStack(rootRoutePathStack);
  }

  Uri? get currentUri {
    return currentRoutePath.uri;
  }

  NavigationState copyWith({
    List<BaseRoutePath>? rootRoutePathStack,
  }) {
    return NavigationState(
      rootRoutePathStack: rootRoutePathStack ?? this.rootRoutePathStack,
    );
  }

  NavigationState pop() {
    final tmp = rootRoutePathStack.last;
    switch (tmp) {
      case ShellRoutePath():
        final popped = tmp.pop();
        return copyWith(
          rootRoutePathStack: [
            ...rootRoutePathStack.sublist(0, rootRoutePathStack.length - 1),
            if (popped != null) popped,
          ],
        );
      case RoutePath():
        return copyWith(
          rootRoutePathStack:
              rootRoutePathStack.sublist(0, rootRoutePathStack.length - 1),
        );
    }
  }
}
