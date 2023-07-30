namespace RedCsharp.Tests;

using Defs;
using NUnit.Framework;

public class UnitTest
{
    [Test]
    public void Test1()
    {
        Assert.That(Defs.Gabbro, Is.EqualTo("Gabbro"));
        Assert.That(Defs.Marble, Is.EqualTo("Marble"));
        Assert.That(Defs.Metamorphic, Is.EqualTo("Metamorphic"));
        Assert.That(Defs.ApplesCount, Is.EqualTo(236));
        Assert.That(Defs.OrangesCount, Is.EqualTo(454588979794318));
    }
}