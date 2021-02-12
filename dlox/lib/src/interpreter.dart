import 'exception.dart';
import 'expression.dart';
import 'statement.dart';
import 'token.dart';
import 'util.dart';
import 'visitor.dart';

class Interpreter implements ExprVisitor<Object>, StmtVisitor<void> {
  void interpret(List<Stmt> statements) {
    try {
      for (final stmt in statements) {
        _execute(stmt);
      }
    } on RuntimeError catch (e) {
      // Lox.runtimeError(e)
      print(e);
    }
  }

  @override
  Object visitBinaryExpr(BinaryExpr expr) {
    final left = _evaluate(expr.left);
    final right = _evaluate(expr.right);

    switch (expr.operator.type) {
      case TokenType.MINUS:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) - (right as double);
      case TokenType.SLASH:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) / (right as double);
      case TokenType.STAR:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) * (right as double);
      case TokenType.PLUS:
        if (left is double && right is double) {
          return left + right;
        }

        if (left is String && right is String) {
          return left + right;
        }

        throw RuntimeError(
            expr.operator, 'Operands must two numbers or two strings');
      case TokenType.GREATER:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) > (right as double);
      case TokenType.GREATER_EQUAL:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) >= (right as double);
      case TokenType.LESS:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) < (right as double);
      case TokenType.LESS_EQUAL:
        _checkNumberOperands(expr.operator, left, right);
        return (left as double) <= (right as double);
      case TokenType.BANG_EQUAL:
        return !_isEqual(left, right);
      case TokenType.EQUAL_EQUAL:
        return _isEqual(left, right);
      default:
        throw UnimplementedError();
    }
  }

  @override
  Object visitGroupingExpr(GroupingExpr expr) {
    return _evaluate(expr.expression);
  }

  @override
  Object visitLiteralExpr(LiteralExpr expr) {
    final val = expr.value;
    if (val == null) {
      throw Exception('found null');
    } else {
      return val;
    }
  }

  @override
  Object visitUnaryExpr(UnaryExpr expr) {
    final right = _evaluate(expr.right);

    switch (expr.operator.type) {
      case TokenType.MINUS:
        _checkNumberOperand(expr.operator, right);
        return -(right as double);
      case TokenType.BANG:
        return !_isTruthy(right);
      default:
        throw Exception('Unreachable');
    }
  }

  void _execute(Stmt stmt) => stmt.accept(this);

  Object _evaluate(Expr expr) => expr.accept(this);

  // false and nil are falsy, everything else is truthy;
  bool _isTruthy(Object? value) {
    if (value == null) {
      return false;
    }

    if (value is bool) {
      return value;
    }

    return true;
  }

  bool _isEqual(Object? a, Object? b) => a == b;

  void _checkNumberOperand(Token operator, Object operand) {
    if (operand is! double) {
      throw RuntimeError(operator, 'Operand must be a number');
    }
  }

  void _checkNumberOperands(Token operator, Object left, Object right) {
    if (left is double && right is double) {
      return;
    }
    throw RuntimeError(operator, 'Operands must be numbers');
  }

  @override
  void visitExpressionStmt(ExpressionStmt stmt) {
    _evaluate(stmt.expression);
  }

  @override
  void visitPrintStmt(PrintStmt stmt) {
    final value = _evaluate(stmt.expression);
    print(stringify(value));
  }
}
