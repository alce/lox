import Foundation

switch CommandLine.argc {
case 1:
    Lox.repl()
case 2:
    Lox.runFile(CommandLine.arguments[1])
default:
    print("Usage: slox [script]")
    exit(64)
}


