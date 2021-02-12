import 'expression.dart';
import 'visitor.dart';

class AstPrinter implements ExprVisitor<String> {
  String print(Expr expr) => expr.accept(this);

  @override
  String visitBinaryExpr(BinaryExpr expr) =>
      _parenthesize(expr.operator.lexeme, [expr.left, expr.right]);

  @override
  String visitGroupingExpr(GroupingExpr expr) =>
      _parenthesize('group', [expr.expression]);

  @override
  String visitLiteralExpr(LiteralExpr expr) => expr.value.toString();

  @override
  String visitUnaryExpr(UnaryExpr expr) =>
      _parenthesize(expr.operator.lexeme, [expr.right]);

  String _parenthesize(String name, List<Expr> expressions) {
    final buf = StringBuffer();

    buf.write('(');
    buf.write(name);

    for (final expr in expressions) {
      buf.write(' ');
      buf.write(expr.accept(this));
    }

    buf.write(')');

    return buf.toString();
  }

  @override
  String visitVariableExpr(VariableExpr expr) => expr.name.lexeme;
}
