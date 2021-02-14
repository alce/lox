class Nil {
  @override
  bool operator ==(Object other) {
    return other is Nil;
  }

  @override
  String toString() => 'null';
}
