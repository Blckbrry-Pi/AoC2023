package javautils;

import java.util.*;
import java.util.stream.*;

import java.util.function.*;
import java.io.*;

public final class Utils {
    public static class Coords {
        private final int x;
        private final int y;

        public Coords(int x, int y) {
            this.x = x;
            this.y = y;
        }
        public Coords(Coords coords) {
            this.x = coords.x;
            this.y = coords.y;
        }

        public int getX() { return this.x; }
        public int getY() { return this.y; }
    }

    public static abstract class Parser {
        public static<T> T[] byChars(String line, IntFunction<T> charMapper, IntFunction<T[]> arrayMaker) {
            return line.chars().mapToObj(charMapper).toArray(arrayMaker);
        }
        public static<T> T[] byChars(String line, BiFunction<Integer, Integer, T> charMapper, IntFunction<T[]> arrayMaker) {
            return IntStream
                .range(0, line.length())
                .mapToObj(x -> charMapper.apply(x, (int)line.charAt(x)))
                .toArray(arrayMaker);
        }
        
        public static<T> T[][] byChars(String[] lines, IntFunction<T> charMapper, IntFunction<T[]> arrayMaker, IntFunction<T[][]> array2DMaker) {
            return Arrays.stream(lines).map(line -> byChars(line, charMapper, arrayMaker)).toArray(array2DMaker);
        }
        public static<T> T[][] byChars(String[] lines, BiFunction<Coords, Integer, T> charMapper, IntFunction<T[]> arrayMaker, IntFunction<T[][]> array2DMaker) {
            return IntStream
                .range(0, lines.length)
                .mapToObj(y -> byChars(
                    lines[y],
                    (x, chr) -> charMapper.apply(
                        new Coords(x, y),
                        (int)lines[y].charAt(x)
                    ),
                    arrayMaker
                ))
                .toArray(array2DMaker);
        }


    }

    public static Iterable<Integer> range(int start, int end) {
        return IntStream.range(start, end)::iterator;
    }
    public static Iterable<Integer> rangeInclusive(int start, int end) {
        return IntStream.rangeClosed(start, end)::iterator;
    }

    public static Iterable<Integer> range(int end) {
        return range(0, end);
    }
    public static Iterable<Integer> rangeInclusive(int end) {
        return rangeInclusive(0, end);
    }

    public static Iterable<Integer> range(int start, int end, int step) {
        return IntStream.range(start, end).filter(n -> (n - start) % step == 0)::iterator;
    }
    public static Iterable<Integer> rangeInclusive(int start, int end, int step) {
        return IntStream.rangeClosed(start, end).filter(n -> (n - start) % step == 0)::iterator;
    }


    public static boolean inRange(int n, int start, int end) {
        return start <= n && n < end;
    }
    public static boolean inRangeInclusive(int n, int start, int end) {
        return start <= n && n <= end;
    }

    public static boolean inRange(int n, int end) {
        return inRange(n, 0, end);
    }
    public static boolean inRangeInclusive(int n, int end) {
        return inRangeInclusive(n, 0, end);
    }


    public static String[] fileLines(String filename) {
        ArrayList<String> lines = new ArrayList<>();

        try {
            File inputFile = new File("./day3/input.txt");
            Scanner inputFileReader = new Scanner(inputFile);
            while (inputFileReader.hasNextLine()) {
                lines.add(inputFileReader.nextLine());
            }
            inputFileReader.close();
        } catch (java.io.IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
            System.out.println("Cancelling file read...");
        }

        return lines.toArray(new String[lines.size()]);
    }


}
