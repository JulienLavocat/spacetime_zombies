server:
    spacetime publish zombies -s local -c -y -p server

update:
    spacetime publish zombies -s local -y -p server

bindings:
    spacetime generate --out-dir client/SpacetimeDB --lang csharp -p server

upload-navmesh:
    spacetime call -s local zombies editor_upload_navmesh 1
