package day03;

import java.util.ArrayList;
import java.util.HashSet;

import javautils.Utils;

public class Solve {
    public static void main(String[] args) {
        String[] lines = javautils.Utils.fileLines("day03/input.txt");
        Schematic schematic = new Schematic(lines);


        HashSet<NumberId> usedNumbers = new HashSet<>();
        HashSet<Symbol> symbols = new HashSet<>();
        HashSet<Star> stars = new HashSet<>();

        for (int y : javautils.Utils.range(0, schematic.getHeight())) {
            for (int x : javautils.Utils.range(0, schematic.getWidth())) {
                Token token = schematic.getTokenAt(x, y);
                if (token.isSymbol()) {
                    symbols.add((Symbol) token);

                    if (token instanceof Star) stars.add((Star) token);
                }
            }
        }


        int part1 = 0;
        int part2 = 0;

        for (Symbol symbol : symbols) {
            NumberId[] surrounding = symbol.getSurroundingNumbers(schematic);
            for (NumberId number : surrounding) {
                if (usedNumbers.contains(number)) continue;
                usedNumbers.add(number);

                int value = number.getNum(schematic);
                part1 += value;
            }
        }

        for (Star star : stars) {
            part2 += star.value(schematic);
        }

        System.out.printf("Part 1: %d\n", part1);
        System.out.printf("Part 2: %d\n", part2);
    }
}


class Schematic {
    private final Token[][] tokenized;
    private final int width;
    private final int height;

    public Schematic(String[] lines) {
        this.width = lines[0].length();
        this.height = lines.length;

        this.tokenized = Parser.parseLines(lines);
    }

    public Token getTokenAt(int x, int y) {
        if (0 <= x && x < width && 0 <= y && y < height) {
            return tokenized[y][x];
        } else {
            return new Dot(x, y);
        }
    }

    public Token getTokenAt(SchematicLocation location) {
        int x = location.getX();
        int y = location.getY();

        if (0 <= x && x < width && 0 <= y && y < height) {
            return tokenized[y][x];
        } else {
            return new Dot(x, y);
        }
    }

    public int getWidth() { return width; }
    public int getHeight() { return height; }
}



/**
 * <h3>A token on a schematic.</h3>
 * 
 * <i>Also see: Symbol, Number, Dot</i>
 */
abstract class Token {
    protected int x;
    protected int y;
    protected char chr;

    public int getX() { return x; }
    public int getY() { return y; }

    public Token(int x, int y, char chr) {
        this.x = x;
        this.y = y;
        this.chr = chr;
    }

    public char getChar() { return chr; }

    public abstract boolean isSymbol();
    public abstract boolean isNumber();

    public abstract char asSymbol();
    public abstract int asNumber(Schematic s);

    public SchematicLocation[] getParts(Schematic s) {
        return new SchematicLocation[]{ new SchematicLocation(x, y) };
    }
}



/**
 * <h3>A symbol on a schematic.</h3>
 * <p>It represents a relevant, non-number symbol.</p>
 * 
 * <i>Also see: Star</i>
 */
class Symbol extends Token {
    public Symbol(int x, int y, char chr) {
        super(x, y, chr);
    }

    public boolean isSymbol() { return true; }
    public boolean isNumber() { return false; }

    public char asSymbol() { return chr; }
    public int asNumber(Schematic s) { throw new ClassCastException("Token is not a number"); }

    public NumberId[] getSurroundingNumbers(Schematic s) {
        HashSet<NumberId> numbers = new HashSet<>();

        for (SchematicLocation point : getParts(s)) {
            for (SchematicLocation surrounding : point.getSurroundingPoints(s)) {
                Token token = s.getTokenAt(surrounding);
                if (!token.isNumber()) continue;

                Number num = (Number) token;
                numbers.add(num.getNumberId(s));
            }
        }

        return numbers.toArray(new NumberId[numbers.size()]);
    }
}


/**
 * <h3>A number on a schematic.</h3>
 * <p>It only contains a single digit, but the full number can be accessed with <code>asNumber</code></p>
 */
class Number extends Token {
    public Number(int x, int y, char chr) {
        super(x, y, chr);
    }

    public boolean isSymbol() { return false; }
    public boolean isNumber() { return true; }

    public char asSymbol() { throw new ClassCastException("Token is not a symbol"); }
    public int asNumber(Schematic s) {
        int value = 0;
        for (SchematicLocation digit : getParts(s)) {
            value *= 10;
            value += s.getTokenAt(digit).getChar() - '0';
        }
        return value;
    }

    public NumberId getNumberId(Schematic s) {
        return new NumberId(getParts(s)[0]);
    }

    @Override
    public SchematicLocation[] getParts(Schematic s) {
        SchematicLocation leftmost = new SchematicLocation(x, y);
        SchematicLocation rightmost = new SchematicLocation(x, y);

        while (s.getTokenAt(leftmost.l()).isNumber()) leftmost = leftmost.l();
        while (s.getTokenAt(rightmost.r()).isNumber()) rightmost = rightmost.r();

        int start = leftmost.getX();
        int end = rightmost.getX();

        SchematicLocation[] points = new SchematicLocation[end - start + 1];
        for (int x = start; x <= end; x++) {
            points[x - start] = new SchematicLocation(x, y);
        }

        return points;
    }
}

