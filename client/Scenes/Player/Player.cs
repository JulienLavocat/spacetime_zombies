using Godot;
using SpacetimeDB.Types;

public partial class Player : CharacterBody3D
{
    private readonly float WalkSpeed = 5.0f;
    private readonly float SprintSpeed = 10.0f;
    private readonly float JumpVelocity = 4.5f;
    private readonly float Gravity = -9.8f;
    private readonly float MouseSensitivityX = 0.3f;
    private readonly float MouseSensitivityY = 0.3f;
    private readonly float BoBFrequency = 2.0f;
    private readonly float BoBAmplitude = 0.05f;
    private readonly float BaseFOV = 80.0f;
    private readonly float FOVSprintMultiplier = 1.1f;

    [Export]
    public Stdb stdb;

    private Node3D head;
    private Camera3D camera;
    private float bobTimer = 0.0f;
    private float lastPositionUpdateTime = 0.0f;
    private Vector3 lastSentPosition = Vector3.Zero;


    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        Input.SetMouseMode(Input.MouseModeEnum.Captured);
        head = GetNode<Node3D>("Head");
        camera = head.GetNode<Camera3D>("Camera");
    }

    public override void _UnhandledInput(InputEvent @event)
    {
        if (@event is InputEventMouseMotion mouseMotion)
        {
            head.RotateY(Mathf.DegToRad(-mouseMotion.Relative.X * MouseSensitivityX));
            camera.RotateX(Mathf.DegToRad(-mouseMotion.Relative.Y * MouseSensitivityY));
            var rotation = camera.RotationDegrees;
            rotation.X = Mathf.Clamp(rotation.X, -90, 90);
            camera.RotationDegrees = rotation;
        }
    }

    public override void _PhysicsProcess(double delta)
    {
        Vector3 velocity = Velocity;
        if (!IsOnFloor())
        {
            velocity.Y += Gravity * (float)delta;
        }

        if (Input.IsActionJustPressed("Jump") && IsOnFloor())
        {
            velocity.Y = JumpVelocity;
        }

        float speed = WalkSpeed;

        if (Input.IsActionPressed("Sprint"))
        {
            speed = SprintSpeed;
        }


        Vector2 inputDirection = Input.GetVector("MoveLeft", "MoveRight", "MoveForward", "MoveBackward");
        Vector3 direction = (head.Transform.Basis * new Vector3(inputDirection.X, 0, inputDirection.Y)).Normalized();
        if (IsOnFloor())
        {
            if (direction != Vector3.Zero)
            {
                velocity.X = direction.X * speed;
                velocity.Z = direction.Z * speed;
            }
            else
            {
                velocity.X = Mathf.Lerp(velocity.X, direction.X * speed, (float)delta * 7.0f);
                velocity.Z = Mathf.Lerp(velocity.Z, direction.Z * speed, (float)delta * 7.0f);
            }
        }
        else
        {
            velocity.X = Mathf.Lerp(velocity.X, direction.X * speed, (float)delta * 2.0f);
            velocity.Z = Mathf.Lerp(velocity.Z, direction.Z * speed, (float)delta * 2.0f);
        }


        Velocity = velocity;
        MoveAndSlide();

        var velocityClamped = Mathf.Clamp(velocity.Length(), 0.5f, SprintSpeed * 2);
        var targetFOV = BaseFOV + FOVSprintMultiplier * velocityClamped;
        camera.Fov = Mathf.Lerp(camera.Fov, targetFOV, (float)delta * 8.0f);

        bobTimer += (float)delta * velocity.Length() * (IsOnFloor() ? 1 : 0);
        camera.Position = Headbob(bobTimer);
    }

    public override void _Process(double delta)
    {
        if (!stdb.IsActive)
        {
            return;
        }

        lastPositionUpdateTime += (float)delta;
        if (lastPositionUpdateTime >= 0.1f && GlobalPosition.DistanceSquaredTo(lastSentPosition) > 0.01f)
        {
            stdb.Reducers().PlayerUpdatePosition(new Vec3(GlobalPosition.X, GlobalPosition.Y, GlobalPosition.Z));
            lastPositionUpdateTime = 0.0f;
        }
    }

    private Vector3 Headbob(float time)
    {
        return new Vector3(
            Mathf.Cos(time * BoBFrequency * 0.5f) * BoBAmplitude,
            Mathf.Sin(time * BoBFrequency) * BoBAmplitude,
            0
        );
    }
}
