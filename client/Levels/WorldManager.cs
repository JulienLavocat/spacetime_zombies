using Godot;
using System.Collections.Generic;

public partial class WorldManager : Node3D
{
    [Export] private Stdb _stdb;
    [Export] private PackedScene _zombieScene;
    [Export] private PackedScene _spitterZombieScene;
    [Export] private PackedScene _playerScene;
    [Export] private PackedScene _spitterAoeScene;

    private readonly Dictionary<ulong, Zombie> zombies = new();
    private readonly Dictionary<ulong, MeshInstance3D> spitterAoes = new();

    private Node3D player;

    public override void _Ready()
    {
        _stdb.OnZombieInserted += (zombie) =>
        {
            var zombieNode = _zombieScene.Instantiate<Zombie>();
            zombieNode.Name = "Zombie_" + zombie.Id;
            zombieNode.SetWorldManager(this);
            zombieNode.SetZombieState(zombie);
            AddChild(zombieNode);
            zombies[zombie.Id] = zombieNode;
        };

        _stdb.OnZombieUpdated += (oldZombie, newZombie) =>
        {
            if (zombies.TryGetValue(oldZombie.Id, out var node))
            {
                node.SetZombieState(newZombie);
            }
        };

        _stdb.OnZombieDeleted += (zombie) =>
        {
            if (zombies.TryGetValue(zombie.Id, out var node))
            {
                node.QueueFree();
                zombies.Remove(zombie.Id);
            }
        };

        _stdb.OnSpitterZombieInserted += (zombie) =>
        {
            var zombieNode = _spitterZombieScene.Instantiate<Zombie>();
            zombieNode.Name = "SpitterZombie_" + zombie.Id;
            zombieNode.SetWorldManager(this);
            zombieNode.SetSpitterZombieState(zombie);
            AddChild(zombieNode);
            zombies[zombie.Id] = zombieNode;
        };

        _stdb.OnSpitterZombieUpdated += (oldZombie, newZombie) =>
        {
            if (zombies.TryGetValue(oldZombie.Id, out var node))
            {
                node.SetSpitterZombieState(newZombie);
            }
        };

        _stdb.OnSpitterZombieDeleted += (zombie) =>
        {
            if (zombies.TryGetValue(zombie.Id, out var node))
            {
                node.QueueFree();
                zombies.Remove(zombie.Id);
            }
        };

        _stdb.OnSpitterAoeInserted += (spitterAoe) =>
        {
            var aoeNode = _spitterAoeScene.Instantiate<MeshInstance3D>();
            aoeNode.Name = "SpitterAoe_" + spitterAoe.Id;
            aoeNode.GlobalPosition = new Vector3(spitterAoe.Position.X, spitterAoe.Position.Y, spitterAoe.Position.Z);
            AddChild(aoeNode);
            spitterAoes[spitterAoe.Id] = aoeNode;
        };

        _stdb.OnSpitterAoeDeleted += (spitterAoe) =>
        {
            if (spitterAoes.TryGetValue(spitterAoe.Id, out var node))
            {
                node.QueueFree();
                spitterAoes.Remove(spitterAoe.Id);
            }
        };

        GD.Print("WorldManager ready.");
        SpawnPlayer();
    }

    public void SpawnPlayer()
    {
        GD.Print("Spawning player...");
        var player = _playerScene.Instantiate<Player>();
        player.Name = "Player";
        player.GlobalPosition = new Vector3(8f, 0f, 2.25f);
        player.Stdb = _stdb;
        AddChild(player);
        this.player = player;
    }

    public Vector3 GetPlayerPosition()
    {
        return player.Position;
    }
}

