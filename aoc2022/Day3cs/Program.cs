// See https://aka.ms/new-console-template for more information
using BenchmarkDotNet.Running;

Console.WriteLine("Hello, World!");

// Day3_Char.Solve();


var summary = BenchmarkRunner.Run<Day3Benchmark>();