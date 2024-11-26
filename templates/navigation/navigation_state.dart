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
  const NavigationState({
    required this.rootRoutePathStack,
    required this.selectedTab,
    required this.incidentLogTabRoutePathStack,
    required this.accountTabRoutePathStack,
    required this.adminAccountTabRoutePathStack,
    required this.groupTabRoutePathStack,
    required this.groupBelongingAccountTabRoutePathStack,
    required this.groupAdminAccountTabRoutePathStack,
    required this.settingsTabRoutePathStack,
  });

  factory NavigationState.fromUri({
    required Uri uri,
    NavigationState? previousState,
    bool isAuthorized = true,
  }) {
    final routePath = RoutePath.fromUri(uri);
    if (isAuthorized) {
      final newState = NavigationState.fromRoutePath(
        routePath: routePath,
        previousState: previousState,
      );
      return switch (newState.currentRoutePath) {
        SignInRoutePath() ||
        SessionTimeoutRoutePath() ||
        SendOtpRoutePath() ||
        ResetPasswordRoutePath() =>
          NavigationState.fromRoutePath(
            routePath: const IncidentLogSearchRoutePath(),
            previousState: previousState,
          ),
        _ => newState,
      };
    }
    final newState = NavigationState.fromRoutePath(
      routePath: routePath,
      previousState: previousState,
    );
    return switch (newState.currentRoutePath) {
      SignInRoutePath() ||
      SessionTimeoutRoutePath() ||
      SendOtpRoutePath() ||
      ResetPasswordRoutePath() ||
      VerifyEmailRoutePath() ||
      InitializePasswordRoutePath() =>
        newState,
      _ => NavigationState.fromRoutePath(
          routePath: const SignInRoutePath(),
          previousState: previousState,
        ),
    };
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
      selectedTab: MainTab.incidentLog,
      incidentLogTabRoutePathStack: [IncidentLogSearchRoutePath()],
      accountTabRoutePathStack: [AccountSearchRoutePath()],
      adminAccountTabRoutePathStack: [AdminAccountSearchRoutePath()],
      groupTabRoutePathStack: [GroupSearchRoutePath()],
      groupAdminAccountTabRoutePathStack: [
        GroupAdminAccountSearchRoutePath(),
      ],
      groupBelongingAccountTabRoutePathStack: [
        GroupBelongingAccountSearchRoutePath(),
      ],
      settingsTabRoutePathStack: [MonitoringAppSettingsRoutePath()],
    );
  }

  // root
  final List<BaseRoutePath> rootRoutePathStack;

  // main tab
  final MainTab selectedTab;
  final List<BaseRoutePath> incidentLogTabRoutePathStack;
  final List<BaseRoutePath> accountTabRoutePathStack;
  final List<BaseRoutePath> adminAccountTabRoutePathStack;
  final List<BaseRoutePath> groupTabRoutePathStack;
  final List<BaseRoutePath> groupBelongingAccountTabRoutePathStack;
  final List<BaseRoutePath> groupAdminAccountTabRoutePathStack;
  final List<BaseRoutePath> settingsTabRoutePathStack;

  List<BaseRoutePath> getMainTabPathStack(MainTab mainTab) {
    return switch (mainTab) {
      MainTab.incidentLog => incidentLogTabRoutePathStack,
      MainTab.account => accountTabRoutePathStack,
      MainTab.adminAccount => adminAccountTabRoutePathStack,
      MainTab.group => groupTabRoutePathStack,
      MainTab.groupAdminAccount => groupAdminAccountTabRoutePathStack,
      MainTab.groupBelongingAccount => groupBelongingAccountTabRoutePathStack,
      MainTab.monitoringAppSettings => settingsTabRoutePathStack,
    };
  }

  RoutePath get currentRoutePath {
    var tmp = rootRoutePathStack.last;
    switch (tmp) {
      case ShellRoutePath():
        switch (tmp) {
          case MainTabShellRoutePath():
            tmp = getMainTabPathStack(selectedTab).last;
            switch (tmp) {
              case RoutePath():
                return tmp;
              case MainTabShellRoutePath():
                throw Exception('MainTabShellRoutePathはネストできません。');
            }
          default:
            throw Exception('Root直下にMainTabShellRoutePath以外を配置できません。');
        }
      case RoutePath():
        return tmp;
    }
  }

  Uri? get currentUri {
    switch (currentRoutePath) {
      case FetchLoadingRoutePath():
        return null;
      case SignInRoutePath():
      case InitializePasswordRoutePath():
      case SessionTimeoutRoutePath():
      case SendOtpRoutePath():
      case ResetPasswordRoutePath():
      case VerifyEmailRoutePath():
      case MonitoringAppSettingsRoutePath():
      case AccountSearchRoutePath():
      case AccountRegisterRoutePath():
      case AccountImportRoutePath():
      case AccountDetailRoutePath():
      case AdminAccountSearchRoutePath():
      case GroupSearchRoutePath():
      case GroupImportRoutePath():
      case GroupRegisterRoutePath():
      case GroupDetailRoutePath():
      case GroupAdminAccountSearchRoutePath():
      case GroupAdminAccountBulkRegisterRoutePath():
      case GroupBelongingAccountSearchRoutePath():
      case GroupBelongingAccountBulkRegisterRoutePath():
      case IncidentLogSearchRoutePath():
      case AdminAccountBulkRegisterRoutePath():
      case AdminAccountDetailRoutePath():
        return currentRoutePath.uri;
    }
  }

  NavigationState copyWith({
    List<BaseRoutePath>? rootRoutePathStack,
    MainTab? selectedTab,
    List<BaseRoutePath>? incidentLogTabRoutePathStack,
    List<BaseRoutePath>? accountTabRoutePathStack,
    List<BaseRoutePath>? adminAccountTabRoutePathStack,
    List<BaseRoutePath>? groupTabRoutePathStack,
    List<BaseRoutePath>? groupBelongingAccountTabRoutePathStack,
    List<BaseRoutePath>? groupAdminAccountTabRoutePathStack,
    List<BaseRoutePath>? settingsTabRoutePathStack,
  }) {
    return NavigationState(
      rootRoutePathStack: rootRoutePathStack ?? this.rootRoutePathStack,
      selectedTab: selectedTab ?? this.selectedTab,
      incidentLogTabRoutePathStack:
          incidentLogTabRoutePathStack ?? this.incidentLogTabRoutePathStack,
      accountTabRoutePathStack:
          accountTabRoutePathStack ?? this.accountTabRoutePathStack,
      adminAccountTabRoutePathStack:
          adminAccountTabRoutePathStack ?? this.adminAccountTabRoutePathStack,
      groupTabRoutePathStack:
          groupTabRoutePathStack ?? this.groupTabRoutePathStack,
      groupBelongingAccountTabRoutePathStack:
          groupBelongingAccountTabRoutePathStack ??
              this.groupBelongingAccountTabRoutePathStack,
      groupAdminAccountTabRoutePathStack: groupAdminAccountTabRoutePathStack ??
          this.groupAdminAccountTabRoutePathStack,
      settingsTabRoutePathStack:
          settingsTabRoutePathStack ?? this.settingsTabRoutePathStack,
    );
  }

  NavigationState pop() {
    final tmp = rootRoutePathStack.last;
    switch (tmp) {
      case MainTabShellRoutePath():
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
