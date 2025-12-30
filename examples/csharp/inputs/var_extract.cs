using System;

public class OrderCalculator
{
    public double CalculateFinalPrice(double price, int quantity, double discount, double shipping)
    {
        var value = (price * quantity) - discount + shipping;
        if (value > 1000.0)
        {
            Console.WriteLine("This is a high-value order.");
        }

        return (price * quantity) - discount + shipping;
    }
}
