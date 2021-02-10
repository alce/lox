import 'expression/expression.dart';
import 'visitor.dart';

class AstPrinter implements Visitor<String> {
  String print(Expression expr) {
    return expr.accept(this);
  }

  @override
  String visitBinaryExpression(BinaryExpression exp) {
    return _parenthesize(exp.operator.lexeme, [exp.left, exp.right]);
  }

  @override
  String visitGroupingExpression(GroupingExpression exp) {
    return _parenthesize('group', [exp.expression]);
  }

  @override
  String visitLiteralExpression(LiteralExpression exp) {
    return exp.value == null ? 'nil' : exp.value.toString();
  }

  @override
  String visitUnaryExpression(UnaryExpression exp) {
    return _parenthesize(exp.operator.lexeme, [exp.right]);
  }

  String _parenthesize(String name, List<Expression> expressions) {
    final buf = StringBuffer();

    buf.write('(');
    buf.write(name);

    for (final exp in expressions) {
      buf.write(' ');
      buf.write(exp.accept(this));
    }

    buf.write(')');

    return buf.toString();
  }
}
