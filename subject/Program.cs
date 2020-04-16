using System;

namespace subject
{
	public static class Constants
	{
		public const float Pi = 3.1415926f;
	}

	internal sealed class Program
	{
		private static readonly int Count = 16;

		public static void Main(string[] args)
		{
			Console.WriteLine("Hello, sailor!");
			Console.WriteLine("こんにちは! Count is {0}.", Count);
		}
	}
}
