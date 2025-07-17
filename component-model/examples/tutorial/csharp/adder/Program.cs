// Pull in all imports of the `hostapp` world, namely the `add` interface.
// example.component refers to the package name defined in the WIT file.
using HostappWorld.wit.imports.docs.adder.v0_1_0;

uint left = 1;
uint right = 2;
var result = AddInterop.Add(left, right);
Console.WriteLine($"{left} + {right} = {result}");
