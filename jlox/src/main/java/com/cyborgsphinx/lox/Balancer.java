package com.cyborgsphinx.lox;

import java.util.ArrayDeque;
import java.util.Deque;

public class Balancer {
    private Deque<Character> unbalanced = new ArrayDeque<>();
    private int lines = 1;
    private StringBuilder buffer = new StringBuilder();

    enum UpdateType {
        Error,
        Unbalanced,
        Balanced
    }

    UpdateType update(String input) {
        if (input == null) return UpdateType.Error;
        for (char c : input.toCharArray()) {
            switch (c) {
                case '(':
                case '{':
                    unbalanced.push(c);
                    break;
                case ')':
                    if (unbalanced.peek() == '(') {
                        unbalanced.pop();
                    } else {
                        Lox.error(lines, "Too many ')' given.");
                        buffer.setLength(0);
                        return UpdateType.Error;
                    }
                    break;
                case '}':
                    if (unbalanced.peek() == '{') {
                        unbalanced.pop();
                    } else {
                        Lox.error(lines, "Too many '}' given.");
                        buffer.setLength(0);
                        return UpdateType.Error;
                    }
                    break;
                default: break; // ignore all other types
            }
        }
        lines++;
        buffer.append(input);
        return unbalanced.size() == 0 ? UpdateType.Balanced : UpdateType.Unbalanced;
    }

    String getBuffer() {
        String out = buffer.toString();
        buffer.setLength(0);
        return out;
    }
}
