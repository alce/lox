import 'package:dlox/src/function.dart';

import 'callable.dart';
import 'environment.dart';
import 'exception.dart';
import 'expression.dart';
import 'lox.dart';
import 'nil.dart';
import 'return.dart';
import 'statement.dart';
import 'token.dart';
import 'util.dart';
import 'visitor.dart';

class _Clock implements LoxCallable {
  @override
  int get arity => 0;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    return DateTime.now().second;
  }

  @override
  String toString() => '<native fn>';
}

class Interpreter implements ExprVisitor<Object>, StmtVisitor<void> {
  final globals = Environment();

  var _env;

  Interpreter() {
    globals.define('clock', _Clock);
    _env = globals;
  }

  void interpret(List<Stmt> statements) {
    try {
      for (final stmt in statements) {
        _execute(stmt);
      }
    } on RuntimeError catch (e) {
      Lox.runtimeError(e);
    }
  }

  @override
  Object visitAssignExpr(AssignExpr expr) {
    final value = _evaluate(expr.value);
    _env.assign(expr.name, value);
    return value;
  }

  @override
  void visitBlockStmt(BlockStmt stmt) =>
      executeBlock(stmt.statements, Environment(_env));

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
          expr.operator,
          'Operands must be two numbers or two strings.',
        );
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
        return !isEqual(left, right);
      case TokenType.EQUAL_EQUAL:
        return isEqual(left, right);
      default:
        throw UnimplementedError();
    }
  }

  @override
  Object visitCallExpr(CallExpr expr) {
    final callee = _evaluate(expr.callee);

    if (callee is! LoxCallable) {
      throw RuntimeError(expr.paren, 'Can only call functions and classes.');
    }

    final args = <Object>[];
    for (final arg in expr.arguments) {
      args.add(_evaluate(arg));
    }

    if (args.length != callee.arity) {
      throw RuntimeError(expr.paren,
          'Expected ${callee.arity} arguments but got ${args.length}.');
    }

    return callee.call(this, args);
  }

  @override
  void visitExpressionStmt(ExpressionStmt stmt) => _evaluate(stmt.expression);

  @override
  void visitFunctionStmt(FunctionStmt stmt) {
    final func = LoxFunction(stmt, _env);
    _env.define(stmt.name.lexeme, func);
  }

  @override
  Object visitGroupingExpr(GroupingExpr expr) => _evaluate(expr.expression);

  @override
  void visitIfStmt(IfStmt stmt) {
    if (isTruthy(_evaluate(stmt.condition))) {
      _execute(stmt.thenBranch);
    } else if (stmt.elseBranch != null) {
      _execute(stmt.elseBranch!);
    }
  }

  @override
  Object visitLiteralExpr(LiteralExpr expr) {
    return expr.value;
  }

  @override
  Object visitLogicalExpr(LogicalExpr expr) {
    final left = _evaluate(expr.left);

    if (expr.operator.type == TokenType.OR) {
      if (isTruthy(left)) return left;
    } else {
      if (!isTruthy(left)) return left;
    }

    return _evaluate(expr.right);
  }

  @override
  Object visitNil(Nil expr) => expr;

  @override
  void visitPrintStmt(PrintStmt stmt) =>
      print(stringify(_evaluate(stmt.expression)));

  @override
  void visitReturnStmt(ReturnStmt stmt) {
    var value;

    if (stmt.value is! Nil) {
      value = _evaluate(stmt.value);
    }

    value ??= Nil();

    throw Return(value);
  }

  @override
  Object visitUnaryExpr(UnaryExpr expr) {
    final right = _evaluate(expr.right);

    switch (expr.operator.type) {
      case TokenType.MINUS:
        _checkNumberOperand(expr.operator, right);
        return -(right as double);
      case TokenType.BANG:
        return !isTruthy(right);
      default:
        throw Exception('Unreachable');
    }
  }

  @override
  Object visitVariableExpr(VariableExpr expr) => _env.get(expr.name);

  @override
  void visitVarStmt(VarStmt stmt) {
    var value;

    if (stmt.initializer != null) {
      value = _evaluate(stmt.initializer!);
    }

    _env.define(stmt.name.lexeme, value);
  }

  void _execute(Stmt stmt) => stmt.accept(this);

  void executeBlock(List<Stmt> statements, Environment env) {
    final prev = _env;

    try {
      _env = env;
      for (final stmt in statements) {
        _execute(stmt);
      }
    } finally {
      _env = prev;
    }
  }

  @override
  void visitWhileStmt(WhileStmt stmt) {
    while (isTruthy(_evaluate(stmt.condition))) {
      _execute(stmt.body);
    }
  }

  Object _evaluate(Expr expr) => expr.accept(this);

  void _checkNumberOperand(Token operator, Object operand) {
    if (operand is! double) {
      throw RuntimeError(operator, 'Operand must be a number.');
    }
  }

  void _checkNumberOperands(Token operator, Object left, Object right) {
    if (left is double && right is double) {
      return;
    }
    throw RuntimeError(operator, 'Operands must be numbers.');
  }
}
