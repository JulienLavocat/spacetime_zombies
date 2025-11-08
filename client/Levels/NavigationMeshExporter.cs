using Godot;
using SpacetimeDB.Types;
using System.Linq;
using System.Collections.Generic;
using System.Text.Json;

[Tool]
public partial class NavigationMeshExporter : NavigationRegion3D
{

    [ExportToolButton("Export NavMesh")]
    public Callable ExportButton => Callable.From(ExportNavmesh);

    public void ExportNavmesh()
    {
        if (NavigationMesh == null)
        {
            GD.PushError("No NavigationMesh on this region.");
            return;
        }

        GD.Print("Exporting NavMesh...");

        GD.Print(NavigationMesh.GetPolygonCount());

        var toWorld = GlobalTransform;
        var verts = NavigationMesh.GetVertices()
            .Select(v => new Vec3(v.X, v.Y, v.Z))
            .ToList();
        var polygons = new List<List<ulong>>();
        for (int i = 0; i < NavigationMesh.GetPolygonCount(); i++)
        {
            var list = new List<ulong>();
            var polyVerts = NavigationMesh.GetPolygon(i);
            foreach (var vertIndex in polyVerts)
            {
                list.Add((ulong)vertIndex);
            }
            polygons.Add(list);
        }

        var navmesh = new NavMesh
        {
            Id = 0,
            PolygonTypeIndices = Enumerable.Repeat(0ul, polygons.Count).ToList(),
            Rotation = 0f,
            Translation = new Vec3(0, 0, 0),
            Vertices = verts,
            Polygons = polygons,
            WorldId = 1,
        };


        var options = new JsonSerializerOptions
        {
            WriteIndented = false,
            IncludeFields = true,
            DictionaryKeyPolicy = JsonNamingPolicy.SnakeCaseLower,
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
            DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.Never
        };
        var json = JsonSerializer.Serialize(navmesh, options);
        var file = FileAccess.Open("res://navmesh_export.json", FileAccess.ModeFlags.Write);
        file.StoreString(json);
        file.Close();

        GD.Print("NavMesh exported to res://navmesh_export.json");

    }

}
