using Godot;
using SpacetimeDB.Types;

public partial class Stdb : Node
{

    private DbConnection _connection;

    public bool IsActive => _connection.IsActive;

    public delegate void OnZombieInsertedDelegate(SpacetimeDB.Types.Zombie zombie);
    public event OnZombieInsertedDelegate OnZombieInserted;

    public delegate void OnZombieUpdatedDelegate(SpacetimeDB.Types.Zombie oldZombie, SpacetimeDB.Types.Zombie newZombie);
    public event OnZombieUpdatedDelegate OnZombieUpdated;

    public delegate void OnZombieDeletedDelegate(SpacetimeDB.Types.Zombie zombie);
    public event OnZombieDeletedDelegate OnZombieDeleted;

    public override void _Ready()
    {
        _connection = DbConnection.Builder()
            .WithUri("ws://localhost:4000")
            .WithModuleName("zombies")
            .WithLightMode(true)
            .OnConnect((ctx, identity, _) =>
            {
                GD.Print("Connected to SpacetimeDB as " + identity);
                ctx.Reducers.PlayerReady();
            })
            .OnConnectError((error) =>
            {
                GD.PrintErr("Connection error: " + error);
            })
            .OnDisconnect((_, reason) =>
            {
                GD.Print("Disconnected from SpacetimeDB: " + reason);
            })
            .Build();

        _connection.Reducers.OnPlayerReady += (ctx) =>
        {
            GD.Print("Player is ready in the game world.");
            ctx.SubscriptionBuilder().OnApplied((_) =>
            {
                GD.Print("Initial data synchronization complete.");

            }).OnError((_, err) =>
            {
                GD.PrintErr("Subscription error: " + err);
            })
            .SubscribeToAllTables();
        };

        _connection.Db.Zombie.OnInsert += (ctx, zombie) =>
        {
            OnZombieInserted?.Invoke(zombie);
        };

        _connection.Db.Zombie.OnUpdate += (ctx, oldZombie, newZombie) =>
        {
            OnZombieUpdated?.Invoke(oldZombie, newZombie);
        };

        _connection.Db.Zombie.OnDelete += (ctx, zombie) =>
        {
            OnZombieDeleted?.Invoke(zombie);
        };
    }

    public override void _Process(double delta)
    {
        _connection.FrameTick();
    }

    public RemoteReducers Reducers()
    {
        return _connection.Reducers;
    }

}
