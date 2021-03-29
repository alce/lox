import Foundation

class Lexer {
    private let source: String
    private var line: Int = 1;
    
    var tokens: [Token] = [];
    
    init(source: String) {
        self.source = source
    }
    
    func scan() -> [Token] {
        for c in source {
            switch c {
            case "(" : addToken(.LEFT_PAREN)
            case ")" : addToken(.RIGHT_PAREN)
            case "{" : addToken(.LEFT_BRACE)
            case "}" : addToken(.RIGHT_BRACE)
            case "," : addToken(.COMMA)
            case "." : addToken(.DOT)
            case "-" : addToken(.MINUS)
            case "+" : addToken(.PLUS)
            case ";" : addToken(.SEMICOLON)
            case "*" : addToken(.STAR)
            default: Lox.error(line: line, message: "Unexpected character.")
            }
        }
        
        tokens.append(Token(type: .EOF, line: line))
        return tokens
    }
    
    private func addToken(_ type: TokenType) {
        tokens.append(Token(type: type, line: line))
    }
}
