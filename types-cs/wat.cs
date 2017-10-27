using System;

namespace wat
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(ReturnsInt());

            Console.WriteLine(ReturnsString());

            Console.WriteLine(ReturnsMyClass());
        }

        // expected return type == int
        // actual Result<int, Exception>
        static int ReturnsInt() {
            return 42;

            // OR

            throw new Exception();
        }

        // expected return type == string
        // actual Result<Option<String>, Exception>
        static string ReturnsString() {
            return "Hello World!!!";

            // OR

            return null;

            // OR

            throw new Exception();
        }


        // expected return type == MyClass
        // actual Result<Option<MyClass>, Exception>
        static string ReturnsMyClass() {
            return new MyClass();

            // OR

            return null;

            // OR

            throw new Exception();
        }
    }

    public class MyClass {
        // properties and stuff
    }
}