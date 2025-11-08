using Godot;
using System.Collections.Generic;

public partial class WorldManager : Node3D
{
    [Export]
    private Stdb stdb;
    [Export]
    private PackedScene zombieScene;

    private readonly Dictionary<ulong, Zombie> zombies = new();

    public override void _Ready()
    {
        stdb.OnZombieInserted += (zombie) =>
        {
            var zombieNode = zombieScene.Instantiate<Zombie>();
            zombieNode.Name = "Zombie_" + zombie.Id;
            zombieNode.SetTargetPosition(new Vector3(zombie.Position.X, zombie.Position.Y, zombie.Position.Z));
            zombieNode.Position = new Vector3(zombie.Position.X, zombie.Position.Y, zombie.Position.Z);
            AddChild(zombieNode);
            zombies[zombie.Id] = zombieNode;
        };

        stdb.OnZombieUpdated += (oldZombie, newZombie) =>
        {
            if (zombies.TryGetValue(oldZombie.Id, out var node))
            {
                node.SetTargetPosition(new Vector3(newZombie.Position.X, newZombie.Position.Y, newZombie.Position.Z));
            }
        };

        stdb.OnZombieDeleted += (zombie) =>
        {
            if (zombies.TryGetValue(zombie.Id, out var node))
            {
                node.QueueFree();
                zombies.Remove(zombie.Id);
            }
        };
    }

    public override void _Process(double delta)
    {
    }
}
