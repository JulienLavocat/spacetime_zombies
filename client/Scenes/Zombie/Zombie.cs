using Godot;

public partial class Zombie : Node3D
{
    [Export] public float InterpolationTime = 0.1f;

    private Vector3 startPosition, targetPosition;
    private float elapsed;
    private StandardMaterial3D _mat;
    private WorldManager _worldManager;

    public void SetWorldManager(WorldManager worldManager)
    {
        _worldManager = worldManager;
    }

    public void SetZombieState(SpacetimeDB.Types.Zombie state)
    {
        startPosition = Position;
        targetPosition = new Vector3(state.Position.X, state.Position.Y, state.Position.Z);
        elapsed = 0f;
    }

    public void SetSpitterZombieState(SpacetimeDB.Types.SpitterZombie state)
    {
        startPosition = Position;
        targetPosition = new Vector3(state.Position.X, state.Position.Y, state.Position.Z);
        elapsed = 0f;
    }

    public override void _Process(double delta)
    {
        elapsed += (float)delta;
        float t = Mathf.Clamp(elapsed / InterpolationTime, 0f, 1f);
        float smoothT = t * t * (3f - 2f * t);
        Position = startPosition.Lerp(targetPosition, smoothT);
        var playerPos = _worldManager.GetPlayerPosition();
        playerPos.Y = Position.Y;
        LookAt(playerPos, Vector3.Up);
    }
}
