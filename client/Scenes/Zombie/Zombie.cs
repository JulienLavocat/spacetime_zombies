using Godot;

public partial class Zombie : Node3D
{
    [Export] public float InterpolationTime = 0.1f;

    private Vector3 startPosition;
    private Vector3 targetPosition;
    private float elapsed = 0f;

    public void SetTargetPosition(Vector3 position)
    {
        // When a new target arrives, restart interpolation
        startPosition = Position;
        targetPosition = position;
        elapsed = 0f;
    }

    public override void _Process(double delta)
    {
        // Advance interpolation timer
        elapsed += (float)delta;

        // Clamp to avoid overshooting
        float t = Mathf.Clamp(elapsed / InterpolationTime, 0f, 1f);

        // Smoothstep easing (optional; can use t directly for linear)
        float smoothT = t * t * (3f - 2f * t);

        // Interpolate between old and new target
        Position = startPosition.Lerp(targetPosition, smoothT);
    }
}
