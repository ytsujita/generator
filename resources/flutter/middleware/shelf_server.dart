import 'dart:io';

import 'package:http/http.dart' as http;
// ignore: depend_on_referenced_packages
import 'package:shelf/shelf.dart';
// ignore: depend_on_referenced_packages
import 'package:shelf/shelf_io.dart' as io;
// ignore: depend_on_referenced_packages
import 'package:shelf_cors_headers/shelf_cors_headers.dart';

final myCorsHeaders = {
  'Access-Control-Allow-Origin': 'https://localhost:5555', // 繝輔Ο繝ｳ繝医お繝ｳ繝峨・URL
  'Access-Control-Allow-Credentials': 'true',
  'Access-Control-Allow-Headers':
      'Origin, Content-Type, X-Auth-Token, X-Requested-With',
  'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
  'Content-Type': 'application/json',
};

void main() async {
  // CORS縺ｮ險ｭ螳・  const certificateChain = 'localhost.crt';
  const certificateKey = 'localhost.key';

  // 繝溘ラ繝ｫ繧ｦ繧ｧ繧｢縺ｮ險ｭ螳・  final handler = const Pipeline()
      .addMiddleware(
        corsHeaders(
          headers: myCorsHeaders,
          originChecker: (origin) => origin == 'https://localhost:5555',
        ),
      )
      .addMiddleware(logRequests())
      .addHandler(_echoRequest);

  // 繧ｵ繝ｼ繝舌・縺ｮ襍ｷ蜍・  final server = await io.serve(
    handler,
    InternetAddress.anyIPv4,
    8080,
    securityContext: SecurityContext()
      ..useCertificateChain(certificateChain)
      ..usePrivateKey(certificateKey),
  );
  // ignore: avoid_print
  print('localhost:${server.port} Server listening');
}

Future<Response> _echoRequest(Request request) async {
  final uri = Uri.parse('https://d2qm6x1ae62nmq.cloudfront.net/${request.url}');
  switch (request.method) {
    case 'OPTIONS':
      return Response(200, headers: myCorsHeaders);
    case 'GET':
      final requestCookies = request.headers['Cookie'];
      final response = await http.get(
        uri,
        headers: {
          ...(requestCookies != null ? {'Cookie': requestCookies} : {}),
          'Content-Type': 'application/json',
          'X-Requested-With': 'XMLHttpRequest',
        },
      );
      final setCookies = response.headers['set-cookie'];
      final newSetCookie = _parseSetCookieString(setCookies ?? '')
          .entries
          .map((e) => '${e.key}=${e.value}')
          .toList();
      return Response(
        response.statusCode,
        headers: {
          ...myCorsHeaders,
          ...{'Set-Cookie': newSetCookie},
          'Content-Type': 'application/json',
          'X-Requested-With': 'XMLHttpRequest',
        },
        body: response.body,
      );
    case 'POST':
      final body = await request.readAsString();
      final requestCookies = request.headers['Cookie'];
      final response = await http.post(
        uri,
        headers: {
          ...(requestCookies != null ? {'Cookie': requestCookies} : {}),
          'Content-Type': 'application/json',
          'X-Requested-With': 'XMLHttpRequest',
        },
        body: body,
      );
      final setCookies = response.headers['set-cookie'];
      final newSetCookie = _parseSetCookieString(setCookies ?? '')
          .entries
          .map((e) => '${e.key}=${e.value}')
          .toList();
      return Response(
        response.statusCode,
        headers: {
          ...myCorsHeaders,
          ...{'Set-Cookie': newSetCookie},
          'Content-Type': 'application/json',
          'X-Requested-With': 'XMLHttpRequest',
        },
        body: response.body,
      );
    default:
      return Response(405, body: 'Method Not Allowed');
  }
}

Map<String, String> _parseSetCookieString(String s) {
  final targetCookieKey = [
    'id',
    'session',
    'refresh',
  ];
  final cookies = <String, String>{};
  final splitCookies = s.split(RegExp(r'[;, ]'));
  for (final setCookie in splitCookies) {
    final cookie = setCookie.split('=');
    if (cookie.length == 2 && targetCookieKey.contains(cookie[0])) {
      cookies[cookie[0]] = cookie[1];
    }
  }
  return cookies;
}
