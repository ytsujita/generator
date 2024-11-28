class UnauthorizedException implements Exception {}

class ForbiddenException implements Exception {
  ForbiddenException({this.errorCode});
  final int? errorCode;
}

class InvalidValueException implements Exception {}

class NotFoundException implements Exception {}

class IsExistResourceException implements Exception {}

class InternalServerErrorException implements Exception {}

class NetworkNotAvailableException implements Exception {}

class BadRequestException implements Exception {}
