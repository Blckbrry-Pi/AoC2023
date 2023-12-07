using System.Collections.Generic;

public class Solve
{

    static void DisplayRangeList(List<RangeMapping> ranges)
    {
        System.Console.WriteLine("Ranges:");
        foreach (RangeMapping rm in ranges)
        {
            System.Console.WriteLine(rm);
        }
        System.Console.WriteLine();
    }

    static List<Seed> Seeds(string seedLine) {
        List<Seed> seeds = new List<Seed>();
        foreach (string part in seedLine.Substring(7).Split(' ')) {
            seeds.Add(new Seed(System.Int64.Parse(part)));
        }
        return seeds;
    }
    static List<RangeEx> SeedsP2(string seedLine) {
        List<RangeEx> seeds = new List<RangeEx>();
        #nullable enable
        long? start = null;
        foreach (string part in seedLine.Substring(7).Split(' ')) {
            if (start == null)
            {
                start = System.Int64.Parse(part);
            }
            else
            {
                seeds.Add(new RangeEx(start ?? 0, (start ?? 0) + System.Int64.Parse(part)));
                start = null;
            }
        }
        return seeds;
    }
    static (List<RangeMapping>, int) rangeMappings(string[] lines, int idx)
    {
        List<RangeMapping> ranges = new List<RangeMapping>();
        while (idx < lines.Length && lines[idx] != "")
        {
            if ('0' <= lines[idx][0] && lines[idx][0] <= '9')
            {
                long dst = System.Int64.Parse(lines[idx].Split(' ')[0]);
                long src = System.Int64.Parse(lines[idx].Split(' ')[1]);
                long len = System.Int64.Parse(lines[idx].Split(' ')[2]);

                ranges.Add(new RangeMapping(
                    new RangeEx(src, src + len),
                    new RangeEx(dst, dst + len)
                ));
            }
            idx++;
        }
        return (ranges, idx + 1);
    }

    static void Main(string[] args)
    {
        System.Console.WriteLine("NOTE: This solve is incomplete and wrong!");
        System.Console.WriteLine("For that reason, it is not included in the day 5 script.");
        System.Console.WriteLine("I committed it because it felt wrong to give it absolutely 0 recognition.");
        System.Console.WriteLine();

        string[] strLines = System.IO.File.ReadAllLines("./day5/test.txt");

        List<RangeMapping> ranges;
        int idx = 2;
        List<Seed> seeds = Seeds(strLines[0]);
        List<RangeEx> seedsP2 = SeedsP2(strLines[0]);

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Seed.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Soil.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Fert.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Water.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Light.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Temp.RegisterMapping(rm);
        }

        (ranges, idx) = rangeMappings(strLines, idx);
        DisplayRangeList(ranges);
        foreach (RangeMapping rm in ranges)
        {
            Humid.RegisterMapping(rm);
        }



        long minLoc = 1000000000000000;
        long minSeed = 0;

        long minLocP2 = 1000000000000000;
        long minSeedP2 = 0;

        foreach (Seed seed in seeds)
        {
            System.Console.WriteLine(seed.Id);
            System.Console.WriteLine(seed.Soil.Id);
            System.Console.WriteLine(seed.Soil.Fert.Water.Light.Temp.Humid.Loc8.Id);
            System.Console.WriteLine();
            if (seed.Soil.Fert.Water.Light.Temp.Humid.Loc8.Id < minLoc) {
                minLoc = seed.Soil.Fert.Water.Light.Temp.Humid.Loc8.Id;
                minSeed = seed.Id;
            }
        }

        foreach (RangeEx seedPair in seedsP2) {
            System.Console.WriteLine(seedPair);
            List<RangeEx> soilRanges = Seed.SoilRanges(seedPair);
            foreach (RangeEx soilRange in soilRanges) {
                System.Console.WriteLine(soilRange);
                // if (range.Start < minLocP2) {
                //     minLocP2 = range.Start;
                //     minSeedP2 = seedPair.Start;
                // }
            }
        }

        System.Console.Write("Part 1: ");
        System.Console.WriteLine(minLoc);
        System.Console.Write("Part 2: ");
        System.Console.WriteLine(minLocP2);
    }


}

class RangeEx {
    public long Start;
    public long End;

    public RangeEx(long start, long end) {
        Start = start;
        End = end;
    }

    public bool Contains(long value) {
        return Start <= value && value < End;
    }

    public RangeEx Intersection(RangeEx other) {
        if (other.End <= Start || End <= other.Start) return new RangeEx(0, 0);
        long newStart = System.Math.Max(Start, other.Start);
        long newEnd = System.Math.Min(End, other.End);
        return new RangeEx(newStart, newEnd);
    }

    public RangeEx[] Difference(RangeEx other) {
        if (other.End <= Start || End <= other.Start) return new RangeEx[]{ new RangeEx(Start, End) };
        else if (other.Start >= Start)
        {
            if (other.End < End)
            {
                return new RangeEx[]{ new RangeEx(Start, other.Start), new RangeEx(other.End, End) };
            }
            else
            {
                return new RangeEx[]{ new RangeEx(Start, other.Start) };
            }
        }
        else if (other.End <= End)
        {
            return new RangeEx[]{ new RangeEx(other.End, End) };
        } else {
            return new RangeEx[0];
        }
    }

    public RangeEx[] Union (RangeEx other) {
        if (other.End <= Start || End <= other.Start) return new RangeEx[]{ new RangeEx(Start, End), new RangeEx(other.Start, other.End) };
        else if (other.Start >= Start)
        {
            if (other.End < End)
            {
                return new RangeEx[]{ new RangeEx(Start, End) };
            }
            else
            {
                return new RangeEx[]{ new RangeEx(Start, other.End) };
            }
        }
        else if (other.End <= End)
        {
            return new RangeEx[]{ new RangeEx(other.Start, End) };
        } else {
            return new RangeEx[]{ new RangeEx(other.Start, other.End) };
        }
    }



