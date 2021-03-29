import Foundation

class Lox {
    static var hadError = false
    
    static func runFile(_ file: String) {
        do {
            let source = try String(contentsOfFile: file, encoding: .utf8);
            run(source)
            
            if hadError {
                exit(65)
            }
        }
        catch {
            print(error)
        }
    }

    static func repl() {
        func prompt() {
            print("> ", terminator: "")
        }
        
        prompt()
        while let line = readLine() {
            run(line)
            hadError = false
            prompt()
        }
    }
    
    static func error(line: Int, message: String) {
        report(line: line, location: "", message: message)
    }

    static func report(line: Int, location: String, message: String) {
        FileHandle
            .standardError
            .write(Data("[line \(line)] Error\(location): \(message)".utf8))
        
        hadError = true
    }

    static private func run(_ source: String) {
        let lexer = Lexer(source: source);
        let tokens = lexer.scan();
        

        for token in tokens {
            print(token)
        }
    }
}
