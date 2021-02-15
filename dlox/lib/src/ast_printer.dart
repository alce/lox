import 'expression.dart';
import 'nil.dart';
import 'statement.dart';
import 'token.dart';
import 'visitor.dart';

class AstPrinter implements ExprVisitor<String> {
  String print(Expr expr) => expr.accept(this);

  @override
  String visitAssignExpr(AssignExpr expr) =>
      _parenthesize2('=', [expr.name.lexeme, expr.value]);

  @override
  String visitBinaryExpr(BinaryExpr expr) =>
      _parenthesize(expr.operator.lexeme, [expr.left, expr.right]);

  @override
  String visitGroupingExpr(GroupingExpr expr) =>
      _parenthesize('group', [expr.expression]);

  @override
  String visitLiteralExpr(LiteralExpr expr) => expr.value.toString();

  @override
  String visitLogicalExpr(LogicalExpr expr) => throw UnimplementedError();

  @override
  String visitNil(Nil expr) => 'nil';

  @override
  String visitUnaryExpr(UnaryExpr expr) =>
      _parenthesize(expr.operator.lexeme, [expr.right]);

  @override
  String visitVariableExpr(VariableExpr expr) => expr.name.lexeme;

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

  String _parenthesize2(String name, List<Object> parts) {
    final buf = StringBuffer();

    buf.write('(');
    buf.write(name);
    _transform(buf, parts);
    buf.write(')');

    return buf.toString();
  }

  void _transform(StringBuffer buf, List<Object> parts) {
    for (final part in parts) {
      buf.write(' ');
      if (part is Expr) {
        buf.write(part.accept(this));
      } else if (part is Stmt) {
        // buf.write(part.accept(this));
      } else if (part is Token) {
        buf.write(part.lexeme);
      } else {
        buf.write(part);
      }
    }
  }

  @override
  String visitCallExpr(CallExpr expr) => throw UnimplementedError();

  @override
  String visitGetExpr(GetExpr expr) => throw UnimplementedError();

  @override
  String visitSetExpr(SetExpr expr) => throw UnimplementedError();
}
