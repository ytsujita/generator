import 'package:hooks_riverpod/hooks_riverpod.dart';

{% match provider_type %}
  {% when Provider %}
final 
  {% when NotifierProvider %}
  {% when FutureProvider %}
  {% when StreamProvider %}
  {% when AsyncNotifierProvider %}
  {% when StreamNotifierProvider %}
{% endmatch %}
