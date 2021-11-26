extends Spatial

"""
Naive implementation: Create all meshes from scratch.

1000 objects:
FPS: 430
Init: 2.5 sec

10000 objects:
FPS: 27
Init: 25.3 sec

Observation from 'Monitors':
- Objects drawn is N
- Draw calls is 2*N
- Surface changes is 2*N
- Mat changes is 2
- Shader changes is 2
"""

class State:
    var id: int
    var position: Vector3
    var color: Color
    
    func _init(_id: int, _position: Vector3, _color: Color):
        id = _id
        position = _position
        color = _color
        
const num_meshes = 1000

var states = []
var id_counter = 0

var state_to_node = {}


func _ready():
    for i in num_meshes:
        var new_state = State.new(
            id_counter, Vector3(randf(), randf(), randf()), Color(randf(), randf(), randf()))
        id_counter += 1
        states.append(new_state)        


func add_sphere(state: State):
    var mesh = SphereMesh.new()
    mesh.height = 0.1
    mesh.radius = 0.05
    
    var mesh_inst = MeshInstance.new()
    mesh_inst.mesh = mesh
    mesh_inst.translate(state.position)
    
    add_child(mesh_inst)
    

func diff_state_to_node():
    var any_change = false
    var t1 = OS.get_ticks_msec()
    for state in states:
        if not state.id in state_to_node:
            # print("adding", state)
            state_to_node[state.id] = state
            add_sphere(state)
            any_change = true
    var t2 = OS.get_ticks_msec()
    if any_change:
        print("Time to diff:", t2 - t1)
    


func _process(_delta):
    diff_state_to_node()
    
    
