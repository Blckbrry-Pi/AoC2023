import java.util.ArrayList;
import java.util.Scanner;
import java.io.File;

import java.util.regex.Matcher;
import java.util.regex.Pattern;



public class Solve {
    public static void main(String[] args) {
        ArrayList<Line> lines = new ArrayList<>();
        try {
            File inputFile = new File("./day1/input.txt");
            Scanner inputFileReader = new Scanner(inputFile);
            while (inputFileReader.hasNextLine()) {
                lines.add(new Line(inputFileReader.nextLine()));
            }
            inputFileReader.close();
        } catch (Exception e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        int part1 = 0;
        int part2 = 0;

        for (Line line : lines) {
            part1 += line.getValue();
            part2 += line.getPart2().getValue();
        }

        System.out.printf("Part 1: %d\n", part1);
        System.out.printf("Part 2: %d\n", part2);
    }
}

class Line {
    private String line;

    public Line(String line) {
        this.line = line;
    }

    public Line clone() {
        return new Line(line);
    }

    public Line getPart2() {
        String part2Line = line;
        Pattern pattern = Pattern.compile("one|two|three|four|five|six|seven|eight|nine");
        while (true) {
            Matcher matcher = pattern.matcher(part2Line);
            if (!matcher.find()) {
                break;
            }

            int index = matcher.start();
            String number = matcher.group();

            String before = part2Line.substring(0, index);
            String after = part2Line.substring(index + number.length());

            switch (number) {
                case "one":
                    part2Line = before + "1ne" + after;
                    break;
                case "two":
                    part2Line = before + "2wo" + after;
                    break;
                case "three":
                    part2Line = before + "3hree" + after;
                    break;
                case "four":
                    part2Line = before + "4our" + after;
                    break;
                case "five":
                    part2Line = before + "5ive" + after;
                    break;
                case "six":
                    part2Line = before + "6ix" + after;
                    break;
                case "seven":
                    part2Line = before + "7even" + after;
                    break;
                case "eight":
                    part2Line = before + "8ight" + after;
                    break;
                case "nine":
                    part2Line = before + "9ine" + after;
                    break;
            }
        }
        
        return new Line(part2Line);
    }

    public int getValue() {
        char[] chars = line.toCharArray();
        int first = -1;
        int last = -1;

        for (int i = 0; i < chars.length; i++) {
            if (chars[i] >= '0' && chars[i] <= '9') {
                if (first == -1) {
                    first = chars[i] - '0';
                }
                last = chars[i] - '0';
            }
        }

        return first * 10 + last;
    }
}

