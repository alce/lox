import 'builtins.dart';
import 'callable.dart';
import 'class.dart';
import 'environment.dart';
import 'exception.dart';
import 'expression.dart';
import 'function.dart';
import 'instance.dart';
import 'lox.dart';
import 'nil.dart';
import 'return.dart';
import 'statement.dart';
import 'token.dart';
import 'util.dart';
import 'visitor.dart';

class Interpreter implements ExprVisitor<Object>, StmtVisitor<void> {
  final globals = Environment();
  final _locals = <Expr, int>{};
  var _env;

  Interpreter() {
    globals.define('clock', Clock());
    _env = globals;
  }

  void interpret(List<Stmt> statements) {
    try {
      statements.forEach(_execute);
    } on RuntimeError catch (e) {
      Lox.runtimeError(e);
    }
  }

  void resolve(Expr expr, int depth) => _locals[expr] = depth;

  @override
  Object visitAssignExpr(AssignExpr expr) {
    final value = _evaluate(expr.value);
    final distance = _locals[expr];

    if (distance != null) {
      _env.assignAt(distance, expr.name, value);
    } else {
      globals.assign(expr.name, value);
    }

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

    final args = expr.arguments.map(_evaluate).toList();

    if (args.length != callee.arity) {
      throw RuntimeError(expr.paren,
          'Expected ${callee.arity} arguments but got ${args.length}.');
    }

    return callee.call(this, args);
  }

  @override
  void visitClassStmt(ClassStmt stmt) {
    var superclass;
    if (stmt.superclass != null) {
      superclass = _evaluate(stmt.superclass!);
      if (superclass is! LoxClass) {
        throw RuntimeError(
            stmt.superclass!.name, 'Superclass must be a class.');
      }
    }

    _env.define(stmt.name.lexeme, Nil());

    if (stmt.superclass != null) {
      _env = Environment(_env);
      _env.define('super', superclass);
    }

    final methods = stmt.methods.fold(
      <String, LoxFunction>{},
      (Map<String, LoxFunction> acc, method) {
        final lexeme = method.name.lexeme;
        acc[lexeme] = LoxFunction(method, _env, lexeme == 'init');
        return acc;
      },
    );

    final klass = LoxClass(stmt.name.lexeme, superclass, methods);

    if (superclass != null) {
      _env = _env.enclosing;
    }

    _env.assign(stmt.name, klass);
  }

  @override
  void visitExpressionStmt(ExpressionStmt stmt) => _evaluate(stmt.expression);

  @override
  void visitFunctionStmt(FunctionStmt stmt) =>
      _env.define(stmt.name.lexeme, LoxFunction(stmt, _env, false));

  @override
  Object visitGetExpr(GetExpr expr) {
    final obj = _evaluate(expr.object);
    if (obj is LoxInstance) {
      return obj.get(expr.name);
    }
    throw RuntimeError(expr.name, 'Only instances have properties.');
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
  Object visitLiteralExpr(LiteralExpr expr) => expr.value;

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
  Object visitSetExpr(SetExpr expr) {
    final obj = _evaluate(expr.object);

    if (obj is! LoxInstance) {
      throw RuntimeError(expr.name, 'Only instances have fields.');
    }

    final value = _evaluate(expr.value);
    obj.set(expr.name, value);
    return value;
  }

  @override
  Object visitSuperExpr(SuperExpr expr) {
    final distance = _locals[expr]!;
    final superclass = _env.getAt(distance, 'super');
    final obj = _env.getAt(distance - 1, 'this');
    final method = superclass?.findMethod(expr.method.lexeme);

    if (method == null) {
      throw RuntimeError(
          expr.method, "Undefined property '${expr.method.lexeme}'.");
    }

    return method.bind(obj);
  }

  @override
  Object visitThisExpr(ThisExpr expr) => _lookup(expr.keyword, expr);

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
  Object visitVariableExpr(VariableExpr expr) => _lookup(expr.name, expr);

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
      statements.forEach(_execute);
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
    if (!(left is double && right is double)) {
      throw RuntimeError(operator, 'Operands must be numbers.');
    }
  }

  Object _lookup(Token name, Expr expr) {
    final distance = _locals[expr];

    if (distance != null) {
      return _env.getAt(distance, name.lexeme) ?? Nil();
    } else {
      return globals.get(name);
    }
  }
}
