import 'route_path.dart';

enum MainTab {
  incidentLog,
  account,
  adminAccount,
  group,
  groupAdminAccount,
  groupBelongingAccount,
  monitoringAppSettings,
  ;
}

class NavigationState {
  const NavigationState({required this.rootRoutePathStack});

  factory NavigationState.fromUri({
    required Uri uri,
    NavigationState? previousState,
  }) {
    final routePath = RoutePath.fromUri(uri);
    return NavigationState.fromRoutePath(
      routePath: const SignInRoutePath(),
      previousState: previousState,
    )
  }

  /// 特定のルート[routePath]が表示されるときのページ構成を構築する
  factory NavigationState.fromRoutePath({
    required RoutePath routePath,
    NavigationState? previousState,
  }) {
    final tmp = previousState ?? NavigationState.init();
    switch (routePath) {
      case InitializePasswordRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const SignInRoutePath(),
            const InitializePasswordRoutePath(),
          ],
        );
      case VerifyEmailRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const SignInRoutePath(),
            const VerifyEmailRoutePath(),
          ],
        );
      case SessionTimeoutRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            ...tmp.rootRoutePathStack,
            const SessionTimeoutRoutePath(),
          ],
        );
      case FetchLoadingRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const FetchLoadingRoutePath(),
          ],
        );
      case SignInRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const SignInRoutePath(),
          ],
        );
      case SendOtpRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const SignInRoutePath(),
            const SendOtpRoutePath(),
          ],
        );
      case ResetPasswordRoutePath():
        return tmp.copyWith(
          rootRoutePathStack: [
            const SignInRoutePath(),
            const SendOtpRoutePath(),
            const ResetPasswordRoutePath(),
          ],
        );
      // ユーザー管理
      case AccountSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.account,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          accountTabRoutePathStack: [
            const AccountSearchRoutePath(),
          ],
        );
      case AccountRegisterRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.account,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          accountTabRoutePathStack: [
            const AccountSearchRoutePath(),
            const AccountRegisterRoutePath(),
          ],
        );
      case AccountImportRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.account,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          accountTabRoutePathStack: [
            const AccountSearchRoutePath(),
            const AccountImportRoutePath(),
          ],
        );
      case AccountDetailRoutePath(:final accountId):
        return tmp.copyWith(
          selectedTab: MainTab.account,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          accountTabRoutePathStack: [
            const AccountSearchRoutePath(),
            AccountDetailRoutePath(accountId: accountId),
          ],
        );
      // 管理者管理
      case AdminAccountSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.adminAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          adminAccountTabRoutePathStack: [
            const AdminAccountSearchRoutePath(),
          ],
        );
      // グループ管理
      case GroupImportRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.group,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupTabRoutePathStack: [
            const GroupSearchRoutePath(),
            const GroupImportRoutePath(),
          ],
        );
      case GroupRegisterRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.group,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupTabRoutePathStack: [
            const GroupSearchRoutePath(),
            const GroupRegisterRoutePath(),
          ],
        );
      case GroupDetailRoutePath(:final groupId):
        return tmp.copyWith(
          selectedTab: MainTab.group,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupTabRoutePathStack: [
            const GroupSearchRoutePath(),
            GroupDetailRoutePath(groupId: groupId),
          ],
        );
      // グループ管理者管理
      case GroupAdminAccountSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.groupAdminAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupAdminAccountTabRoutePathStack: [
            const GroupAdminAccountSearchRoutePath(),
          ],
        );
      case GroupAdminAccountBulkRegisterRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.groupAdminAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupAdminAccountTabRoutePathStack: [
            const GroupAdminAccountSearchRoutePath(),
            const GroupAdminAccountBulkRegisterRoutePath(),
          ],
        );
      case GroupBelongingAccountSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.groupBelongingAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupBelongingAccountTabRoutePathStack: [
            const GroupBelongingAccountSearchRoutePath(),
          ],
        );
      case GroupBelongingAccountBulkRegisterRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.groupBelongingAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupBelongingAccountTabRoutePathStack: [
            const GroupBelongingAccountSearchRoutePath(),
            const GroupBelongingAccountBulkRegisterRoutePath(),
          ],
        );
      case IncidentLogSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.incidentLog,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          incidentLogTabRoutePathStack: [
            const IncidentLogSearchRoutePath(),
          ],
        );
      case MonitoringAppSettingsRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.monitoringAppSettings,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          settingsTabRoutePathStack: [
            const MonitoringAppSettingsRoutePath(),
          ],
        );
      case GroupSearchRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.group,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          groupTabRoutePathStack: [
            const GroupSearchRoutePath(),
          ],
        );
      case AdminAccountDetailRoutePath(:final adminAccountId):
        return tmp.copyWith(
          selectedTab: MainTab.adminAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          adminAccountTabRoutePathStack: [
            const AdminAccountSearchRoutePath(),
            AdminAccountDetailRoutePath(adminAccountId: adminAccountId),
          ],
        );
      case AdminAccountBulkRegisterRoutePath():
        return tmp.copyWith(
          selectedTab: MainTab.adminAccount,
          rootRoutePathStack: [
            const MainTabShellRoutePath(),
          ],
          adminAccountTabRoutePathStack: [
            const AdminAccountSearchRoutePath(),
            const AdminAccountBulkRegisterRoutePath(),
          ],
        );
    }
  }

  factory NavigationState.init() {
    return const NavigationState(
      rootRoutePathStack: [FetchLoadingRoutePath()],
    );
  }

  // root
  final List<BaseRoutePath> rootRoutePathStack;

  RoutePath _getRouteFromStack(List<RoutePath> pathStack) {
    if (pathStack.isEmpty) {
      throw Exception("Shellの中身を空にできません。");
    }
    final lastOne = pathStack.last;
    return switch (lastOne) {
      ShellRoutePath(:final pathStack) => _getRouteFromStack(pathStack),
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
      case ShellRoutePath(:final pathStack):
        switch (selectedTab) {
          case MainTab.account:
            return copyWith(
              accountTabRoutePathStack: accountTabRoutePathStack.sublist(
                0,
                accountTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.incidentLog:
            return copyWith(
              incidentLogTabRoutePathStack:
                  incidentLogTabRoutePathStack.sublist(
                0,
                incidentLogTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.adminAccount:
            return copyWith(
              adminAccountTabRoutePathStack:
                  adminAccountTabRoutePathStack.sublist(
                0,
                adminAccountTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.group:
            return copyWith(
              groupTabRoutePathStack: groupTabRoutePathStack.sublist(
                0,
                groupTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.groupAdminAccount:
            return copyWith(
              groupAdminAccountTabRoutePathStack:
                  groupAdminAccountTabRoutePathStack.sublist(
                0,
                groupAdminAccountTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.groupBelongingAccount:
            return copyWith(
              groupBelongingAccountTabRoutePathStack:
                  groupBelongingAccountTabRoutePathStack.sublist(
                0,
                groupBelongingAccountTabRoutePathStack.length - 1,
              ),
            );
          case MainTab.monitoringAppSettings:
            return copyWith(
              settingsTabRoutePathStack: settingsTabRoutePathStack.sublist(
                0,
                settingsTabRoutePathStack.length - 1,
              ),
            );
        }
      case RoutePath():
        return copyWith(
          rootRoutePathStack:
              rootRoutePathStack.sublist(0, rootRoutePathStack.length - 1),
        );
    }
  }
}
