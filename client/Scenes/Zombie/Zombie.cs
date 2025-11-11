using Godot;

public partial class Zombie : Node3D
{
    [Export] public float InterpolationTime = 0.1f;
    [Export] public MeshInstance3D MeshInstance;

    private Vector3 startPosition, targetPosition;
    private float elapsed;
    private StandardMaterial3D _mat;

    public override void _Ready()
    {
        if (MeshInstance == null || MeshInstance.Mesh == null)
        {
            GD.PushWarning($"{Name}: MeshInstance or Mesh is null.");
            return;
        }

        _mat = MeshInstance.MaterialOverride as StandardMaterial3D;

        if (_mat == null)
        {
            _mat = new StandardMaterial3D
            {
                ResourceLocalToScene = true,
            };
            MeshInstance.MaterialOverride = _mat;
        }
        else
        {
            _mat = (StandardMaterial3D)_mat.Duplicate();
            _mat.ResourceLocalToScene = true;
            MeshInstance.MaterialOverride = _mat;
        }

        _mat.AlbedoColor = Colors.Green;
    }

    public void SetTargetPosition(Vector3 position)
    {
        startPosition = Position;
        targetPosition = position;
        elapsed = 0f;
    }

    public void SetAttacking(bool attacking)
    {
        if (_mat != null)
            _mat.AlbedoColor = attacking ? Colors.Red : Colors.Green;
    }

    public override void _Process(double delta)
    {
        elapsed += (float)delta;
        float t = Mathf.Clamp(elapsed / InterpolationTime, 0f, 1f);
        float smoothT = t * t * (3f - 2f * t);
        Position = startPosition.Lerp(targetPosition, smoothT);
    }
}
