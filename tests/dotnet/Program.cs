using System;
using System.Runtime.InteropServices;

class Program
{
    static class Native
    {
        [DllImport("native_test", EntryPoint = "run", ExactSpelling = true)]
        public static extern bool Run();
    }

    static void Main(string[] args)
    {
        var success = Native.Run();

        if (!success)
        {
            throw new Exception("Native call failed");
        }
    }
}
