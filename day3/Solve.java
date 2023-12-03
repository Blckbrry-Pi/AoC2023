package day3;

import java.util.ArrayList;
import java.util.HashSet;

import java.util.Scanner;
import java.io.File;


// FIXME: Self-explanatory
public class Solve {
    public static void main(String[] args) {
        ArrayList<String> strLines = new ArrayList<>();
        try {
            File inputFile = new File("./day3/input.txt");
            Scanner inputFileReader = new Scanner(inputFile);
            while (inputFileReader.hasNextLine()) {
                strLines.add(inputFileReader.nextLine());
            }
            inputFileReader.close();
        } catch (Exception e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        String[] lines = strLines.toArray(new String[strLines.size()]);
        Schematic schematic = new Schematic(lines);

        HashSet<SchematicLocation> numbers = new HashSet<>();

        HashSet<SchematicLocation> usedNumbers = new HashSet<>();

        for (int x = 0; x < schematic.getWidth(); x++) {
            for (int y = 0; y < schematic.getHeight(); y++) {
                Token token = schematic.getTokenAt(x, y);
                if (token.isNumber()) {
                    numbers.add(new SchematicLocation(x, y));
                }
            }
        }

        int part1 = 0;
        int part2 = 0;

        for (int x = 0; x < schematic.getWidth(); x++) {
            for (int y = 0; y < schematic.getHeight(); y++) {
                Token token = schematic.getTokenAt(x, y);
                if (token.isSymbol()) {
                    System.out.printf("found symbol @ (%d, %d)\n", x, y);
                    SchematicLocation[] surrounding = new SchematicLocation(x, y).getSurroundingPoints(schematic.getWidth(), schematic.getHeight());
                    ArrayList<Integer> adjacentNumbers = new ArrayList<>();

                    for (SchematicLocation point : surrounding) {
                        if (numbers.contains(point)) {
                            Token tk = schematic.getTokenAt(point);
                            SchematicLocation numberIdent = tk.getParts(schematic)[0];
                            if (usedNumbers.contains(numberIdent)) continue;
                            usedNumbers.add(numberIdent);

                            int value = tk.asNumber(schematic);

                            part1 += value;
                            adjacentNumbers.add(value);
                            System.out.printf("found number at (%d, %d)\n", point.getX(), point.getY());
                        }
                    }

                    if (token.chr == '*' && adjacentNumbers.size() == 2) {
                        part2 += adjacentNumbers.get(0) * adjacentNumbers.get(1);
                    }
                }
            }
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

        this.tokenized = new Token[height][width];

        for (int y = 0; y < height; y++) {
            char[] chars = lines[y].toCharArray();
            for (int x = 0; x < width; x++) {
                char chr = chars[x];
                if (chr >= '0' && chr <= '9') {
                    tokenized[y][x] = new Number(x, y, chr);
                } else if (chr == '.') {
                    tokenized[y][x] = new Dot(x, y, chr);
                } else {
                    tokenized[y][x] = new Symbol(x, y, chr);
                }
            }
        }
    }

    public Token getTokenAt(int x, int y) {
        return tokenized[y][x];
    }

    public Token getTokenAt(SchematicLocation location) {
        return tokenized[location.getY()][location.getX()];
    }

    public int getWidth() { return width; }
    public int getHeight() { return height; }
}



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



class Symbol extends Token {
    public Symbol(int x, int y, char chr) {
        super(x, y, chr);
    }

    public boolean isSymbol() { return true; }
    public boolean isNumber() { return false; }

    public char asSymbol() { return chr; }
    public int asNumber(Schematic s) { throw new ClassCastException("Token is not a number"); }
}

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

    @Override
    public SchematicLocation[] getParts(Schematic s) {
        SchematicLocation leftmost = new SchematicLocation(x, y);
        SchematicLocation rightmost = new SchematicLocation(x, y);

        // System.out.printf("number '%c' @ (%d, %d)\n", chr, x, y);

        while (true) {
            SchematicLocation toCheck = new SchematicLocation(leftmost.getX() - 1, leftmost.getY());
            // System.out.printf("checking left (%d, %d)\n", toCheck.getX(), toCheck.getY());

            if (toCheck.getX() < 0) break;

            if (s.getTokenAt(toCheck).isNumber()) {
                leftmost = toCheck;
            } else {
                break;
            }
        }

        while (true) {
            SchematicLocation toCheck = new SchematicLocation(rightmost.getX() + 1, rightmost.getY());
            // System.out.printf("checking right (%d, %d)\n", toCheck.getX(), toCheck.getY());
            if (toCheck.getX() >= s.getWidth()) break;

            if (s.getTokenAt(toCheck).isNumber()) {
                rightmost = toCheck;
            } else {
                break;
            }
        }

        ArrayList<SchematicLocation> points = new ArrayList<>();

        for (int x = leftmost.getX(); x <= rightmost.getX(); x++) {
            points.add(new SchematicLocation(x, y));
        }

        return points.toArray(new SchematicLocation[points.size()]);
    }
}

class Dot extends Token {
    public Dot(int x, int y, char chr) {
        super(x, y, chr);
    }

    public boolean isSymbol() { return false; }
    public boolean isNumber() { return false; }

    public char asSymbol() { throw new ClassCastException("Token is not a symbol"); }
    public int asNumber(Schematic s) { throw new ClassCastException("Token is not a number"); }
}


class SchematicLocation {
    private int x;
    private int y;

    public SchematicLocation(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public int getX() { return x; }
    public int getY() { return y; }

    public SchematicLocation[] getSurroundingPoints(int width, int height) {
        ArrayList<SchematicLocation> points = new ArrayList<>();

        for (int x = -1; x <= 1; x++) {
            for (int y = -1; y <= 1; y++) {
                if (x == 0 && y == 0) continue;

                int newX = this.x + x;
                int newY = this.y + y;

                if (0 <= newX && newX < width && 0 <= newY && newY < height) {
                    points.add(new SchematicLocation(newX, newY));
                }
            }
        }

        return points.toArray(new SchematicLocation[points.size()]);
    }

    public boolean equals(Object o) {
        if (o == this) return true;

        if (!(o instanceof SchematicLocation)) return false;
        SchematicLocation other = (SchematicLocation) o;

        return x == other.getX() && y == other.getY();
    }


    public int hashCode() {
        return x * 5623 + y * 5869;
    }
}
