import bpy
import os

data = bpy.context.active_object.data
name = bpy.context.active_object.name

# Extract vertex color information. Translate color into a bitfield
# so that exact colors do not matter.
vert_colors = {}
for i in range(0, len(data.vertex_colors[0].data)):
    r = int(data.vertex_colors[0].data[i].color[0] + 0.5)
    g = int(data.vertex_colors[0].data[i].color[1] + 0.5)
    b = int(data.vertex_colors[0].data[i].color[2] + 0.5)
    c = (r * 4) + (g * 2) + b
    vert_colors[data.loops[i].vertex_index] = c

# Translation table for vertex color to corner/edge piece color
# index used during the cube model creation code.
color_table = {0: -1, 7: 0, 4: 1, 2: 2}
shared_edge_color = 1

# Process polygons. We will add vertices as we need them because we
# want to duplicate shared edges (marked in blue in the model).
verts = []
vert_map = {}
shared_vert_map = {0: {}, 2: {}, 4: {}, 7: {}}
index = []
for poly in data.polygons:
    # Determine the primary (non-shared) color of this polygon
    primary = None
    for i in poly.vertices:
        if vert_colors[i] != shared_edge_color:
            if (primary is not None) and (primary != vert_colors[i]):
                raise Exception("Shared colors not marked shared")
            primary = vert_colors[i]

    if len(poly.vertices) != 3:
        raise Exception("Polygon is not a triangle")

    for i in poly.vertices:
        if vert_colors[i] == shared_edge_color:
            # This is a shared edge. Use the shared_vert_map, which
            # also includes the primary color for unique checks. This
            # will copy the vertex and allow a hard color edge in
            # the final model.
            if i not in shared_vert_map[primary]:
                shared_vert_map[primary][i] = len(verts)
                verts.append((data.vertices[i], color_table[primary]))
            index.append(shared_vert_map[primary][i])
        else:
            if i not in vert_map:
                vert_map[i] = len(verts)
                verts.append((data.vertices[i], color_table[primary]))
            index.append(vert_map[i])

# Output model
path = __file__
while os.path.basename(path) != 'tools':
    path = os.path.dirname(path)
path = os.path.join(os.path.dirname(path), f"src/{name}_generated.rs")

f = open(path, 'w')
f.write('// This file was autogenerated by tools/blend_to_struct.py\n')
f.write('use crate::cube::SourceVertex;\n\n')

f.write('pub const ' + name.upper() + '_SOURCE_VERTS: &\'static [SourceVertex] = &[\n')
for vert in verts:
    pos = vert[0].co
    normal = vert[0].normal
    normal = [normal.x, normal.y, normal.z]

    # Correct normals of faces
    if abs(pos.x) == 1 or abs(pos.y) == 1 or abs(pos.z) == 1:
        normal[0] = float(int(normal[0] * 1.5))
        normal[1] = float(int(normal[1] * 1.5))
        normal[2] = float(int(normal[2] * 1.5))

    f.write('    SourceVertex {\n')
    f.write(f'        pos: [{pos.x:.7}, {pos.y:.7}, {pos.z:.7}],\n')
    f.write(f'        normal: [{normal[0]:.7}, {normal[1]:.7}, {normal[2]:.7}],\n')
    f.write(f'        color: {vert[1]},\n')
    f.write('    },\n')
f.write('];\n\n')

f.write('pub const ' + name.upper() + '_INDEX: &\'static [u16] = &[\n    ')
for i in range(0, len(index)):
    if i != 0 and (i % 32) == 0:
        f.write('\n    ')
    f.write(f'{index[i]}')
    if (i + 1) < len(index):
        f.write(', ')
f.write('\n];\n')

f.close()
