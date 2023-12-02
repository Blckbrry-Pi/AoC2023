using System.Collections.Generic;

public class Solve
{
    static void Main(string[] args)
    {
        string[] strLines = System.IO.File.ReadAllLines("./day2/input.txt");

        Game[] games = new Game[strLines.Length];
        for (int i = 0; i < strLines.Length; i++) games[i] = new Game(strLines[i]);

        int part1 = 0, part2 = 0;

        foreach (Game game in games)
        {
            part1 += game.GetValuePart1();
            part2 += game.GetValuePart2();
        }

        System.Console.Write("Part 1: ");
        System.Console.WriteLine(part1);
        System.Console.Write("Part 2: ");
        System.Console.WriteLine(part2);
    }


}

class Game
{
    public int Id { get; private set; }
    public List<Round> Rounds { get; private set; }

    public Game(string line)
    {
        Id = Game.ParseId(line);
        Rounds = Game.ParseRounds(line);
    }

    private static int ParseId(string line)
    {
        int value = 0;
        int i = 0;
        while ('9' < line[i] || line[i] < '0') i++;
        while ('0' <= line[i] && line[i] <= '9')
        {
            value = value * 10 + line[i] - '0';
            i++;
        }

        return value;
    }

    private static List<Round> ParseRounds(string line)
    {
        string roundsStr = line.Substring(line.IndexOf(":") + 1);
        List<Round> rounds = new List<Round>();

        foreach (string roundText in roundsStr.Split(";"))
        {
            rounds.Add(new Round(roundText));
        }

        return rounds;
    }

    public Round MaxRound()
    {
        int r = 0, g = 0, b = 0;
        foreach (Round round in Rounds)
        {
            r = System.Math.Max(r, round.r);
            g = System.Math.Max(g, round.g);
            b = System.Math.Max(b, round.b);
        }
        return new Round(r, g, b);
    }

    public int GetValuePart1()
    {
        Round maxRound = MaxRound();
        if (maxRound.r <= 12 && maxRound.g <= 13 && maxRound.b <= 14)
        {
            return Id;
        } else
        {
            return 0;
        }
    }
    public int GetValuePart2()
    {
        Round maxRound = MaxRound();
        return maxRound.r * maxRound.g * maxRound.b;
    }

}

class Round
{
    public readonly int r;
    public readonly int g;
    public readonly int b;

    public Round(int r, int g, int b)
    {
        this.r = r;
        this.g = g;
        this.b = b;
    }

    public Round(string roundStr)
    {
        int r = 0, g = 0, b = 0;

        for (int i = 0; i < roundStr.Length; i++)
        {
            if (roundStr[i] == ',' || roundStr[i] == ' ') continue;

            int val = 0;
            while ('0' <= roundStr[i] && roundStr[i] <= '9')
            {
                val *= 10;
                val += roundStr[i] - '0';
                i++;
            }

            i++;

            switch (roundStr[i])
            {
                case 'r':
                    r = val;
                    i += "ed, ".Length;
                    break;

                case 'g':
                    g = val;
                    i += "reen, ".Length;
                    break;

                case 'b':
                    b = val;
                    i += "lue, ".Length;
                    break;
            }
        }

        this.r = r;
        this.g = g;
        this.b = b;
    }
}