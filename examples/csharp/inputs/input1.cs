// This example was copied from https://introcs.cs.luc.edu/data/sampleprogram.html

using System;

class Painting
{
    public int CalculateArea(double length, double width)
    {
        return length * width;
    }

    static void Main()
    {
        double width, length, wallArea, ceilingArea;
        string widthString, lengthString;
        double height = 8;

        Console.WriteLine ( "Calculation of Room Paint Requirements");
        Console.Write ( "Enter room length: ");
        lengthString = Console.ReadLine();
        length = double.Parse(lengthString);
        Console.Write( "Enter room width: ");
        widthString = Console.ReadLine();
        width = double.Parse(widthString);

        wallArea = 2 * (length + width) * height;
        ceilingArea = CalculateArea(length, width);

        Console.WriteLine("The wall area is " + wallArea
                          + " square feet.");
        Console.WriteLine("The ceiling area is " + ceilingArea
                          + " square feet.");
    }
}