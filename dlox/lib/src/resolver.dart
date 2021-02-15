import 'expression.dart';
import 'interpreter.dart';
import 'lox.dart';
import 'nil.dart';
import 'statement.dart';
import 'token.dart';
import 'visitor.dart';

enum _FunctionType {
  NONE,
  FUNCTION,
}

class Resolver implements ExprVisitor<void>, StmtVisitor<void> {
  final Interpreter _interpreter;
  final _scopes = <Map<String, bool>>[];
  var _currentFunction = _FunctionType.NONE;

  Resolver(this._interpreter);

  @override
  void visitAssignExpr(AssignExpr expr) {
    _resolveExpr(expr.value);
    _resolveLocal(expr, expr.name);
  }

  @override
  void visitBinaryExpr(BinaryExpr expr) {
    _resolveExpr(expr.left);
    _resolveExpr(expr.right);
  }

  @override
  void visitBlockStmt(BlockStmt stmt) {
    _beginScope();
    resolve(stmt.statements);
    _endScope();
  }

  @override
  void visitCallExpr(CallExpr expr) {
    _resolveExpr(expr.callee);
    expr.arguments.forEach(_resolveExpr);
  }

  @override
  void visitExpressionStmt(ExpressionStmt stmt) =>
      _resolveExpr(stmt.expression);

  @override
  void visitFunctionStmt(FunctionStmt stmt) {
    _declare(stmt.name);
    _define(stmt.name);
    _resolveFunction(stmt, _FunctionType.FUNCTION);
  }

  @override
  void visitGroupingExpr(GroupingExpr expr) => _resolveExpr(expr.expression);

  @override
  void visitIfStmt(IfStmt stmt) {
    _resolveExpr(stmt.condition);
    _resolveStmt(stmt.thenBranch);
    if (stmt.elseBranch != null) {
      _resolveStmt(stmt.elseBranch!);
    }
  }

  @override
  void visitLiteralExpr(LiteralExpr expr) {}

  @override
  void visitLogicalExpr(LogicalExpr expr) {
    _resolveExpr(expr.left);
    _resolveExpr(expr.right);
  }

  @override
  void visitNil(Nil expr) {}

  @override
  void visitPrintStmt(PrintStmt stmt) => _resolveExpr(stmt.expression);

  @override
  void visitReturnStmt(ReturnStmt stmt) {
    if (_currentFunction == _FunctionType.NONE) {
      Lox.error(stmt.keyword, "Can't return from top-level code.");
    }
    if (stmt.value is! Nil) {
      _resolveExpr(stmt.value);
    }
  }

  @override
  void visitUnaryExpr(UnaryExpr expr) => _resolveExpr(expr.right);

  @override
  void visitVarStmt(VarStmt stmt) {
    _declare(stmt.name);
    if (stmt.initializer != null) {
      _resolveExpr(stmt.initializer!);
    }
    _define(stmt.name);
  }

  @override
  void visitVariableExpr(VariableExpr expr) {
    if (_scopes.isNotEmpty && _scopes.last[expr.name.lexeme] == false) {
      Lox.error(expr.name, "Can't read local variable in its own initializer.");
    }
    _resolveLocal(expr, expr.name);
  }

  @override
  void visitWhileStmt(WhileStmt stmt) {
    _resolveExpr(stmt.condition);
    _resolveStmt(stmt.body);
  }

  void resolve(List<Stmt> statements) => statements.forEach(_resolveStmt);

  void _resolveStmt(Stmt stmt) => stmt.accept(this);

  void _resolveExpr(Expr expr) => expr.accept(this);

  void _resolveLocal(Expr expr, Token name) {
    for (var i = _scopes.length - 1; i >= 0; i--) {
      if (_scopes[i].containsKey(name.lexeme)) {
        _interpreter.resolve(expr, _scopes.length - 1 - i);
        return;
      }
    }
  }

  void _resolveFunction(FunctionStmt func, _FunctionType type) {
    final enclosingFunction = _currentFunction;
    _currentFunction = type;

    _beginScope();
    func.params.forEach((param) {
      _declare(param);
      _define(param);
    });
    resolve(func.body);
    _endScope();

    _currentFunction = enclosingFunction;
  }

  void _beginScope() => _scopes.add(<String, bool>{});

  void _endScope() => _scopes.removeLast();

  void _declare(Token name) {
    if (_scopes.isNotEmpty) {
      final scope = _scopes.last;
      if (scope.containsKey(name.lexeme)) {
        Lox.error(name, 'Already variable with this name in this scope.');
      }
      scope[name.lexeme] = false;
    }
  }

  void _define(Token name) {
    if (_scopes.isNotEmpty) {
      _scopes.last[name.lexeme] = true;
    }
  }
}
