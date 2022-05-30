
var <name_vec> str vec push >> string/lxstring/var <name>
var <name_vec> int vec push >> lx/rv/var <name>

var <name_vec> int vec lx/rv/var <name> >> [lx/rv/var name/<number>]
var <name_vec> str vec string/lxstring/var <name> >> [lx/rv/var name/<number>]

#############################

var <name_vec> str vec >> push(string/lxstring/var name)
var <name_vec> int vec >> push(lx/rv/var <name>)

var <name_vec> int vec lx/rv/var <name>[lx/rv/var name/<number>]
var <name_vec> str vec string/lxstring/var <name>[lx/rv/var name/<number>]

New print:

print vec str/vec int
print var str <hash_str_var>
print var <hash_int_var> int
print var <hash_str_vec_var> str vec
print var <hash_int_vec_var> int vec

New vec assignments:

vec str push(string/lxstring)
vec int push(lx)
vec int push(rv)

