public class Solve
{
    static void Main(string[] args)
    {
        string[] strLines = System.IO.File.ReadAllLines("./day1/input.txt");
        Line[] lines = new Line[strLines.Length];
        for (int i = 0; i < strLines.Length; i++) lines[i] = new Line(strLines[i]);

        int part1 = 0;
        int part2 = 0;

        foreach (Line line in lines)
        {
            part1 += line.Value();
            part2 += line.Part2().Value();
        }

        System.Console.Write("Part 1:");
        System.Console.WriteLine(part1);
        System.Console.Write("Part 2:");
        System.Console.WriteLine(part2);
    }


}

class Line
{
    private readonly string line;

    public Line(string line)
    {
        this.line = line;
    }

    public int Value() {
        int first = -1;
        int last = -1;
        for (int i = 0; i < line.Length; i++)
        {
            if ('0' <= line[i] && line[i] <= '9')
            {
                if (first == -1)
                {
                    first = line[i] - '0';
                }
                last = line[i] - '0';
            }
        }
        return first * 10 + last;
    }

    public Line Part2()
    {
        string part2Line = line;
        for (int i = 0; i < part2Line.Length; i++)
        {
            bool is_1 = part2Line.Substring(i).StartsWith("one");
            bool is_2 = part2Line.Substring(i).StartsWith("two");
            bool is_3 = part2Line.Substring(i).StartsWith("three");
            bool is_4 = part2Line.Substring(i).StartsWith("four");
            bool is_5 = part2Line.Substring(i).StartsWith("five");
            bool is_6 = part2Line.Substring(i).StartsWith("six");
            bool is_7 = part2Line.Substring(i).StartsWith("seven");
            bool is_8 = part2Line.Substring(i).StartsWith("eight");
            bool is_9 = part2Line.Substring(i).StartsWith("nine");

            int value = 0
                + (is_1 ? 1 : 0)
                + (is_2 ? 2 : 0)
                + (is_3 ? 3 : 0)
                + (is_4 ? 4 : 0)
                + (is_5 ? 5 : 0)
                + (is_6 ? 6 : 0)
                + (is_7 ? 7 : 0)
                + (is_8 ? 8 : 0)
                + (is_9 ? 9 : 0);

            if (value != 0)
            {
                part2Line = part2Line.Substring(0, i) + value.ToString() + part2Line.Substring(i + 1);
            }
        }
        return new Line(part2Line);
    }
}