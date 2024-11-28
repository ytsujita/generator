class QueryResult<T> {
  QueryResult({required this.paginationToken, required this.items});

  final String? paginationToken;

  final List<T> items;
}
