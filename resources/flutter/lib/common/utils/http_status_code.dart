class StatusCode {
  static const int continue_ = 100; // Continue
  static const int switchingProtocols = 101; // Switching Protocols
  static const int processing = 102; // Processing
  static const int ok = 200; // OK
  static const int created = 201; // Created
  static const int accepted = 202; // Accepted
  static const int nonAuthoritativeInformation =
      203; // Non Authoritative Information
  static const int noContent = 204; // No Content
  static const int resetContent = 205; // Reset Content
  static const int partialContent = 206; // Partial Content
  static const int multiStatus = 207; // Multi-Status
  static const int multipleChoices = 300; // Multiple Choices
  static const int movedPermanently = 301; // Moved Permanently
  static const int movedTemporarily = 302; // Moved Temporarily
  static const int seeOther = 303; // See Other
  static const int notModified = 304; // Not Modified
  static const int useProxy = 305; // Use Proxy
  static const int temporaryRedirect = 307; // Temporary Redirect
  static const int permanentRedirect = 308; // Permanent Redirect
  static const int badRequest = 400; // Bad Request
  static const int unauthorized = 401; // Unauthorized
  static const int paymentRequired = 402; // Payment Required
  static const int forbidden = 403; // Forbidden
  static const int notFound = 404; // Not Found
  static const int methodNotAllowed = 405; // Method Not Allowed
  static const int notAcceptable = 406; // Not Acceptable
  static const int proxyAuthenticationRequired =
      407; // Proxy Authentication Required
  static const int requestTimeout = 408; // Request Timeout
  static const int conflict = 409; // Conflict
  static const int gone = 410; // Gone
  static const int lengthRequired = 411; // Length Required
  static const int preconditionFailed = 412; // Precondition Failed
  static const int requestTooLong = 413; // Request Entity Too Large
  static const int requestUriTooLong = 414; // Request-URI Too Long
  static const int unsupportedMediaType = 415; // Unsupported Media Type
  static const int requestedRangeNotSatisfiable =
      416; // Requested Range Not Satisfiable
  static const int expectationFailed = 417; // Expectation Failed
  static const int imATeapot = 418; // I'm a teapot
  static const int insufficientSpaceOnResource =
      419; // Insufficient Space on Resource
  static const int methodFailure = 420; // Method Failure
  static const int unprocessableEntity = 422; // Unprocessable Entity
  static const int locked = 423; // Locked
  static const int failedDependency = 424; // Failed Dependency
  static const int preconditionRequired = 428; // Precondition Required
  static const int tooManyRequests = 429; // Too Many Requests
  static const int requestHeaderFieldsTooLarge =
      431; // Request Header Fields Too Large
  static const int unavailableForLegalReasons =
      451; // Unavailable For Legal Reasons
  static const int internalServerError = 500; // Internal Server Error
  static const int notImplemented = 501; // Not Implemented
  static const int badGateway = 502; // Bad Gateway
  static const int serviceUnavailable = 503; // Service Unavailable
  static const int gatewayTimeout = 504; // Gateway Timeout
  static const int httpVersionNotSupported = 505; // HTTP Version Not Supported
  static const int insufficientStorage = 507; // Insufficient Storage
  static const int networkAuthenticationRequired =
      511; // Network Authentication Required
}