/**
 * <h3>A special case of a symbol. (a possible "gear")</h3>
 */
class Star extends Symbol {
    public Star(int x, int y) {
        super(x, y, '*');
    }

    public int value(Schematic s) {
        NumberId[] surrounding = getSurroundingNumbers(s);
        if (surrounding.length == 2) {
            return surrounding[0].getNum(s) * surrounding[1].getNum(s);
        } else {
            return 0;
        }
    }
}



/**
 * <h3>A dot on a schematic.</h3>
 * <p>It represents nothing!</p>
 */
class Dot extends Token {
    public Dot(int x, int y) {
        super(x, y, '.');
    }

    public boolean isSymbol() { return false; }
    public boolean isNumber() { return false; }

    public char asSymbol() { throw new ClassCastException("Token is not a symbol"); }
    public int asNumber(Schematic s) { throw new ClassCastException("Token is not a number"); }
}





/**
 * <h3>
 *  Represents a location on a schematic (cartesian based).
 * </h3>
 * 
 * <em>Note:</em> This class is immutable, and it supports `equals` and
 * `hashCode` for use in hash sets.
 * 
 * <p>Methods:</p>
 * <ul>
 *  <li><code>u</code>, <code>d</code>, <code>l</code>, <code>r</code> — return the location one step in that direction</li>
 *  <li><code>getSurroundingPoints</code> — return an array of all points surrounding this one</li>
 * </ul>
 */
class SchematicLocation {
    private final int x;
    private final int y;

    public SchematicLocation(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() { return x; }
    public int getY() { return y; }


    /**
     * <h3>An array of all points surrounding this one.</h3>
     * 
     * <p>The schematic is required so it doesn't return out-of-bounds points.</p>
     * 
     * @param s
     * @return an array of in-bounds all points surrounding this one
     */
    public SchematicLocation[] getSurroundingPoints(Schematic s) {
        ArrayList<SchematicLocation> points = new ArrayList<>();

        for (int x : javautils.Utils.rangeInclusive(-1, 1)) {
            for (int y : javautils.Utils.rangeInclusive(-1, 1)) {
                if (x == 0 && y == 0) continue;

                int newX = this.x + x;
                int newY = this.y + y;

                if (Utils.inRange(newX, s.getWidth()) && Utils.inRange(newY, s.getHeight())) {
                    points.add(new SchematicLocation(newX, newY));
                }
            }
        }

        return points.toArray(new SchematicLocation[points.size()]);
    }


    @Override
    public boolean equals(Object o) {
        if (o == this) return true;

        if (!(o instanceof SchematicLocation)) return false;
        SchematicLocation other = (SchematicLocation) o;

        return x == other.getX() && y == other.getY();
    }

    @Override
    public int hashCode() {
        return x * 5623 + y * 5869;
    }


    /** @return a new location one step up */
    public SchematicLocation u() { return new SchematicLocation(x, y - 1); }

    /** @return a new location one step down */
    public SchematicLocation d() { return new SchematicLocation(x, y + 1); }

    /** @return a new location one step left */
    public SchematicLocation l() { return new SchematicLocation(x - 1, y); }

    /** @return a new location one step right */
    public SchematicLocation r() { return new SchematicLocation(x + 1, y); }
}

/**
 * <h3>A location on a schematic that contains the leftmost digit a number.</h3>
 */
class NumberId extends SchematicLocation {
    public NumberId(SchematicLocation location) {
        super(location.getX(), location.getY());
    }

    public NumberId(int x, int y) {
        super(x, y);
    }

    public int getNum(Schematic s) {
        return s.getTokenAt(this).asNumber(s);
    }

    public String toString() {
        return String.format("NumberId(%d, %d)", getX(), getY());
    }

    @Override
    public boolean equals(Object o) {
        if (o == this) return true;

        if (!(o instanceof NumberId)) return false;
        NumberId other = (NumberId) o;

        return getX() == other.getX() && getY() == other.getY();
    }

    @Override
    public int hashCode() {
        return getX() * 5623 + getY() * 5869;
    }
}

abstract class Parser {
    public static Token parseToken(char chr, int x, int y) {
        if ('0' <= chr && chr <= '9') {
            return new Number(x, y, chr);
        } else if (chr == '.') {
            return new Dot(x, y);
        } else if (chr == '*') {
            return new Star(x, y);
        } else {
            return new Symbol(x, y, chr);
        }
    }

    public static Token[][] parseLines(String[] lines) {
        return Utils.Parser.byChars(
            lines,
            (coords, chr) -> parseToken((char) chr.intValue(), coords.getX(), coords.getY()),
            Token[]::new,
            Token[][]::new
        );
    }
}