    public override int GetHashCode() {
        return (int) (Start * 5623 + End * 5869);
    }
    public override bool Equals(object obj) {
        if (obj == null) return false;
        if (!(obj is RangeEx)) return false;

        RangeEx other = (RangeEx)obj;
        return other.Start == Start && other.End == End;
    }

    public override string ToString() {
        return $"[{Start}, {End})";
    }
}

class RangeMapping {
    public RangeEx From;
    public RangeEx To;

    public RangeMapping(RangeEx from, RangeEx to) {
        From = from;
        To = to;
    }

    public override int GetHashCode() {
        return From.GetHashCode() * 2477 + To.GetHashCode() * 2749;
    }
    public override bool Equals(object obj) {
        if (obj == null) return false;
        if (!(obj is RangeMapping)) return false;

        RangeMapping other = (RangeMapping)obj;
        return other.From.Equals(From) && other.To.Equals(To);
    }

    public bool Contains(long value) {
        return From.Contains(value);
    }
    public bool ContainsRev(long value) {
        return To.Contains(value);
    }

    public RangeEx MappedIntersection(RangeEx inputRange) {
        if (From.Intersection(inputRange).Start == From.Intersection(inputRange).End) return new RangeEx(0, 0);
        long mappedStart = GetValue(From.Intersection(inputRange).Start) ?? 0;
        long mappedEnd = GetValue(From.Intersection(inputRange).End) ?? 0;
        return new RangeEx(mappedStart, mappedEnd);
    }

    #nullable enable
    public long? GetValue(long value) {
        if (!Contains(value)) return null;
        return value - From.Start + To.Start;
    }

    #nullable enable
    public long? GetValueRev(long value) {
        if (!ContainsRev(value)) return null;
        return value - To.Start + From.Start;
    }

    public override string ToString() {
        return $"{From} -> {To}";
    }
}

abstract class Item
{
    // protected abstract static List<RangeMapping> _toNextMap { get; }

    public long Id;

    protected long _getNextId(List<RangeMapping> mappings) {
        return mappings.Find(mapping => mapping.Contains(Id))?.GetValue(Id) ?? Id;
    }
    protected static List<RangeEx> _getNextIdRange(List<RangeMapping> mappings, RangeEx range) {
        List<RangeEx> unmapped = new List<RangeEx> { range };
        List<RangeEx> mapped = new List<RangeEx> {};

        bool wentThroughAndNothingChanged = false;
        while (!wentThroughAndNothingChanged)
        {
            List<RangeEx> newUnmapped = new List<RangeEx> {};
            wentThroughAndNothingChanged = true;

            System.Console.WriteLine("Unmapped:");
            foreach (RangeEx unmappedRange in unmapped)
            {
                System.Console.WriteLine(unmappedRange);
            }

            System.Console.WriteLine("Mapped:");
            foreach (RangeEx mappedRange in mapped)
            {
                System.Console.WriteLine(mappedRange);
            }

            foreach (RangeEx unmappedRange in unmapped)
            {
                bool wasMapped = false;
                foreach (RangeMapping mapping in mappings)
                {
                    RangeEx intersection = mapping.MappedIntersection(unmappedRange);
                    if (intersection.Start != intersection.End)
                    {
                        newUnmapped.AddRange(unmappedRange.Difference(intersection));
                        mapped.Add(mapping.MappedIntersection(unmappedRange));
                        wentThroughAndNothingChanged = false;
                        wasMapped = true;
                        break;
                    }
                }
                if (!wasMapped)
                {
                    newUnmapped.Add(unmappedRange);
                }
            }
            unmapped = newUnmapped;
        }

        List<RangeEx> outputs = new List<RangeEx> {  };

        foreach (RangeEx mappedRange in mapped)
        {
            outputs.Add(mappedRange);
        }

        foreach (RangeEx unmappedRange in unmapped)
        {
            outputs.Add(unmappedRange);
        }

        return outputs;
    }


    public Item(long id) {
        Id = id;
    }

    public override int GetHashCode() {
        return (int) Id;
    }

    public override bool Equals(object obj) {
        return obj is Item && ((Item)obj).Id == Id;
    }
}

class Seed : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();

    public Soil Soil {
        get {
            return new Soil(_getNextId(_toNextMap));
        }
    }
    public static List<RangeEx> SoilRanges(RangeEx range) {
        return _getNextIdRange(_toNextMap, range);
    }

    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Seed(long id) : base(id) {}
}
class Soil : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();
    public Fert Fert {
        get {
            return new Fert(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Soil(long id) : base(id) {}
}
class Fert : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();

    public Water Water {
        get {
            return new Water(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Fert(long id) : base(id) {}
}
class Water : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();

    public Light Light {
        get {
            return new Light(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Water(long id) : base(id) {}
}
class Light : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();
    public Temp Temp {
        get {
            return new Temp(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Light(long id) : base(id) {}
}
class Temp : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();
    public Humid Humid {
        get {
            return new Humid(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Temp(long id) : base(id) {}
}
class Humid : Item
{
    protected static List<RangeMapping> _toNextMap = new List<RangeMapping>();
    public Loc8 Loc8 {
        get {
            return new Loc8(_getNextId(_toNextMap));
        }
    }
    public static void RegisterMapping(RangeMapping mapping) {
        _toNextMap.Add(mapping);
    }

    public Humid(long id) : base(id) {}
}

class Loc8 : Item {
    public Loc8(long id) : base(id) {}
}
