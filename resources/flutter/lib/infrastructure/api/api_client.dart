// import 'package:dio/browser.dart';
// import 'package:dio/dio.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final apiClientProvider = Provider<ApiClient>((ref) {
  const env = String.fromEnvironment('ENV');
  return ApiClient(
    baseUrl: const String.fromEnvironment('API_BASE_URL'),
    subBasePath: env.isEmpty ? null : env,
  );
});

class MyResponse<T> {
  MyResponse({
    required this.data,
    required this.exception,
    required this.statusCode,
    required this.errorCode,
  });
  final T? data;
  final Exception? exception;
  final int? statusCode;
  final int? errorCode;
}

/// APIアクセス処理を薄くラップしたクラス
class ApiClient {
  ApiClient({
    required this.baseUrl,
    this.subBasePath,
  });
  final String baseUrl;
  final String? subBasePath;
  // final Dio dio = Dio();

  Future<MyResponse<T>> get<T>({
    required String subPath,
    Map<String, String>? headers,
    Map<String, dynamic>? queryParameters,
    bool withCredentials = false,
  }) async {
    // if (kIsWeb) {
    //   dio.httpClientAdapter = BrowserHttpClientAdapter(
    //     withCredentials: withCredentials,
    //   );
    // }
    // try {
    //   final res = await dio.get<T>(
    //     _getUri(subPath: subPath).toString(),
    //     options: Options(
    //       headers: {
    //         ...headers ?? {},
    //         'X-Requested-With': 'XMLHttpRequest',
    //       },
    //     ),
    //     queryParameters: queryParameters,
    //   );
    //   if (withCredentials && res.statusCode == StatusCode.unauthorized) {
    //     final refreshTokenRes = await dio.post<void>(
    //       _getUri(subPath: '/refresh-token').toString(),
    //       options: Options(
    //         headers: {
    //           ...headers ?? {},
    //           'X-Requested-With': 'XMLHttpRequest',
    //         },
    //       ),
    //     );
    //     if (refreshTokenRes.statusCode == StatusCode.ok) {
    //       final res = await dio.get<T>(
    //         _getUri(subPath: subPath).toString(),
    //         options: Options(
    //           headers: {
    //             ...headers ?? {},
    //             'X-Requested-With': 'XMLHttpRequest',
    //           },
    //         ),
    //         queryParameters: queryParameters,
    //       );
    //       return MyResponse(
    //         data: res.data,
    //         statusCode: res.statusCode,
    //         exception: null,
    //         errorCode: null,
    //       );
    //     } else {
    //       return MyResponse(
    //         data: res.data,
    //         statusCode: res.statusCode,
    //         exception: null,
    //         errorCode: null,
    //       );
    //     }
    //   }
    //   return MyResponse(
    //     data: res.data,
    //     statusCode: res.statusCode,
    //     exception: null,
    //     errorCode: null,
    //   );
    // } on DioException catch (e) {
    // ignore: avoid_dynamic_calls
    //   final errorCode = e.response?.data['errorCode'] as int?;
    //   return MyResponse(
    //     data: null,
    //     statusCode: e.response?.statusCode,
    //     exception: e,
    //     errorCode: errorCode,
    //   );
    // }
    return MyResponse(
      data: null,
      statusCode: null,
      exception: null,
      errorCode: null,
    );
  }

  Future<MyResponse<T>> post<T>({
    required String subPath,
    String? body,
    Map<String, String>? headers,
    bool withCredentials = false,
  }) async {
    return MyResponse(
      data: null,
      statusCode: null,
      exception: null,
      errorCode: null,
    );
    // if (kIsWeb) {
    //   dio.httpClientAdapter = BrowserHttpClientAdapter(
    //     withCredentials: withCredentials,
    //   );
    // }
    // try {
    //   final res = await dio.post<T>(
    //     _getUri(subPath: subPath).toString(),
    //     options: Options(
    //       headers: {
    //         ...headers ?? {},
    //         'X-Requested-With': 'XMLHttpRequest',
    //       },
    //     ),
    //     data: body,
    //   );
    //   if (withCredentials && res.statusCode == StatusCode.unauthorized) {
    //     final refreshTokenRes = await dio.post<void>(
    //       _getUri(subPath: '/refresh-token').toString(),
    //       options: Options(
    //         headers: {
    //           ...headers ?? {},
    //           'X-Requested-With': 'XMLHttpRequest',
    //         },
    //       ),
    //     );
    //     if (refreshTokenRes.statusCode == StatusCode.ok) {
    //       final res = await dio.post<T>(
    //         _getUri(subPath: subPath).toString(),
    //         options: Options(
    //           headers: {
    //             ...headers ?? {},
    //             'X-Requested-With': 'XMLHttpRequest',
    //           },
    //         ),
    //         data: body,
    //       );
    //       return MyResponse(
    //         data: res.data,
    //         statusCode: res.statusCode,
    //         exception: null,
    //         errorCode: null,
    //       );
    //     } else {
    //       return MyResponse(
    //         data: res.data,
    //         statusCode: res.statusCode,
    //         exception: null,
    //         errorCode: null,
    //       );
    //     }
    //   }
    //   return MyResponse(
    //     data: res.data,
    //     statusCode: res.statusCode,
    //     exception: null,
    //     errorCode: null,
    //   );
    // } on DioException catch (e) {
    //   // ignore: avoid_dynamic_calls
    //   final errorCode = e.response?.data['errorCode'] as int?;
    //   return MyResponse(
    //     data: null,
    //     statusCode: e.response?.statusCode,
    //     exception: e,
    //     errorCode: errorCode,
    //   );
    // }
  }

  // Uri _getUri({
  //   required String subPath,
  //   Map<String, dynamic>? queryParameters,
  // }) {
  //   final path = subBasePath == null ? subPath : '$subBasePath/$subPath';
  //   return Uri.https(
  //     baseUrl,
  //     path,
  //     queryParameters,
  //   );
  // }
}
